use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserPrivateInformation {
    pub name: String,
    pub display_name: String,
    pub joined_date: String,
    pub last_online_date: String,
    pub total_request_count: u64
}

#[derive(Serialize, Deserialize)]
pub struct UserPublicInformation {
    pub name: String,
    pub display_name: String,
    pub joined_date: String,
    pub last_online_date: String
}