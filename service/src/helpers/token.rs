use async_once::AsyncOnce;
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::entity::meta::{self, Entity as Meta};
use crate::entity::token_invalidation;
use crate::helpers::meta::JWT_SECRET_KEY;

async fn get_or_generate_secret(db: &DatabaseConnection) -> String {
    let secret = Meta::find_by_id(JWT_SECRET_KEY).one(db).await.unwrap();
    if let Some(secret) = secret {
        return secret.value;
    }
    let secret = meta::ActiveModel {
        key: Set(JWT_SECRET_KEY.to_string()),
        value: Set(ulid::Ulid::new().to_string()),
    }
    .insert(db)
    .await
    .unwrap();
    secret.value
}

lazy_static! {
    pub static ref JWT_SECRET: AsyncOnce<String> = {
        AsyncOnce::new(async {
            get_or_generate_secret(&crate::state::get_db().await.unwrap()).await
        })
    };
    pub static ref JWT_ENCODING_SECRET: AsyncOnce<EncodingKey> =
        AsyncOnce::new(async { EncodingKey::from_secret(JWT_SECRET.get().await.as_ref()) });
    pub static ref JWT_DECODING_SECRET: AsyncOnce<DecodingKey> =
        AsyncOnce::new(async { DecodingKey::from_secret(JWT_SECRET.get().await.as_ref()) });
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Issuer
    pub iss: String,
    /// Subject (account ID)
    pub sub: String,
    /// Expires at (UNIX timestamp)
    pub exp: usize,
    /// Issued at (UNIX timestamp)
    pub iat: usize,
}

pub async fn issue_token(account_id: String) -> String {
    // Issued at
    let iat = Utc::now().timestamp() as usize;
    // Expires in
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
        .timestamp() as usize;
    let claims = TokenClaims {
        sub: account_id,
        exp,
        iat,
        iss: "ferrite".to_string(),
    };
    encode(&Header::default(), &claims, JWT_ENCODING_SECRET.get().await).unwrap()
}

/// Returns the token claims if the token was valid, or None if it was invalid
pub async fn verify_token(db: &DatabaseConnection, token: &str) -> Option<TokenClaims> {
    let secret = JWT_SECRET.get().await;
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    if let Ok(token_data) = token_data {
        let account_id = &token_data.claims.sub;
        let invalidation = token_invalidation::Entity::find_by_id(account_id)
            .one(db)
            .await
            .unwrap();
        if let Some(invalidation) = invalidation {
            // The user issued a token invalidation some time in the past. Check if this token was issued after that.
            let invalidated_at_timestamp =
                DateTime::parse_from_rfc3339(&invalidation.invalidated_at)
                    .unwrap()
                    .timestamp() as usize;
            // If the token was issued after the invalidation, so it is valid
            if token_data.claims.iat < invalidated_at_timestamp {
                return None;
            }
        }
        // This user never invalidated, so the token is definitely valid
        return Some(token_data.claims);
    }
    return None;
}
