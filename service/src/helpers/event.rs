use std::collections::HashMap;

use sea_orm::{ActiveModelTrait, Set, TransactionTrait};

use crate::entity::{event as Event, property as Property};
use crate::state::AppState;

#[async_trait::async_trait]
pub trait EventHelper {
    async fn save_event(self, name: &str, properties: HashMap<&str, String>);
}

#[async_trait::async_trait]
impl EventHelper for AppState {
    async fn save_event(self, name: &str, properties: HashMap<&str, String>) {
        let txn = self.db.begin().await.unwrap();
        let event = Event::ActiveModel {
            key: Set(ulid::Ulid::new().to_string()),
            date: Set(chrono::Utc::now().to_rfc3339()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        // The name of the event is also a property.
        Property::ActiveModel {
            event_key: Set(event.key.to_owned()),
            name: Set("name".to_string()),
            value: Set(name.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await
        .unwrap();

        for (name, value) in properties {
            Property::ActiveModel {
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
