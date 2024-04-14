use serde::Deserialize;
use crate::api::utils::sanitize;

#[derive(Deserialize)]
pub struct UserName {
    pub name: String
}

impl UserName {
    pub fn sanitize(&self) -> UserName {
        return UserName {
            name: sanitize::alphanumeric(self.name.clone())
        }
    }
}