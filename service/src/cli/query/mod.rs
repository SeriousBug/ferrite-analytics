use clap::Args;
use futures::stream::BoxStream;
use futures::StreamExt;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_query::{Expr, IntoCondition, Query};
use serde::{Deserialize, Serialize};

use crate::cli::run_command::RunCommand;
use crate::entity::{event, property};
use crate::state::get_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub from_date: String,
    pub to_date: String,
    pub filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
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

pub async fn run_query<'a>(
    db: &'a DatabaseConnection,
    data: QueryData,
) -> BoxStream<'a, Result<event::Model, DbErr>> {
    let mut query = event::Entity::find().filter(
        event::Column::Date
            .gte(&data.from_date)
            .and(event::Column::Date.lte(&data.to_date)),
    );
    query = query.filter(data.filter.to_query());

    query.stream(db).await.unwrap().boxed()
}

#[async_trait::async_trait]
impl RunCommand for QueryCommand {
    async fn run(&self) -> anyhow::Result<()> {
        let db = get_db().await?;

        let query_data = QueryData {
            from_date: self.from_date.clone(),
            to_date: self.to_date.clone(),
            filter: if let Some(filter_json) = &self.filter {
                serde_json::from_str(filter_json)?
            } else {
                Filter::FilterAnd(vec![])
            },
        };
        let mut events = run_query(&db, query_data).await;

        println!("key,date");
        while let Some(event) = events.next().await {
            let event = event.unwrap();
            println!("{},{}", event.key, event.date);
        }

        Ok(())
    }
}
