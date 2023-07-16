use axum::Json;
use http::StatusCode;
use password_auth::{generate_hash, is_hash_obsolete, verify_password};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entity::account::{self, Entity as Account};
use crate::helpers::token::issue_token;
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
            return Ok(issue_token(account.id).await);
        }
    }
    // Failed to log in
    Err(StatusCode::UNAUTHORIZED)
}
