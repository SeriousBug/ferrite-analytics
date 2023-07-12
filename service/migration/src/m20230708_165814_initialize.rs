use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // The events table identifies the events by the time they occurred.
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Event::Key).string().not_null().primary_key())
                    .col(ColumnDef::new(Event::Date).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        // The properties table stores the actual data attached to the events.
        manager
            .create_table(
                Table::create()
                    .table(Property::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Property::EventKey).string().not_null())
                    .col(ColumnDef::new(Property::Name).string().not_null())
                    .col(ColumnDef::new(Property::Value).string().not_null())
                    .col(ColumnDef::new(Property::ValueType).integer().not_null())
                    .primary_key(Index::create().col(Property::EventKey).col(Property::Name))
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Property::Table)
                            .from_col(Property::EventKey)
                            .to_tbl(Event::Table)
                            .to_col(Event::Key)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Property::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Event {
    Table,
    Key,
    Date,
}

#[derive(Iden)]
pub enum Property {
    Table,
    EventKey,
    Name,
    Value,
    ValueType,
}
