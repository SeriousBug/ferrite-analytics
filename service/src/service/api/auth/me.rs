use axum::response::IntoResponse;

use crate::helpers::token::TokenClaims;

pub async fn get(claims: TokenClaims) -> impl IntoResponse {
    claims.sub
}
