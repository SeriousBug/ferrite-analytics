mod entity;
mod helpers;
mod service;
mod state;

use axum::routing::post;
use axum::{http::header, routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let state = Arc::new(state::AppStateData {
        db: get_db().await.unwrap(),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/t/t.png", get(crate::service::t::t_png::get))
        .route("/t/event", post(crate::service::t::event::post))
        .route("/r/script.js", get(crate::service::r::script_js::get))
        .route(
            "/",
            get(|| async {
                // Placeholder page for testing
                (
                    [(header::CONTENT_TYPE, "html")],
                    "<!DOCTYPE html><html><body><img src=\"/t/t.png\" /><div id=\"foo\">button</div><script src=\"/r/script.js\">{\"eventTrackers\": [{\"selector\": \"#foo\", \"event\": \"onclick\"}], \"visibilityTrackers\": [{\"selector\": \"#foo\"}]}</script></body></html>",
                )
            }),
        )
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn get_db() -> anyhow::Result<DatabaseConnection> {
    let db: DatabaseConnection = Database::connect("sqlite://test.sqlite?mode=rwc").await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
