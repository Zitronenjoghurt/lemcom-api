use crate::api::models::response_models::UserPublicInformation;
use crate::api::models::user::get_public_users;
use crate::api::models::{query_models::PaginationQuery, response_models::UserList};
use crate::api::security::authentication::ExtractUser;
use crate::{unpack_result, AppState};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};

/// Retrieve public users.
///
/// This endpoint returns a list of users which are publicly visible.
/// To be publicly visible, users have to set profile_public to true.
#[utoipa::path(
    get,
    path = "/users",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Publicly visible users", body = UserList),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Users"
)]
async fn get_users(
    ExtractUser(_): ExtractUser,
    State(state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Response {
    let query = query.sanitize();

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (users, pagination) = unpack_result!(
        get_public_users(&state.database.user_collection, page, page_size).await,
        "An error occured while fetching users"
    );

    let public_information: Vec<UserPublicInformation> = users
        .iter()
        .map(|user| user.public_information(false)) // ToDo: is_friend set to false, implement when friend feature is added
        .collect();
    let user_list = UserList {
        users: public_information,
        pagination,
    };
    Json(user_list).into_response()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/users", get(get_users))
}
