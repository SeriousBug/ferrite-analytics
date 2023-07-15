use async_trait::async_trait;
use clap::{Args, Subcommand};

use self::{create::AccountCreate, delete::AccountDelete, list::AccountList};

use super::run_command::RunCommand;

pub mod create;
pub mod delete;
pub mod list;

#[derive(Debug, Subcommand)]
pub enum AccountCommands {
    Create(AccountCreate),
    Delete(AccountDelete),
    List(AccountList),
}

#[async_trait]
impl RunCommand for AccountCommands {
    async fn run(self) -> anyhow::Result<()> {
        match self {
            Self::Create(cmd) => cmd.run().await,
            Self::Delete(cmd) => cmd.run().await,
            Self::List(cmd) => cmd.run().await,
        }
    }
}

#[derive(Debug, Args)]
pub struct Account {
    #[command(subcommand)]
    command: AccountCommands,
}

#[async_trait]
impl RunCommand for Account {
    async fn run(self) -> anyhow::Result<()> {
        self.command.run().await
    }
}
