use crate::api::models::enums::PrivacyLevel;
use crate::api::utils::sanitize;
use serde::Deserialize;
use utoipa::IntoParams;

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
pub struct UserSettingsEdit {
    /// If other people should be able to find you without knowing your name, for example through a public user list
    pub show_public: Option<PrivacyLevel>,
    /// If other people should be able to see when you joined the network
    pub show_join_date: Option<PrivacyLevel>,
    /// If other people should be able to see when you were last online
    pub show_online: Option<PrivacyLevel>,
    /// If other people should be able to find you in search using your username
    pub show_in_search: Option<PrivacyLevel>,
    /// If other people should be able to send you friend requests using your username
    pub allow_friend_requests: Option<bool>,
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
