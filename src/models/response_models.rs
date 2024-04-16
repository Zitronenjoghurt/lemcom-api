use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

/// Private user information only accessible by yourself
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserPrivateInformation {
    /// Your username, used for identifying you
    pub name: String,
    /// The username that will be displayed to others, does not have to be the same as your regular name
    pub display_name: String,
    /// The date and time your account was created
    pub joined_date: String,
    /// The date and time you last sent an API request
    pub last_online_date: String,
    /// The total amount of API request that were processed for your account
    pub total_request_count: u64,
}

/// Public user information accessible by everyone
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserPublicInformation {
    /// The name which is used for identification
    pub name: String,
    /// The name which is displayed to others, can be a nickname
    pub display_name: String,
    /// The date and time this account was created
    pub joined_date: String,
    /// The date and time this account last sent an API request
    pub last_online_date: String,
}
