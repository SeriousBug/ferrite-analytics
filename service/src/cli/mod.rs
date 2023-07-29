pub mod account;
pub mod run_command;

use std::process::exit;

use clap::{Parser, Subcommand};

use self::{account::Account, run_command::RunCommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long)]
    pub forward_ip_header: Option<String>,
}

#[async_trait::async_trait]
impl RunCommand for Cli {
    async fn run(&self) -> anyhow::Result<()> {
        if let Some(command) = &self.command {
            command.run().await?;
            exit(0);
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Account(Account),
}

#[async_trait::async_trait]
impl RunCommand for Commands {
    async fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::Account(account) => account.run().await,
        }
    }
}
