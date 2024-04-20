use crate::api::models::enums::PrivacyLevel;
use crate::api::models::query_models::UserSettingsEdit;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User configuration
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserSettings {
    /// If people can see you on the public user list
    #[serde(default = "default_false")]
    pub appear_on_public_list: bool,
    /// Who is able to see when you joined the network
    #[serde(default = "default_public")]
    pub show_join_date: PrivacyLevel,
    /// Who is able to see when you were last online
    #[serde(default = "default_public")]
    pub show_online_date: PrivacyLevel,
    /// Who is able to find you using the search functionality
    #[serde(default = "default_public")]
    pub show_in_search: PrivacyLevel,
    /// If people can send you friend requests when they know your username
    #[serde(default = "default_true")]
    pub allow_friend_requests: bool,
    /// If people can see your timezone
    #[serde(default = "default_private")]
    pub show_timezone: PrivacyLevel,
}

fn default_private() -> PrivacyLevel {
    PrivacyLevel::Private
}

fn default_public() -> PrivacyLevel {
    PrivacyLevel::Public
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl UserSettings {
    pub fn update(&mut self, data: Query<UserSettingsEdit>) {
        if let Some(new_value) = &data.appear_on_public_list {
            self.appear_on_public_list = *new_value;
        }
        if let Some(new_value) = &data.show_join_date {
            self.show_join_date = *new_value;
        }
        if let Some(new_value) = &data.show_online {
            self.show_online_date = *new_value;
        }
        if let Some(new_value) = &data.show_in_search {
            self.show_in_search = *new_value;
        }
        if let Some(new_value) = &data.allow_friend_requests {
            self.allow_friend_requests = *new_value;
        }
        if let Some(new_value) = &data.show_timezone {
            self.show_timezone = *new_value;
        }
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            appear_on_public_list: false,
            show_join_date: PrivacyLevel::Public,
            show_online_date: PrivacyLevel::Public,
            show_in_search: PrivacyLevel::Public,
            allow_friend_requests: true,
            show_timezone: PrivacyLevel::Private,
        }
    }
}
