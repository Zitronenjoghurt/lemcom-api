use crate::api::entities::friendship::are_friends;
use crate::api::entities::user::get_public_users;
use crate::api::models::query_models::IncludeUserProfile;
use crate::api::models::response_models::UserPublicInformation;
use crate::api::models::{query_models::PaginationQuery, response_models::UserList};
use crate::api::security::authentication::ExtractUser;
use crate::{unpack_result, AppState};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};

/// Retrieve public users.
// region: get_users
/// This endpoint returns a list of users which are publicly visible.
/// To be publicly visible, users have to set profile_public to true.
#[utoipa::path(
    get,
    path = "/users",
    params(PaginationQuery, IncludeUserProfile),
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
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    pagination: Query<PaginationQuery>,
    profile_query: Query<IncludeUserProfile>,
) -> Response {
    let pagination = pagination.sanitize();

    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let (users, pagination) = unpack_result!(
        get_public_users(&state.database.user_collection, page, page_size).await,
        "An error occured while fetching users"
    );

    let mut public_information: Vec<UserPublicInformation> = Vec::new();
    for target in users.iter() {
        let is_friend = unpack_result!(
            are_friends(
                &state.database.friendship_collection,
                vec![user.key.clone(), target.key.clone()]
            )
            .await,
            "An error occured while fetching friendship"
        );
        public_information.push(target.public_information(
            is_friend,
            profile_query.include_user_profile,
            &user.timezone,
        ));
    }

    let user_list = UserList {
        users: public_information,
        pagination,
    };
    Json(user_list).into_response()
}
// endregion: get_users

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/users", get(get_users))
}
