use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_query::{Condition, Expr, IntoCondition, Query};
use serde::{Deserialize, Serialize};

use crate::entity::{event, property};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub from_date: String,
    pub to_date: String,
    #[serde(flatten)]
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
    pub fn to_query(&self) -> Condition {
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

impl QueryData {
    pub async fn run(&self, db: &DatabaseConnection) -> u64 {
        event::Entity::find()
            .filter(
                event::Column::Date
                    .gt(&self.from_date)
                    .and(event::Column::Date.lt(&self.to_date)),
            )
            .filter(self.filter.to_query())
            .count(db)
            .await
            .unwrap()
    }
}
