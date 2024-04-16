use crate::api::models::query_models::UserSettingsEdit;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User configuration
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserSettings {
    /// If other people are able to find you without knowing your name, for example on a public user list
    #[serde(default = "default_false")]
    pub profile_public: bool,
    /// If other people are able to see when you joined the network
    #[serde(default = "default_true")]
    pub join_date_public: bool,
    /// If other people are able to see when you were last online
    #[serde(default = "default_true")]
    pub online_date_public: bool,
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

impl UserSettings {
    pub fn update(&mut self, data: Query<UserSettingsEdit>) {
        self.profile_public = data.profile_public.unwrap_or(self.profile_public);
        self.join_date_public = data.show_join_date.unwrap_or(self.join_date_public);
        self.online_date_public = data.show_online.unwrap_or(self.online_date_public);
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            profile_public: false,
            join_date_public: true,
            online_date_public: true,
        }
    }
}
