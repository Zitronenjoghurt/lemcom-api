use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{enums::PermissionLevel, user_profile::UserProfile};

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
    /// The global permission level
    pub permission_level: PermissionLevel,
    /// Your profile
    pub profile: UserProfile,
    /// Your timezone
    pub timezone: String,
}

/// Public user information accessible by everyone
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserPublicInformation {
    /// The name which is used for identification
    pub name: String,
    /// The name which is displayed to others, can be a nickname
    pub display_name: String,
    /// The date and time this account was created, null if the user set it to private
    pub joined_date: Option<String>,
    /// The date and time this account last sent an API request, null if the user set it to private
    pub last_online_date: Option<String>,
    /// The global permission level of the user
    pub permission_level: PermissionLevel,
    /// The profile of the user, not included if not stated in the query
    pub profile: Option<UserProfile>,
    /// The timezone of the user
    pub timezone: Option<String>,
}

/// Pagination information for the request results
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    /// The amount of results on the current page
    pub results: u32,
    /// The total amount of results
    pub total: u32,
    /// The current page
    pub page: u32,
    /// The amount of results per page
    pub page_size: u32,
    /// The total amount of pages
    pub pages_total: u32,
    /// The offset applied according to the current page
    pub offset: u32,
}

impl Pagination {
    pub fn new(total: u32, page: u32, page_size: u32, results: u32) -> Self {
        let offset = (page - 1) * page_size;
        let pages_total = (total + page_size - 1) / page_size;

        Pagination {
            results,
            total,
            page,
            page_size,
            pages_total,
            offset,
        }
    }
}

/// A list of users and their public information
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserList {
    /// Public user information
    pub users: Vec<UserPublicInformation>,
    pub pagination: Pagination,
}

/// A received friend request
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendRequestInformation {
    /// Public user information
    pub user: UserPublicInformation,
    /// The time the request was sent
    pub sent_date: String,
}

/// A list of received friend request
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendRequests {
    /// Received friend requests
    pub requests: Vec<FriendRequestInformation>,
    pub pagination: Pagination,
}

/// Information of a friend
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendInformation {
    /// Public user information
    pub user: UserPublicInformation,
    /// The date you became friends
    pub since_date: String,
}

/// The list of your friends
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendList {
    /// Current friends on your friendlist
    pub friends: Vec<FriendInformation>,
    pub pagination: Pagination,
}

/// An entry on your block list
#[derive(Serialize, Deserialize, ToSchema)]
pub struct BlockListEntry {
    /// The username of the blocked user
    pub name: String,
    /// The date and time you blocked this user
    pub since_date: String,
}

/// Your list of blocked users
#[derive(Serialize, Deserialize, ToSchema)]
pub struct BlockList {
    /// Entries on your block list including the name of the user and the time they were blocked
    pub entries: Vec<BlockListEntry>,
    pub pagination: Pagination,
}
