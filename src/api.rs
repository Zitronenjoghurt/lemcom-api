#[path ="./database"]
pub mod database {
    pub mod db;
}

#[path ="./models"]
pub mod models {
    pub mod base_models;
    pub mod user;
}

#[path ="./resources"]
pub mod resources {
    pub mod ping;
}

#[path ="./security"]
pub mod security {
    pub mod authentication;
}

#[path ="./utils"]
pub mod utils {
    pub mod route_capture;
    pub mod time_operations;
}