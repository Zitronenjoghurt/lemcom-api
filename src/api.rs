#[path = "./database"]
pub mod database {
    pub mod db;
}

#[path = "./models"]
pub mod models {
    pub mod query_models;
    pub mod response_models;
    pub mod user;
    pub mod user_settings;
}

#[path = "./resources"]
pub mod resources {
    pub mod ping;
    pub mod user;
    pub mod users;
}

#[path = "./security"]
pub mod security {
    pub mod authentication;
}

#[path = "./utils"]
pub mod utils {
    pub mod sanitize;
    pub mod time_operations;
}
