use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Copy, Clone, ToSchema)]
pub enum PrivacyLevel {
    Public,
    Friends,
    Private,
}

impl PrivacyLevel {
    pub fn is_visible(&self, is_friend: bool) -> bool {
        match self {
            PrivacyLevel::Public => true,
            PrivacyLevel::Friends => is_friend,
            PrivacyLevel::Private => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Default)]
pub enum PermissionLevel {
    #[default]
    User,
    Moderator,
    Administrator,
    Owner,
}
