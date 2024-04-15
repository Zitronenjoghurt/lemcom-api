use crate::api::utils::sanitize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserName {
    pub name: String,
}

impl UserName {
    pub fn sanitize(&self) -> UserName {
        return UserName {
            name: sanitize::alphanumeric(self.name.clone()),
        };
    }
}
