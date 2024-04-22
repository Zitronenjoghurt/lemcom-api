use super::response_models::{Pagination, UserPublicInformation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotificationResponse {
    FriendRequest(FriendRequestNotification),
}

#[derive(Serialize, Deserialize)]
pub struct NotificationList {
    pub notifications: Vec<NotificationResponse>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct FriendRequestNotification {
    pub sender: Option<UserPublicInformation>,
    pub date: String,
}
