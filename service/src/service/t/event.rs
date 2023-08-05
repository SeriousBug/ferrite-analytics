use std::collections::HashMap;

use crate::entity::{property_boolean, property_integer, property_string};
use crate::helpers::event::EventValue;
use crate::state::AppState;
use crate::{entity, helpers::session_id::SessionId};

use axum::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DatabaseTransaction, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    properties: HashMap<String, EventValue>,
}

pub async fn post(session_id: SessionId, state: AppState, Json(events): Json<Vec<Event>>) {
    // Save the event data
    for event in events {
        save_event(event, state.db.clone(), &session_id).await;
    }
}

async fn save_event(event: Event, db: DatabaseConnection, SessionId(session_id): &SessionId) {
    let txn = db.begin().await.unwrap();
    let event_model = entity::event::ActiveModel {
        key: Set(ulid::Ulid::new().to_string()),
        date: Set(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .unwrap();

    property_string::ActiveModel {
        event_key: Set(event_model.key.to_owned()),
        name: Set("name".to_string()),
        value: Set(event.name.to_string()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .unwrap();

    for (name, value) in event.properties {
        save_json_property(&txn, event_model.key.to_owned(), name, value).await;
    }

    property_string::ActiveModel {
        event_key: Set(event_model.key.to_owned()),
        name: Set("session".to_string()),
        value: Set(session_id.to_string()),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .unwrap();

    txn.commit().await.unwrap();
}

async fn save_json_property(
    txn: &DatabaseTransaction,
    event_key: String,
    name: String,
    value: EventValue,
) {
    match value {
        EventValue::Boolean(value) => {
            property_boolean::ActiveModel {
                event_key: Set(event_key),
                name: Set(name),
                value: Set(value),
                ..Default::default()
            }
            .insert(txn)
            .await
            .unwrap();
        }
        EventValue::Integer(value) => {
            property_integer::ActiveModel {
                event_key: Set(event_key),
                name: Set(name),
                value: Set(value),
                ..Default::default()
            }
            .insert(txn)
            .await
            .unwrap();
        }
        EventValue::String(value) => {
            property_string::ActiveModel {
                event_key: Set(event_key),
                name: Set(name),
                value: Set(value),
                ..Default::default()
            }
            .insert(txn)
            .await
            .unwrap();
        }
    };
}
