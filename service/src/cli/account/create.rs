use clap::Args;
use password_auth::generate_hash;
use sea_orm::{ActiveModelTrait, Set};

use crate::cli::run_command::RunCommand;
use crate::entity::account;

#[derive(Debug, Args)]
pub struct AccountCreate {
    name: String,
    password: String,
}

#[async_trait::async_trait]
impl RunCommand for AccountCreate {
    async fn run(self) -> anyhow::Result<()> {
        let hash = generate_hash(&self.password);
        let account = account::ActiveModel {
            id: Set(ulid::Ulid::new().to_string()),
            username: Set(self.name),
            hashed_password: Set(hash),
        }
        .insert(&crate::state::get_db().await?)
        .await?;

        println!("Created account {:}", account.username);
        Ok(())
    }
}
