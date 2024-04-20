use crate::api::models::enums::PrivacyLevel;
use crate::api::utils::sanitize;
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UserName {
    /// The username, does not have to be case-sensitive
    pub name: String,
}

impl UserName {
    pub fn sanitize(&self) -> UserName {
        UserName {
            name: sanitize::alphanumeric(&self.name),
        }
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IncludeUserProfile {
    /// If user profile should be included in the user information
    #[serde(default = "default_false")]
    pub include_user_profile: bool,
}

fn default_false() -> bool {
    false
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationQuery {
    /// The results page number
    pub page: Option<u32>,
    /// The maximum amount of results per page, has to be between 1 and 100
    pub page_size: Option<u32>,
}

impl PaginationQuery {
    pub fn sanitize(&self) -> PaginationQuery {
        let clamped_page_size = self.page_size.map(|size| size.clamp(1, 100));

        PaginationQuery {
            page: self.page,
            page_size: clamped_page_size,
        }
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UserSettingsEdit {
    /// If other people should be able to find you on the public user lists
    pub appear_on_public_list: Option<bool>,
    /// If other people should be able to see when you joined the network
    pub show_join_date: Option<PrivacyLevel>,
    /// If other people should be able to see when you were last online
    pub show_online: Option<PrivacyLevel>,
    /// If other people should be able to find you in search using your username
    pub show_in_search: Option<PrivacyLevel>,
    /// If other people should be able to send you friend requests using your username
    pub allow_friend_requests: Option<bool>,
    /// If other people should be able to see your timezone
    pub show_timezone: Option<PrivacyLevel>,
}

#[derive(Deserialize, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct UserProfileEdit {
    /// MAX LENGTH: 64 | Your preferred pronouns
    #[validate(length(min = 1, max = 64))]
    pub pronouns: Option<String>,
    /// MAX LENGTH: 4096 | Your bio text, it's a custom long text description
    #[validate(length(min = 1, max = 4096))]
    pub bio: Option<String>,
    /// MAX LENGTH: 128 | Your current status, a short text describing your current situation, like being at work or on vacation
    #[validate(length(min = 1, max = 128))]
    pub status: Option<String>,
    /// MAX LENGTH: 32 | One word describing your current mood
    #[validate(length(min = 1, max = 32))]
    pub mood: Option<String>,
}

impl UserProfileEdit {
    pub fn sanitize(&self) -> UserProfileEdit {
        UserProfileEdit {
            pronouns: self
                .pronouns
                .as_ref()
                .map(|pronouns| sanitize::profanity(&sanitize::limit_string(pronouns, 64))),
            bio: self
                .bio
                .as_ref()
                .map(|bio| sanitize::profanity(&sanitize::limit_string(bio, 4096))),
            status: self
                .status
                .as_ref()
                .map(|status| sanitize::profanity(&sanitize::limit_string(status, 128))),
            mood: self
                .mood
                .as_ref()
                .map(|mood| sanitize::profanity(&sanitize::limit_string(mood, 32))),
        }
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TimezoneQuery {
    /// The timezone you want to use, not to be case-sensitive. Look up available timezones at GET /timezone.
    pub timezone: String,
}
