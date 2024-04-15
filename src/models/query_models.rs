use crate::api::utils::sanitize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserName {
    pub name: String,
}

impl UserName {
    pub fn sanitize(&self) -> UserName {
        UserName {
            name: sanitize::alphanumeric(&self.name),
        }
    }
}

#[derive(Deserialize)]
pub struct UserSettingsEdit {
    pub show_join_date: Option<bool>,
    pub show_online: Option<bool>,
}
