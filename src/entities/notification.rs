use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    options::{InsertOneOptions, UpdateOptions},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::api::utils::time_operations::timestamp_now_nanos;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Notification {
    FriendRequestReceived(FriendRequestReceived),
}

impl Notification {
    pub async fn save(&self, collection: &Collection<Notification>) -> mongodb::error::Result<()> {
        let mut doc = bson::to_document(self)?;
        let id = doc.get("_id").and_then(Bson::as_object_id);

        match id {
            Some(id) => {
                let filter = doc! { "_id": id };
                doc.remove("_id");
                let update = doc! { "$set": doc };
                let options = UpdateOptions::builder().upsert(true).build();
                collection.update_one(filter, update, Some(options)).await?;
            }
            None => {
                let options = InsertOneOptions::builder().build();
                collection.insert_one(self, Some(options)).await?;
            }
        }
        Ok(())
    }

    pub async fn friend_request(
        collection: &Collection<Notification>,
        sender_key: &str,
        receiver_key: &str,
    ) -> mongodb::error::Result<()> {
        Notification::FriendRequestReceived(FriendRequestReceived::new(sender_key, receiver_key))
            .save(collection)
            .await
    }
}

pub async fn find_notifications_by_receiver_key(
    collection: &Collection<Notification>,
    key: &str,
) -> mongodb::error::Result<Vec<Notification>> {
    let filter = doc! { "common.receiver_key": key};
    let cursor = collection.find(filter, None).await?;
    let notifications: Vec<Notification> = cursor.try_collect().await?;
    Ok(notifications)
}

#[derive(Serialize, Deserialize)]
pub struct CommonFields {
    pub created_at: u64,
    pub receiver_key: String,
}

impl CommonFields {
    pub fn new(receiver_key: &str) -> CommonFields {
        CommonFields {
            created_at: timestamp_now_nanos(),
            receiver_key: receiver_key.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FriendRequestReceived {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub common: CommonFields,
    pub sender_key: String,
}

impl FriendRequestReceived {
    pub fn new(sender_key: &str, receiver_key: &str) -> FriendRequestReceived {
        FriendRequestReceived {
            id: None,
            common: CommonFields::new(receiver_key),
            sender_key: sender_key.to_string(),
        }
    }
}
