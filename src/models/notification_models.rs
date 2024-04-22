use super::response_models::{Pagination, UserPublicInformation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Different types of notifications
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum NotificationResponse {
    TestNotification(TestNotification),
    FriendRequest(FriendRequestNotification),
}

/// A list of notifications
#[derive(Serialize, Deserialize, ToSchema)]
pub struct NotificationList {
    /// All your current notifications
    pub notifications: Vec<NotificationResponse>,
    pub pagination: Pagination,
}

/// A notification about a received friend request
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendRequestNotification {
    /// The user that sent you this friend request
    pub sender: Option<UserPublicInformation>,
    /// The date you received this notification
    pub date: String,
}

/// A simple test notification
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TestNotification {
    /// The date you received this notification
    pub date: String,
}
