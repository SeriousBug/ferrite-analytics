pub use sea_orm_migration::prelude::*;

mod m20230708_165814_initialize;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230708_165814_initialize::Migration)]
    }
}
