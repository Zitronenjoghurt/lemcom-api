use mongodb::{
    bson::{self, doc},
    options::UpdateOptions,
    Collection,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::models::response_models::{UserPrivateInformation, UserPublicInformation};
use crate::api::utils::time_operations::{nanos_to_date, timestamp_now_nanos};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub created_stamp: u64,
    pub last_access_stamp: u64,
    #[serde(default)]
    pub endpoint_usage: HashMap<String, u64>,
    #[serde(default = "default_true")]
    pub join_date_public: bool,
    #[serde(default = "default_true")]
    pub online_date_public: bool,
}

fn default_true() -> bool {
    true
}

impl User {
    pub async fn save(&self, collection: &Collection<User>) -> mongodb::error::Result<()> {
        let filter = doc! { "key": &self.key };
        let update = doc! { "$set": bson::to_bson(self)? };
        let options = UpdateOptions::builder().upsert(true).build();

        collection.update_one(filter, update, Some(options)).await?;
        Ok(())
    }

    pub fn use_endpoint(&mut self, method: &str, path: &str) {
        self.last_access_stamp = timestamp_now_nanos();
        *self
            .endpoint_usage
            .entry(format!("{method} {path}"))
            .or_insert(0) += 1;
    }

    pub fn request_count(&self) -> u64 {
        let mut sum: u64 = 0;
        for (_, value) in self.endpoint_usage.iter() {
            sum += value;
        }
        sum
    }

    pub fn private_information(&self) -> UserPrivateInformation {
        UserPrivateInformation {
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            joined_date: nanos_to_date(self.created_stamp),
            last_online_date: nanos_to_date(self.last_access_stamp),
            total_request_count: self.request_count(),
        }
    }

    pub fn public_information(&self) -> UserPublicInformation {
        let joined_date = if self.join_date_public {
            nanos_to_date(self.created_stamp)
        } else {
            String::new()
        };
        let last_online_date = if self.online_date_public {
            nanos_to_date(self.last_access_stamp)
        } else {
            String::new()
        };
        UserPublicInformation {
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            joined_date,
            last_online_date,
        }
    }
}

pub async fn find_user_by_key(
    collection: &Collection<User>,
    key: &str,
) -> mongodb::error::Result<Option<User>> {
    let filter = doc! { "key": key };
    let user = collection.find_one(Some(filter), None).await?;
    Ok(user)
}

pub async fn find_user_by_name(
    collection: &Collection<User>,
    name: &str,
) -> mongodb::error::Result<Option<User>> {
    let filter = doc! { "name": name };
    let user = collection.find_one(Some(filter), None).await?;
    Ok(user)
}
