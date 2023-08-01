use clap::Args;

use crate::cli::run_command::RunCommand;
use crate::helpers::query::{Filter, QueryData};
use crate::state::get_db;

#[derive(Debug, Args)]
pub struct QueryCommand {
    from_date: String,
    to_date: String,
    filter: Option<String>,
}

#[async_trait::async_trait]
impl RunCommand for QueryCommand {
    async fn run(&self) -> anyhow::Result<()> {
        let db = get_db().await?;

        let query = QueryData {
            from_date: self.from_date.to_owned(),
            to_date: self.to_date.to_owned(),
            filter: if let Some(filter_json) = &self.filter {
                serde_json::from_str::<Filter>(filter_json).unwrap()
            } else {
                Filter::FilterAnd(vec![])
            },
        };

        let events = query.run(&db).await;
        println!("{:}", events);

        Ok(())
    }
}
