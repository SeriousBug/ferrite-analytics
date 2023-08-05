use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use sea_query::{Condition, Expr, IntoCondition, Query};
use serde::{Deserialize, Serialize};

use crate::entity::{event, property_boolean, property_integer, property_string};

use super::event::EventValue;

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
    FilterValue { name: String, eq: EventValue },
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
                .in_subquery(match eq {
                    EventValue::Boolean(eq) => Query::select()
                        .column(property_boolean::Column::EventKey)
                        .from(property_boolean::Entity)
                        .and_where(Expr::col(property_boolean::Column::Name).eq(name))
                        .and_where(Expr::col(property_boolean::Column::Value).eq(*eq))
                        .to_owned(),
                    EventValue::Integer(eq) => Query::select()
                        .column(property_integer::Column::EventKey)
                        .from(property_integer::Entity)
                        .and_where(Expr::col(property_integer::Column::Name).eq(name))
                        .and_where(Expr::col(property_integer::Column::Value).eq(*eq))
                        .to_owned(),
                    EventValue::String(eq) => Query::select()
                        .column(property_string::Column::EventKey)
                        .from(property_string::Entity)
                        .and_where(Expr::col(property_string::Column::Name).eq(name))
                        .and_where(Expr::col(property_string::Column::Value).eq(eq))
                        .to_owned(),
                })
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
