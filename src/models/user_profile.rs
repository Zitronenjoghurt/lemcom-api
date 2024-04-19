use crate::api::models::query_models::UserProfileEdit;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User profile
#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
pub struct UserProfile {
    /// The preferred pronouns of the user
    #[serde(default)]
    pub pronouns: String,
    /// The bio text of the user, it's a custom long text description
    #[serde(default)]
    pub bio: String,
    /// The current status of the user, a short text describing the current situation of the user, like being at work or on vacation
    #[serde(default)]
    pub status: String,
    /// One word describing the users current mood
    #[serde(default)]
    pub mood: String,
}

impl UserProfile {
    pub fn update(&mut self, data: UserProfileEdit) {
        if let Some(new_value) = &data.pronouns {
            self.pronouns = new_value.clone();
        }
        if let Some(new_value) = &data.bio {
            self.bio = new_value.clone();
        }
        if let Some(new_value) = &data.status {
            self.status = new_value.clone();
        }
        if let Some(new_value) = &data.mood {
            self.mood = new_value.clone();
        }
    }
}
