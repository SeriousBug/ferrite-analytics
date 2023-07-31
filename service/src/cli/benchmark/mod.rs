use chrono::Days;
use clap::Args;
use futures::StreamExt;
use rand::seq::SliceRandom;
use rand::{rngs::SmallRng, SeedableRng};
use sea_orm::{ActiveModelTrait, Set};

use crate::{entity, helpers::event::EventDataTypes, state::get_db};

use super::{
    query::{run_query, Filter, QueryData},
    run_command::RunCommand,
};

#[derive(Debug, Args)]
pub struct BenchmarkCommand {
    #[arg(long)]
    insert_count: usize,
    #[arg(long)]
    query_count: usize,
    #[arg(long)]
    query_specificity_date: usize,
    #[arg(long)]
    query_specificity_property: usize,
}

#[async_trait::async_trait]
impl RunCommand for BenchmarkCommand {
    async fn run(&self) -> anyhow::Result<()> {
        let db = get_db().await?;
        let now = chrono::Utc::now();

        let mut possible_names: Vec<String> = Vec::new();
        for i in 0..self.query_specificity_date {
            possible_names.push(format!("name{}", i));
        }
        let mut possible_dates: Vec<String> = Vec::new();
        for i in 0..self.query_specificity_property {
            possible_dates.push(
                now.checked_add_days(Days::new(i as u64))
                    .unwrap()
                    .to_rfc3339(),
            );
        }

        let mut rng = SmallRng::from_entropy();

        let insert_start = chrono::Utc::now();
        for _ in 0..self.insert_count {
            let event = entity::event::ActiveModel {
                key: Set(ulid::Ulid::new().to_string()),
                date: Set(possible_dates.choose(&mut rng).unwrap().clone()),
                ..Default::default()
            }
            .insert(&db)
            .await
            .unwrap();

            // The name of the event is also a property.
            entity::property::ActiveModel {
                event_key: Set(event.key.to_owned()),
                name: Set("name".to_string()),
                value: Set(possible_names.choose(&mut rng).unwrap().clone()),
                value_type: Set(EventDataTypes::String as i32),
                ..Default::default()
            }
            .insert(&db)
            .await
            .unwrap();
        }
        let insert_end = chrono::Utc::now();
        let insert_duration = insert_end - insert_start;
        println!(
            "Inserting {} events took {}ms",
            self.insert_count,
            insert_duration.num_milliseconds(),
        );

        println!("Running {} queries", self.query_count);
        let all_queries_start = chrono::Utc::now();
        for i in 0..self.query_count {
            let date = possible_dates.choose(&mut rng).unwrap();

            let query_data = QueryData {
                from_date: date.clone(),
                to_date: date.clone(),
                filter: Filter::FilterValue {
                    name: "name".to_string(),
                    eq: possible_names.choose(&mut rng).unwrap().clone(),
                },
            };

            let start = chrono::Utc::now();

            let events = run_query(&db, query_data).await;

            let end = chrono::Utc::now();
            let duration = end - start;
            println!(
                "Query {} took {}ms, {} events",
                i,
                duration.num_milliseconds(),
                events
            );
        }
        let all_queries_end = chrono::Utc::now();
        let all_queries_duration = all_queries_end - all_queries_start;
        println!(
            "All queries took {}ms, {}ms on average per query",
            all_queries_duration.num_milliseconds(),
            all_queries_duration.num_milliseconds() / self.query_count as i64
        );

        Ok(())
    }
}
