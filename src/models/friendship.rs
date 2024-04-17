use mongodb::{
    bson::{self, doc, oid::ObjectId},
    options::UpdateOptions,
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::api::utils::time_operations::timestamp_now_nanos;

#[derive(Serialize, Deserialize)]
pub struct Friendship {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub keys: Vec<String>,
    pub created_stamp: u64,
}

impl Friendship {
    pub fn new(keys: Vec<String>) -> Friendship {
        Friendship {
            id: None,
            keys,
            created_stamp: timestamp_now_nanos(),
        }
    }

    pub async fn save(&self, collection: &Collection<Friendship>) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": &self.id };
        let update = doc! { "$set": bson::to_bson(self)? };
        let options = UpdateOptions::builder().upsert(true).build();

        collection.update_one(filter, update, Some(options)).await?;
        Ok(())
    }
}

pub async fn find_friendship_by_keys(
    collection: &Collection<Friendship>,
    keys: Vec<String>,
) -> mongodb::error::Result<Option<Friendship>> {
    let filter = doc! { "keys": { "$all": keys.clone() }};
    let friendship = collection.find_one(filter, None).await?;
    Ok(friendship)
}

pub async fn are_friends(
    collection: &Collection<Friendship>,
    keys: Vec<String>,
) -> mongodb::error::Result<bool> {
    let result = find_friendship_by_keys(collection, keys).await?;
    match result {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
