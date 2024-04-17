use crate::api::models::query_models::PaginationQuery;
use crate::api::security::authentication::ExtractUser;
use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};

/// Retrieve pending friend requests.
///
/// This endpoint returns a list of users that have sent you friend requests.
#[utoipa::path(
    get,
    path = "/friend/request",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Users you have pending friend requests from", body = FriendRequests),
        (status = 400, description = "Missing API Key or Invalid header/query"),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn get_friend_request(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Response {
    let query = query.sanitize();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let result = user
        .friend_requests_with_pagination(&state.database.user_collection, page, page_size)
        .await;

    match result {
        Ok(requests) => Json(requests).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ("An error occured while fetching your friend requests"),
        )
            .into_response(),
    }
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/friend/request", get(get_friend_request))
}
