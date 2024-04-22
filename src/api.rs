#[path = "./database"]
pub mod database {
    pub mod db;
}

#[path = "./entities"]
pub mod entities {
    pub mod friendship;
    pub mod notification;
    pub mod user;
}

#[path = "./models"]
pub mod models {
    pub mod enums;
    pub mod notification_models;
    pub mod query_models;
    pub mod response_models;
    pub mod user_profile;
    pub mod user_settings;
}

#[path = "./resources"]
pub mod resources {
    pub mod friend;
    pub mod metrics;
    pub mod ping;
    pub mod timezone;
    pub mod user;
    pub mod users;
}

#[path = "./security"]
pub mod security {
    pub mod authentication;
}

#[path = "./utils"]
pub mod utils {
    pub mod macros;
    pub mod sanitize;
    pub mod serde_tz;
    pub mod time_operations;
}
