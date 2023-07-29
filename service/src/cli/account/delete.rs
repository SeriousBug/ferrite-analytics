use clap::Args;
use sea_orm::{ColumnTrait, Condition, EntityTrait, ModelTrait, QueryFilter};

use crate::cli::run_command::RunCommand;
use crate::entity::account;

#[derive(Debug, Args)]
pub struct AccountDelete {
    name: String,
}

#[async_trait::async_trait]
impl RunCommand for AccountDelete {
    async fn run(&self) -> anyhow::Result<()> {
        let db = crate::state::get_db().await?;
        let account = account::Entity::find()
            .filter(Condition::all().add(account::Column::Username.eq(&self.name)))
            .one(&db)
            .await?;
        if let Some(account) = account {
            account.delete(&db).await?;
            println!("Deleted account {:}", &self.name);
        } else {
            println!("Account {:} not found", &self.name);
        }

        Ok(())
    }
}
