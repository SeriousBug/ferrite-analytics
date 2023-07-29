use clap::Args;
use futures::StreamExt;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use sea_query::{Expr, Query};
use serde::{Deserialize, Serialize};

use crate::cli::run_command::RunCommand;
use crate::entity::{event, property};
use crate::state::get_db;

#[derive(Debug, Serialize, Deserialize)]
struct QueryData {
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
            .filter(
                event::Column::Date
                    .gt(&self.from_date)
                    .and(event::Column::Date.lt(&self.to_date)),
            )
            .filter(
                Condition::any().add(
                    event::Column::Key.in_subquery(
                        Query::select()
                            .column(property::Column::EventKey)
                            .from(property::Entity)
                            .and_where(Expr::col(property::Column::Name).eq("name"))
                            .and_where(Expr::col(property::Column::Value).eq("tracking-pixel"))
                            .to_owned(),
                    ),
                ),
            )
            .stream(&db)
            .await
            .unwrap();

        println!("Events:");
        while let Some(event) = events.next().await {
            println!("{:?}", event);
        }
        Ok(())
    }
}