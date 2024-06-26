use crate::api::entities::friendship::are_friends;
use crate::api::entities::user::find_user_by_name;
use crate::api::models::query_models::{
    IncludeUserProfile, PaginationQuery, UserProfileEdit, UserSettingsEdit,
};
use crate::api::models::user_settings::UserSettings;
use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation};
use crate::api::security::authentication::ExtractUser;
use crate::{unpack_result, unpack_result_option, AppState};
use axum::extract::State;
use axum::response::Response;
use axum::routing::{delete, patch, post};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_valid::Valid;

/// Retrieve own user information.
// region: get_user
/// This endpoint returns your private user information.
#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Personal private user information", body = UserPrivateInformation),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}
// endregion: get_user

/// Retrieve public user information.
// region: get_user_search
/// This endpoint returns the user information of the specified username, should they exist.
#[utoipa::path(
    get,
    path = "/user/search",
    params(UserName, IncludeUserProfile),
    responses(
        (status = 200, description = "Personal private user information", body = UserPrivateInformation),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User does not exist"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user_search(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    name_query: Query<UserName>,
    profile_query: Query<IncludeUserProfile>,
) -> Response {
    let name_query = name_query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &name_query.name).await,
        StatusCode::NOT_FOUND,
        "User not found",
        "An error occured while fetching user"
    );

    if target.block_list.contains_key(&user.key) || user.block_list.contains_key(&target.key) {
        return (StatusCode::NOT_FOUND, "User not found").into_response();
    }

    let is_friend = unpack_result!(
        are_friends(
            &state.database.friendship_collection,
            vec![user.key, target.key.clone()],
        )
        .await,
        "An error occured while trying to fetch friendship"
    );

    if !target.settings.show_in_search.is_visible(is_friend) {
        return (StatusCode::NOT_FOUND, "User not found").into_response();
    };

    Json(target.public_information(
        is_friend,
        profile_query.include_user_profile,
        &user.timezone,
    ))
    .into_response()
}
// endregion: get_user_search

/// Retrieve own user settings.
// region: get_user_settings
/// This endpoint returns your user settings.
#[utoipa::path(
    get,
    path = "/user/settings",
    responses(
        (status = 200, description = "Your user settings", body = UserSettings),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user_settings(ExtractUser(user): ExtractUser) -> Json<UserSettings> {
    Json(user.settings)
}
// endregion: get_user_search

/// Edit own user settings.
// region: patch_user_settings
/// This endpoint allows you to edit your own user settings.
#[utoipa::path(
    patch,
    path = "/user/settings",
    params(UserSettingsEdit),
    responses(
        (status = 200, description = "Your updated user settings", body = UserSettings),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn patch_user_settings(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserSettingsEdit>,
) -> Response {
    user.settings.update(query);

    unpack_result!(
        user.save(&state.database.user_collection).await,
        "Failed to save user settings"
    );
    Json(user.settings).into_response()
}
// endregion: patch_user_settings

/// Edit own user profile.
// region: patch_user_profile
/// This endpoint allows you to edit your own user profile.
#[utoipa::path(
    patch,
    path = "/user/profile",
    params(UserProfileEdit),
    responses(
        (status = 200, description = "Your updated user profile", body = UserProfile),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn patch_user_profile(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Valid<Query<UserProfileEdit>>,
) -> Response {
    let query = query.sanitize();
    user.profile.update(query);

    unpack_result!(
        user.save(&state.database.user_collection).await,
        "Failed to save user profile"
    );
    Json(user.profile).into_response()
}
// endregion: patch_user_profile

/// Retrieve users on your block list.
// region: get_user_block
/// This endpoint returns a list of usernames that are on your blocklist.
#[utoipa::path(
    get,
    path = "/user/block",
    params(PaginationQuery),
    responses(
        (status = 200, description = "Your block list", body = BlockList),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user_block(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Response {
    let query = query.sanitize();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let block_list = unpack_result!(
        user.block_list_with_pagination(&state.database.user_collection, page, page_size)
            .await,
        "An error occured while fetching users"
    );

    Json(block_list).into_response()
}
// endregion: get_user_block

/// Block a user.
// region: post_user_block
/// This endpoint allows you to block users.
#[utoipa::path(
    post,
    path = "/user/block",
    params(UserName),
    responses(
        (status = 200, description = "Successfully blocked user"),
        (status = 400, description = "Unable to block user"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn post_user_block(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found",
        "An error occured while fetching user"
    );

    if target.key == user.key {
        return (StatusCode::BAD_REQUEST, "You can't block yourself").into_response();
    };

    let is_friend = unpack_result!(
        are_friends(
            &state.database.friendship_collection,
            vec![user.key.clone(), target.key.clone()]
        )
        .await,
        "An error occured while fetching friendship"
    );

    if is_friend {
        return (
            StatusCode::BAD_REQUEST,
            "Can't block your friends, remove them first",
        )
            .into_response();
    };

    let result = user.block_user(&target.key);
    match result {
        Ok(_) => {
            unpack_result!(
                user.save(&state.database.user_collection).await,
                "An error occured while saving user"
            );
            (StatusCode::OK, "Successsfully blocked user").into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "User is already blocked").into_response(),
    }
}
// endregion: post_user_block

/// Unblock a user.
// region: delete_user_block
/// This endpoint allows you to unblock users.
#[utoipa::path(
    delete,
    path = "/user/block",
    params(UserName),
    responses(
        (status = 200, description = "Successfully unblocked user"),
        (status = 400, description = "Unable to unblock user"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn delete_user_block(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found",
        "An error occured while fetching user"
    );

    if !user.block_list.contains_key(&target.key) {
        return (StatusCode::BAD_REQUEST, "User is not on your block list").into_response();
    };

    user.block_list.remove(&target.key);
    unpack_result!(
        user.save(&state.database.user_collection).await,
        "An error occured while saving user"
    );

    (StatusCode::OK, "Successsfully unblocked user").into_response()
}
// endregion: delete_user_block

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/user", get(get_user))
        .route("/user/search", get(get_user_search))
        .route("/user/settings", get(get_user_settings))
        .route("/user/settings", patch(patch_user_settings))
        .route("/user/profile", patch(patch_user_profile))
        .route("/user/block", get(get_user_block))
        .route("/user/block", post(post_user_block))
        .route("/user/block", delete(delete_user_block))
}
