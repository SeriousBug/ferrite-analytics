use axum::extract::State;
use lazy_static::lazy_static;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppStateData {
    pub db: DatabaseConnection,
    pub forwarded_ip_header: Option<String>,
}

pub type AppState = State<Arc<AppStateData>>;

pub async fn get_db() -> anyhow::Result<DatabaseConnection> {
    // Not sure if this is strictly required, but we'll use a lock to ensure
    // that we don't try to run migrations in parallel.
    let lock = MIGRATION_LOCK.lock().await;
    let db: DatabaseConnection = Database::connect("sqlite://test.sqlite?mode=rwc").await?;
    Migrator::up(&db, None).await?;
    drop(lock);
    Ok(db)
}

lazy_static! {
    static ref MIGRATION_LOCK: Mutex<()> = Mutex::new(());
}
