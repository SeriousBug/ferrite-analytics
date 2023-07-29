use clap::Args;
use futures::StreamExt;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::cli::run_command::RunCommand;
use crate::entity::{event, property};
use crate::state::get_db;

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    from_date: String,
    to_date: String,
    filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
enum Filter {
    FilterAnd(Vec<Filter>),
    FilterOr(Vec<Filter>),
    FilterValue { name: String, eq: String },
}

#[derive(Debug, Args)]
pub struct QueryCommand {
    from_date: String,
    to_date: String,
}

#[async_trait::async_trait]
impl RunCommand for QueryCommand {
    async fn run(&self) -> anyhow::Result<()> {
        let db = get_db().await?;
        let mut events = event::Entity::find()
            .find_with_related(property::Entity)
            .filter(
                event::Column::Date
                    .gt(self.from_date)
                    .and(event::Column::Date.lt(self.to_date))
                    .and(property::Column::Name.eq("name")),
            )
            .stream(&db)
            .await
            .unwrap();

        while let Some(event) = events.next().await {
            println!("{:?}", event);
        }
        Ok(())
    }
}
