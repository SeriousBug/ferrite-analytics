use axum::{response::IntoResponse, Json};

use crate::{
    helpers::{query::QueryData, token::TokenClaims},
    state::AppState,
};

pub async fn post(
    _: TokenClaims,
    state: AppState,
    Json(query_data): Json<QueryData>,
) -> impl IntoResponse {
    query_data.run(&state.db).await.to_string()
}
