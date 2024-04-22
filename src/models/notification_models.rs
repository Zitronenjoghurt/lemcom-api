use super::response_models::UserPublicInformation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotificationResponse {
    FriendRequest(FriendRequestNotification),
}

#[derive(Serialize, Deserialize)]
pub struct FriendRequestNotification {
    pub sender: Option<UserPublicInformation>,
    pub date: String,
}
