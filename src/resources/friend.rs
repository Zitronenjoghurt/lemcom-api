use crate::api::models::friendship::are_friends;
use crate::api::models::query_models::{PaginationQuery, UserName};
use crate::api::models::user::find_user_by_name;
use crate::api::security::authentication::ExtractUser;
use crate::api::utils::time_operations::timestamp_now_nanos;
use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
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

/// Send friend requests.
///
/// This endpoint allows you to send a friend request to users.
#[utoipa::path(
    post,
    path = "/friend/request",
    params(UserName),
    responses(
        (status = 200, description = "Friend request was sent"),
        (status = 400, description = "Already friends with the user"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User not found or user does not allow friend requests"),
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

    let mut target = match find_user_by_name(&state.database.user_collection, &query.name).await {
        Ok(Some(target)) => {
            if target.key == user.key {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("Can't send a friend request to yourself".into())
                    .unwrap();
            }
            target
        }
        Ok(None) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("User not found or user does not allow friend requests".into())
                .unwrap();
        }
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("An error occurred while fetching user".into())
                .unwrap();
        }
    };

    if !target.settings.allow_friend_requests {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("User not found or user does not allow friend requests".into())
            .unwrap();
    }

    let already_friends = match are_friends(
        &state.database.friendship_collection,
        vec![user.key.clone(), target.key.clone()],
    )
    .await
    {
        Ok(already_friends) => already_friends,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("An error occurred while fetching friendship".into())
                .unwrap();
        }
    };

    if already_friends {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("You are already friends with the user".into())
            .unwrap();
    }

    target
        .friend_requests
        .insert(user.key, timestamp_now_nanos());
    match target.save(&state.database.user_collection).await {
        Ok(_) => (StatusCode::OK, "Friend request sent").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error occured while saving the target user",
        )
            .into_response(),
    }
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/friend/request", get(get_friend_request))
        .route("/friend/request", post(post_friend_request))
}
