use mongodb::{
    error::Result, options::ClientOptions, Client, Collection
};

use crate::api::models::user::User;

pub struct DB {
    pub client: Client,
    pub user_collection: Collection<User>
}

pub async fn setup() -> Result<DB> {
    let mongo_url = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(mongo_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("your_database_name");

    Ok(DB {
        client,
        user_collection: db.collection::<User>("users"),
    })
}