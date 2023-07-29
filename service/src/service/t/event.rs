use std::collections::HashMap;

use crate::helpers::event::EventDataTypes;
use crate::state::AppState;
use crate::{entity, helpers::session_id::SessionId};

use axum::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    properties: HashMap<String, serde_json::Value>,
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

    for (name, value) in event.properties {
        if let Some(value_type) = get_value_type(&value) {
            entity::property::ActiveModel {
                event_key: Set(event_model.key.to_owned()),
                name: Set(name.to_string()),
                value: Set(value.to_string()),
                value_type: Set(value_type as i32),
                ..Default::default()
            }
            .insert(&txn)
            .await
            .unwrap();
        }
    }

    entity::property::ActiveModel {
        event_key: Set(event_model.key.to_owned()),
        name: Set("session".to_string()),
        value: Set(session_id.to_string()),
        value_type: Set(EventDataTypes::String as i32),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .unwrap();

    txn.commit().await.unwrap();
}

fn get_value_type(value: &Value) -> Option<EventDataTypes> {
    if value.is_string() {
        Some(EventDataTypes::String)
    } else if value.is_number() {
        Some(EventDataTypes::Number)
    } else if value.is_boolean() {
        Some(EventDataTypes::Boolean)
    } else if value.is_null() {
        Some(EventDataTypes::Null)
    } else {
        None
    }
}
