use axum::extract::FromRequestParts;
use http::{header, request::Parts, StatusCode};
use std::sync::Arc;

use crate::{
    helpers::token::{verify_token, TokenClaims},
    state,
};

#[async_trait::async_trait]
impl FromRequestParts<Arc<state::AppStateData>> for TokenClaims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<state::AppStateData>,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));

        if let Some(token) = token {
            if let Some(claims) = verify_token(&state.db, token).await {
                return Ok(claims);
            }
        }
        Err(StatusCode::UNAUTHORIZED)
    }
}
