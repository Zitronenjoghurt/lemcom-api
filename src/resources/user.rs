use crate::api::entities::friendship::are_friends;
use crate::api::entities::user::find_user_by_name;
use crate::api::models::query_models::{IncludeUserProfile, UserProfileEdit, UserSettingsEdit};
use crate::api::models::user_settings::UserSettings;
use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation};
use crate::api::security::authentication::ExtractUser;
use crate::{unpack_result, unpack_result_option, AppState};
use axum::extract::State;
use axum::response::Response;
use axum::routing::patch;
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

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/user", get(get_user))
        .route("/user/search", get(get_user_search))
        .route("/user/settings", get(get_user_settings))
        .route("/user/settings", patch(patch_user_settings))
        .route("/user/profile", patch(patch_user_profile))
}
