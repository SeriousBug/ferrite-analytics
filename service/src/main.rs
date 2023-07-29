pub mod cli;
mod entity;
mod helpers;
pub mod middleware;
mod service;
mod state;

use axum::routing::post;
use axum::{http::header, routing::get, Router};
use clap::Parser;
use cli::run_command::RunCommand;
use cli::Cli;
use state::get_db;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.run().await.unwrap();

    let state = Arc::new(state::AppStateData {
        db: get_db().await.unwrap(),
        forwarded_ip_header: cli.forward_ip_header.to_owned(),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/t/t.png", get(crate::service::t::t_png::get))
        .route("/t/event", post(crate::service::t::event::post))
        .route("/r/script.js", get(crate::service::r::script_js::get))
        .route("/api/auth/login", post(crate::service::api::auth::login::post))
        .route("/api/auth/me", get(crate::service::api::auth::me::get))
        .route(
            "/",
            get(|| async {
                // Placeholder page for testing
                (
                    [(header::CONTENT_TYPE, "html")],
                    "<!DOCTYPE html><html><body><img src=\"/t/tx.png\" width=1 height=1 /><div id=\"foo\">button</div><script src=\"/r/script.js\">{\"trackSessions\": true, \"eventTrackers\": [{\"selector\": \"#foo\", \"event\": \"click\"}], \"visibilityTrackers\": [{\"selector\": \"#foo\"}]}</script></body></html>",
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
