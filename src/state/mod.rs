use axum::extract::State;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppStateData {
    pub db: DatabaseConnection,
}

pub type AppState = State<Arc<AppStateData>>;
