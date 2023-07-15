use clap::Args;
use futures::StreamExt;
use sea_orm::EntityTrait;

use crate::cli::run_command::RunCommand;
use crate::entity::account;

#[derive(Debug, Args)]
pub struct AccountList {}

#[async_trait::async_trait]
impl RunCommand for AccountList {
    async fn run(self) -> anyhow::Result<()> {
        let db = crate::state::get_db().await?;

        let mut accounts = account::Entity::find().stream(&db).await?;
        let mut count: usize = 0;

        println!("Accounts:");
        while let Some(account) = accounts.next().await {
            println!("\t{:}", account?.username);
            count += 1;
        }
        println!("Total {:} accounts", count);
        Ok(())
    }
}
