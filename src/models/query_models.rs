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
    /// If other people should be able to see when you joined the network
    pub show_join_date: Option<bool>,
    /// If other people should be able to see when you were last online
    pub show_online: Option<bool>,
}
