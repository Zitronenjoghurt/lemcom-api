use std::collections::HashMap;
use mongodb::{bson::{self, doc}, options::UpdateOptions, Collection};
use serde::{ Deserialize, Serialize };

use crate::api::utils::time_operations::timestamp_now_micro;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub created_stamp: u64,
    pub last_access_stamp: u64,
    #[serde(default)]
    pub endpoint_usage: HashMap<String, u64>
}

impl User {
    pub async fn save(&self, collection: &Collection<User>) -> mongodb::error::Result<()> {
        let filter = doc! { "key": &self.key };
        let update = doc! { "$set": bson::to_bson(self)? };
        let options = UpdateOptions::builder().upsert(true).build();

        collection.update_one(filter, update, Some(options)).await?;
        println!("User saved.");
        Ok(())
    }

    pub fn use_endpoint(&mut self, endpoint_name: &str) {
        self.last_access_stamp = timestamp_now_micro();
        *self.endpoint_usage.entry(endpoint_name.to_string()).or_insert(0) += 1;
    }
}

pub async fn find_user_by_key(collection: &Collection<User>, key: &str) -> mongodb::error::Result<Option<User>> {
    let filter = doc! { "key": key };
    let user = collection.find_one(Some(filter), None).await?;
    Ok(user)
}