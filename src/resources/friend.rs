use crate::api::entities::friendship::{
    are_friends, find_friendship_by_keys, remove_friendship_by_id, Friendship,
};
use crate::api::entities::user::find_user_by_name;
use crate::api::models::query_models::{IncludeUserProfile, PaginationQuery, UserName};
use crate::api::security::authentication::ExtractUser;
use crate::api::utils::time_operations::timestamp_now_nanos;
use crate::{unpack_result, unpack_result_option, AppState};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use axum::{routing::get, Json, Router};

/// Retrieve your current friends.
// region: get_friend
/// This endpoint returns a list of users that have sent you friend requests.
#[utoipa::path(
    get,
    path = "/friend",
    params(PaginationQuery, IncludeUserProfile),
    responses(
        (status = 200, description = "Your friends", body = FriendList),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn get_friend(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    pagination: Query<PaginationQuery>,
    profile_query: Query<IncludeUserProfile>,
) -> Response {
    let pagination = pagination.sanitize();
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let friend_list = unpack_result!(
        user.friend_list_with_pagination(
            &state.database.user_collection,
            &state.database.friendship_collection,
            page,
            page_size,
            profile_query.include_user_profile
        )
        .await,
        "An error occured while fetching your friendships"
    );

    Json(friend_list).into_response()
}
// endregion: get_friend

/// Remove a friend.
// region: delete_friend
/// This endpoint allows the user to remove a friend..
#[utoipa::path(
    delete,
    path = "/friend",
    params(UserName),
    responses(
        (status = 200, description = "Friend successfully removed"),
        (status = 400, description = "Unable to remove friend"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn delete_friend(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found",
        "An error occurred while fetching user"
    );

    let friendship = unpack_result_option!(
        find_friendship_by_keys(
            &state.database.friendship_collection,
            vec![target.key, user.key],
        )
        .await,
        StatusCode::BAD_REQUEST,
        "Not friend with the user",
        "An error occured while fetching friendship"
    );

    unpack_result!(
        remove_friendship_by_id(
            &state.database.friendship_collection,
            &friendship.id.unwrap(),
        )
        .await,
        "An error occured while removing friendship"
    );

    Json((StatusCode::OK, "Friend successfully removed")).into_response()
}
// endregion: delete_friend

/// Retrieve pending friend requests.
// region: get_friend_request
/// This endpoint returns a list of users that have sent you friend requests.
#[utoipa::path(
    get,
    path = "/friend/request",
    params(PaginationQuery, IncludeUserProfile),
    responses(
        (status = 200, description = "Users you have pending friend requests from", body = FriendRequests),
        (status = 401, description = "Invalid API Key"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn get_friend_request(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    pagination: Query<PaginationQuery>,
    profile_query: Query<IncludeUserProfile>,
) -> Response {
    let pagination = pagination.sanitize();
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let requests = unpack_result!(
        user.friend_requests_with_pagination(
            &state.database.user_collection,
            page,
            page_size,
            profile_query.include_user_profile
        )
        .await,
        "An error occured while fetching your friend requests"
    );
    Json(requests).into_response()
}
// endregion: get_friend_request

/// Send friend requests.
// region: post_friend_request
/// This endpoint allows you to send a friend request to users.
#[utoipa::path(
    post,
    path = "/friend/request",
    params(UserName),
    responses(
        (status = 200, description = "Friend request was sent"),
        (status = 400, description = "Unable to send request"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found or user does not allow friend requests"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn post_friend_request(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let mut target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found or user does not allow friend requests",
        "An error occurred while fetching user"
    );

    if target.key == user.key {
        return Json((
            StatusCode::BAD_REQUEST,
            "Can't send a friend request to yourself",
        ))
        .into_response();
    }

    if !target.settings.allow_friend_requests {
        return Json((
            StatusCode::NOT_FOUND,
            "User not found or user does not allow friend requests",
        ))
        .into_response();
    }

    let already_friends = unpack_result!(
        are_friends(
            &state.database.friendship_collection,
            vec![user.key.clone(), target.key.clone()],
        )
        .await,
        "An error occurred while fetching friendship"
    );

    if already_friends {
        return Json((
            StatusCode::BAD_REQUEST,
            "You are already friends with the user",
        ))
        .into_response();
    }

    if target.friend_requests.contains_key(&user.key) {
        return Json((
            StatusCode::BAD_REQUEST,
            "Already sent a request to the user",
        ))
        .into_response();
    }

    target
        .friend_requests
        .insert(user.key, timestamp_now_nanos());

    unpack_result!(
        target.save(&state.database.user_collection).await,
        "An error occured while saving the target user"
    );

    Json((StatusCode::OK, "Friend request sent")).into_response()
}
// endregion: post_friend_request

/// Retract friend requests.
// region: delete_friend_request
/// This endpoint allows you to retract a friend request.
#[utoipa::path(
    delete,
    path = "/friend/request",
    params(UserName),
    responses(
        (status = 200, description = "Friend request was retracted"),
        (status = 400, description = "You did not send a request to the user"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn delete_friend_request(
    ExtractUser(user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let mut target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found",
        "An error occurred while fetching user"
    );

    if !target.friend_requests.contains_key(&user.key) {
        return Json((
            StatusCode::BAD_REQUEST,
            "You did not send a request to the user",
        ))
        .into_response();
    }

    target.friend_requests.remove(&user.key);

    unpack_result!(
        target.save(&state.database.user_collection).await,
        "An error occured while saving the target user"
    );

    Json((StatusCode::OK, "Friend request retracted")).into_response()
}
// endregion: delete_friend_request

/// Accept a pending friend request.
// region: post_friend_request_accept
/// This endpoint allows you to accept friend requests.
#[utoipa::path(
    post,
    path = "/friend/request/accept",
    params(UserName),
    responses(
        (status = 200, description = "Friend request accepted"),
        (status = 401, description = "Unable to accept request"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found or no pending request from user"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn post_friend_request_accept(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found or no pending request from user",
        "An error occurred while fetching user"
    );

    if !user.friend_requests.contains_key(&target.key) {
        return (
            StatusCode::NOT_FOUND,
            "User not found or no pending request from user",
        )
            .into_response();
    };

    user.friend_requests.remove(&target.key);
    unpack_result!(
        user.save(&state.database.user_collection).await,
        "An error occured while saving the user"
    );

    let already_friends = unpack_result!(
        are_friends(
            &state.database.friendship_collection,
            vec![user.key.clone(), target.key.clone()],
        )
        .await,
        "An error occured while trying to fetch friendship"
    );

    if already_friends {
        return (
            StatusCode::BAD_REQUEST,
            "You are already friends with the user",
        )
            .into_response();
    }

    let new_friendship = Friendship::new(vec![user.key, target.key]);
    unpack_result!(
        new_friendship
            .save(&state.database.friendship_collection)
            .await,
        "An error occured while saving the friendship"
    );

    (StatusCode::OK, "Friend request accepted").into_response()
}
// endregion: post_friend_request_accept

/// Deny a pending friend request.
// region: post_friend_request_deny
/// This endpoint allows you to deny friend requests.
#[utoipa::path(
    post,
    path = "/friend/request/deny",
    params(UserName),
    responses(
        (status = 200, description = "Friend request denied"),
        (status = 401, description = "Unable to deny request"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found or no pending request from user"),
        (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Friends"
)]
async fn post_friend_request_deny(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();

    let target = unpack_result_option!(
        find_user_by_name(&state.database.user_collection, &query.name).await,
        StatusCode::NOT_FOUND,
        "User not found or no pending request from user",
        "An error occurred while fetching user"
    );

    if !user.friend_requests.contains_key(&target.key) {
        return (
            StatusCode::NOT_FOUND,
            "User not found or no pending request from user",
        )
            .into_response();
    };

    user.friend_requests.remove(&target.key);
    unpack_result!(
        user.save(&state.database.user_collection).await,
        "An error occured while saving the user"
    );

    (StatusCode::OK, "Friend request denied").into_response()
}
// endregion: post_friend_request_deny

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/friend", get(get_friend))
        .route("/friend", delete(delete_friend))
        .route("/friend/request", get(get_friend_request))
        .route("/friend/request", post(post_friend_request))
        .route("/friend/request", delete(delete_friend_request))
        .route("/friend/request/accept", post(post_friend_request_accept))
        .route("/friend/request/deny", post(post_friend_request_deny))
}
