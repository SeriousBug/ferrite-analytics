use clap::Args;
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use sea_query::{Expr, IntoCondition, Query};
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
    #[serde(rename = "and")]
    FilterAnd(Vec<Filter>),
    #[serde(rename = "or")]
    FilterOr(Vec<Filter>),
    #[serde(rename = "filter")]
    FilterValue { name: String, eq: String },
}

impl Filter {
    fn to_query(&self) -> Condition {
        match self {
            Filter::FilterAnd(filters) => {
                let mut cond = Condition::all();
                for filter in filters {
                    cond = cond.add(filter.to_query());
                }
                cond
            }
            Filter::FilterOr(filters) => {
                let mut cond = Condition::any();
                for filter in filters {
                    cond = cond.add(filter.to_query());
                }
                cond
            }
            Filter::FilterValue { name, eq } => event::Column::Key
                .in_subquery(
                    Query::select()
                        .column(property::Column::EventKey)
                        .from(property::Entity)
                        .and_where(Expr::col(property::Column::Name).eq(name))
                        .and_where(Expr::col(property::Column::Value).eq(eq))
                        .to_owned(),
                )
                .into_condition(),
        }
    }
}

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

        let mut query = event::Entity::find().filter(
            event::Column::Date
                .gt(&self.from_date)
                .and(event::Column::Date.lt(&self.to_date)),
        );
        if let Some(filter_json) = &self.filter {
            let filters = serde_json::from_str::<Filter>(filter_json)?;
            query = query.filter(filters.to_query());
        }

        let events = query.count(&db).await.unwrap();
        println!("{:}", events);

        Ok(())
    }
}
