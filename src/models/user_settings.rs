use crate::api::models::query_models::UserSettingsEdit;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::enums::PrivacyLevel;

/// User configuration
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserSettings {
    /// Who is able to see you, public means you may appear on public user lists
    #[serde(default = "default_friends")]
    pub show_profile: PrivacyLevel,
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
}

fn default_friends() -> PrivacyLevel {
    PrivacyLevel::Friends
}

fn default_public() -> PrivacyLevel {
    PrivacyLevel::Public
}

fn default_true() -> bool {
    true
}

impl UserSettings {
    pub fn update(&mut self, data: Query<UserSettingsEdit>) {
        if let Some(new_value) = &data.show_public {
            self.show_profile = new_value.clone();
        }
        if let Some(new_value) = &data.show_join_date {
            self.show_join_date = new_value.clone();
        }
        if let Some(new_value) = &data.show_online {
            self.show_online_date = new_value.clone();
        }
        if let Some(new_value) = &data.show_in_search {
            self.show_in_search = new_value.clone();
        }
        if let Some(new_value) = &data.allow_friend_requests {
            self.allow_friend_requests = *new_value;
        }
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            show_profile: PrivacyLevel::Friends,
            show_join_date: PrivacyLevel::Public,
            show_online_date: PrivacyLevel::Public,
            show_in_search: PrivacyLevel::Public,
            allow_friend_requests: true,
        }
    }
}
