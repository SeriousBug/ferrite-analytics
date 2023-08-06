use axum::extract::State;
use lazy_static::lazy_static;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
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
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let db: DatabaseConnection = Database::connect(db_url).await?;
    Migrator::up(&db, None).await?;

    if db
        .get_database_backend()
        .eq(&sea_orm::DatabaseBackend::Sqlite)
    {
        // synchronous=NORMAL reduces how often Sqlite flushes to disk,
        // journal_mode=WAL is required to make prevent corruption in this mode.
        db.execute_unprepared("PRAGMA journal_mode=WAL;").await?;
        db.execute_unprepared("PRAGMA synchronous=NORMAL;").await?;
        // This improves performance, because we don't really care if a query
        // reads an uncommitted event. (Does it? Need to benchmark with it
        // on/off. Especially doing queries while writes are happening.)
        //db.execute_unprepared("PRAGMA read_uncommitted=ON;").await?;
    }

    drop(lock);
    Ok(db)
}

lazy_static! {
    static ref MIGRATION_LOCK: Mutex<()> = Mutex::new(());
}
