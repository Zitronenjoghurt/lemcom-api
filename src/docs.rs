use utoipa::{openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}, Modify, OpenApi};
use crate::api::{self, models::{enums::{PermissionLevel, PrivacyLevel}, notification_models::{FriendRequestNotification, NotificationList, NotificationResponse, TestNotification}, response_models::{BlockList, BlockListEntry, CountResponse, FriendInformation, FriendList, FriendRequestInformation, FriendRequests, MessageResponse, Pagination, UserList, UserPrivateInformation, UserPublicInformation}, user_profile::UserProfile, user_settings::UserSettings}};

#[derive(OpenApi)]
#[openapi(
    info(
        title="LemCom API",
        description="A webservice for handling LemCom online services. LemCom will be a messaging application for desktop written in Rust.\n\nAll available docs: Rapidoc (/docs), Swagger (/swagger) and Redoc (/redoc).\n\nIf you find bugs or have feedback please create an issue here: https://github.com/Zitronenjoghurt/lemcom-api/issues"
    ),
    paths(
        api::resources::friend::get_friend,
        api::resources::friend::delete_friend,
        api::resources::friend::delete_friend_request,
        api::resources::friend::get_friend_request,
        api::resources::friend::post_friend_request,
        api::resources::friend::post_friend_request_accept,
        api::resources::friend::post_friend_request_deny,
        api::resources::metrics::get_metrics_usage,
        api::resources::notification::get_notification,
        api::resources::notification::delete_notification,
        api::resources::ping::get_ping,  
        api::resources::timezone::get_timezone,
        api::resources::timezone::put_timezone,
        api::resources::user::get_user,
        api::resources::user::post_user_block,
        api::resources::user::get_user_block,
        api::resources::user::delete_user_block,
        api::resources::user::get_user_search,
        api::resources::user::get_user_settings,
        api::resources::user::patch_user_profile,
        api::resources::user::patch_user_settings,
        api::resources::users::get_users
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints"),
        (name = "Notification", description = "Endpoints for handling your notifications"),
        (name = "User", description = "User management endpoints"),
        (name = "Users", description = "Endpoint for handling multiple users"),
        (name = "Friends", description = "Endpoints for handling friend requests and friendships"),
    ),
    modifiers(&SecurityAddon),
    components(
        schemas(MessageResponse, UserPublicInformation, UserPrivateInformation, UserSettings, UserList, Pagination, PrivacyLevel, PermissionLevel, FriendRequestInformation, FriendRequests, FriendInformation, FriendList, UserProfile, BlockList, BlockListEntry, NotificationList, NotificationResponse, FriendRequestNotification, TestNotification, CountResponse),
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            )
        }
    }
}