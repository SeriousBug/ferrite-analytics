use async_once::AsyncOnce;
use axum::Json;
use chrono::Utc;
use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use password_auth::{generate_hash, is_hash_obsolete, verify_password};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::entity::account::{self, Entity as Account};
use crate::entity::meta::{self, Entity as Meta};
use crate::helpers::meta::JWT_SECRET_KEY;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn post(state: AppState, Json(login): Json<LoginRequest>) -> Result<String, StatusCode> {
    let db = &state.db;
    let account = Account::find()
        .filter(Condition::all().add(account::Column::Username.eq(login.username)))
        .one(db)
        .await
        .unwrap();
    if let Some(account) = account {
        // Found account
        if verify_password(&login.password, &account.hashed_password).is_ok() {
            // Password is good
            if is_hash_obsolete(&account.hashed_password).unwrap() {
                // Need to rehash
                let new_hash = generate_hash(&login.password);
                let mut updated_account: account::ActiveModel = account.clone().into();
                updated_account.hashed_password = Set(new_hash);
                updated_account.update(db).await.unwrap();
            }
            // Logged in, return JWT token
            return Ok(issue_token(account.id));
        }
    }
    // Failed to log in
    Err(StatusCode::UNAUTHORIZED)
}

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
    static ref JWT_SECRET: AsyncOnce<String> = {
        AsyncOnce::new(async {
            get_or_generate_secret(&crate::state::get_db().await.unwrap()).await
        })
    };
}

fn issue_token(account_id: String) -> String {
    // Issued at
    let iat = Utc::now().timestamp() as usize;
    // Expires in
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(365))
        .unwrap()
        .timestamp() as usize;
    let claims = Claims {
        sub: account_id,
        exp,
        iat,
        iss: "basalytics",
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: &'static str,
    sub: String,
    exp: usize,
    iat: usize,
}
