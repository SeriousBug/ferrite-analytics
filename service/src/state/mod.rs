use axum::extract::State;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

pub struct AppStateData {
    pub db: DatabaseConnection,
}

pub type AppState = State<Arc<AppStateData>>;

pub async fn get_db() -> anyhow::Result<DatabaseConnection> {
    let db: DatabaseConnection = Database::connect("sqlite://test.sqlite?mode=rwc").await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
