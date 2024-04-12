use mongodb::{bson::doc, Collection};
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct User {
    pub key: String,
    pub name: String,
    pub display_name: String
}

pub async fn insert_user(collection: &Collection<User>, user: User) -> mongodb::error::Result<()> {
    collection.insert_one(user, None).await?;
    println!("User added.");
    Ok(())
}

pub async fn find_user_by_key(collection: &Collection<User>, key: &str) -> mongodb::error::Result<Option<User>> {
    let filter = doc! { "key": key };
    let user = collection.find_one(Some(filter), None).await?;
    Ok(user)
}