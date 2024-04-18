use crate::api::utils::time_operations::timestamp_now_nanos;
use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    options::{InsertOneOptions, UpdateOptions},
    Collection,
};
use serde::{Deserialize, Serialize};

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
        if let Some(id) = &self.id {
            let filter = doc! { "_id": id };
            let update = doc! { "$set": bson::to_bson(self)? };
            let options = UpdateOptions::builder().upsert(true).build();
            collection.update_one(filter, update, Some(options)).await?;
        } else {
            let options = InsertOneOptions::builder().build();
            collection.insert_one(self, Some(options)).await?;
        }
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

pub async fn find_friendships_by_key(
    collection: &Collection<Friendship>,
    key: &str,
) -> mongodb::error::Result<Vec<Friendship>> {
    let filter = doc! { "keys": key};
    let cursor = collection.find(filter, None).await?;
    let friendships: Vec<Friendship> = cursor.try_collect().await?;
    Ok(friendships)
}

pub async fn remove_friendship_by_id(
    collection: &Collection<Friendship>,
    object_id: &ObjectId,
) -> Result<mongodb::results::DeleteResult, mongodb::error::Error> {
    let filter = doc! { "_id": object_id };
    collection.delete_one(filter, None).await
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
