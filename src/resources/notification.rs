use crate::api::entities::notification::{
    clear_notifications_by_key, find_notifications_by_receiver_key,
};
use crate::api::models::notification_models::NotificationList;
use crate::api::models::query_models::PaginationQuery;
use crate::api::models::response_models::{CountResponse, Pagination};
use crate::api::security::authentication::ExtractUser;
use crate::{unpack_result, AppState};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::delete;
use axum::{routing::get, Json, Router};
use futures::{stream, StreamExt, TryStreamExt};

/// Retrieve your notifications.
// region: get_notification
/// This endpoint returns a list of your notifications.
#[utoipa::path(
    get,
    path = "/notification",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Your list of notifications", body = NotificationList),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Notification"
)]
async fn get_notification(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Response {
    let query = query.sanitize();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let notifications = unpack_result!(
        find_notifications_by_receiver_key(&state.database.notification_collection, &user.key)
            .await,
        "An error occured while fetching notifications"
    );

    let start = ((page - 1) * page_size) as usize;
    let entry_count = notifications.len();

    if start >= entry_count {
        return Json(NotificationList {
            notifications: vec![],
            pagination: Pagination::new(entry_count as u32, page, page_size, 0),
        })
        .into_response();
    }
    let end = std::cmp::min(start + page_size as usize, entry_count);

    let responses = stream::iter(notifications[start..end].iter())
        .then(|notification| notification.get_response(&user, &state.database))
        .try_collect::<Vec<_>>()
        .await
        .unwrap_or_else(|_| Vec::new());

    let pagination = Pagination::new(entry_count as u32, page, page_size, responses.len() as u32);

    Json(NotificationList {
        notifications: responses,
        pagination,
    })
    .into_response()
}
// endregion: get_notification

/// Clear your notifications.
// region: delete_notification
/// This endpoint allows you to clear all of your notifications.
#[utoipa::path(
    delete,
    path = "/notification",
    responses(
        (status = 200, description = "Notifications cleared", body = CountResponse),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Notification"
)]
async fn delete_notification(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
) -> Response {
    let count = unpack_result!(
        clear_notifications_by_key(&state.database.notification_collection, &user.key).await,
        "An error occured while clearing notifications"
    );

    Json(CountResponse {
        message: format!("Cleared {} notification(s)", count),
        count,
    })
    .into_response()
}
// endregion: delete_notification

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/notification", get(get_notification))
        .route("/notification", delete(delete_notification))
}
