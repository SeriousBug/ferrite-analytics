pub mod account;
pub mod benchmark;
pub mod query;
pub mod run_command;

use std::process::exit;

use clap::{Parser, Subcommand};

use self::{
    account::Account, benchmark::BenchmarkCommand, query::QueryCommand, run_command::RunCommand,
};

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
    Query(QueryCommand),
    Benchmark(BenchmarkCommand),
}

#[async_trait::async_trait]
impl RunCommand for Commands {
    async fn run(&self) -> anyhow::Result<()> {
        match self {
            Commands::Account(account) => account.run().await,
            Commands::Query(query) => query.run().await,
            Commands::Benchmark(benchmark) => benchmark.run().await,
        }
    }
}
