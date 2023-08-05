use std::collections::HashMap;

use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::entity::{event, property_string};
use crate::state::AppState;

#[async_trait::async_trait]
pub trait EventHelper {
    async fn save_event(self, name: &str, properties: HashMap<&str, String>);
}

#[async_trait::async_trait]
impl EventHelper for AppState {
    async fn save_event(self, name: &str, properties: HashMap<&str, String>) {
        let txn = self.db.begin().await.unwrap();
        let event = event::ActiveModel {
            key: Set(ulid::Ulid::new().to_string()),
            date: Set(chrono::Utc::now().to_rfc3339()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        // The name of the event is also a property.
        property_string::ActiveModel {
            event_key: Set(event.key.to_owned()),
            name: Set("name".to_string()),
            value: Set(name.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        for (name, value) in properties {
            property_string::ActiveModel {
                event_key: Set(event.key.to_owned()),
                name: Set(name.to_string()),
                value: Set(value.to_string()),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .unwrap();
        }

        txn.commit().await.unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventValue {
    Boolean(bool),
    Integer(i32),
    String(String),
}
