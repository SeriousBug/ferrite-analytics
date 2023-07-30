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

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ux_name_value")
                    .table(Property::Table)
                    .col(Property::Name)
                    .col(Property::Value)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Account::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Account::HashedPassword).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ux_account_username")
                    .table(Account::Table)
                    .col(Account::Username)
                    .unique()
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Dashboard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dashboard::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dashboard::Name).string().not_null())
                    .col(ColumnDef::new(Dashboard::Description).string().not_null())
                    .col(ColumnDef::new(Dashboard::Configuration).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Tracker::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tracker::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tracker::Configuration).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(TrackingPixel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TrackingPixel::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TrackingPixel::EventName).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Meta::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Meta::Key).string().not_null().primary_key())
                    .col(ColumnDef::new(Meta::Value).string().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(TokenInvalidation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TokenInvalidation::AccountId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TokenInvalidation::InvalidatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(TokenInvalidation::Table)
                            .from_col(TokenInvalidation::AccountId)
                            .to_tbl(Account::Table)
                            .to_col(Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("ux_account_username")
                    .table(Account::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Dashboard::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tracker::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TrackingPixel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Property::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Meta::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TokenInvalidation::Table).to_owned())
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

#[derive(Iden)]
pub enum Account {
    Table,
    Id,
    Username,
    HashedPassword,
}

#[derive(Iden)]
pub enum Dashboard {
    Table,
    Id,
    Name,
    Description,
    Configuration,
}

#[derive(Iden)]
pub enum Tracker {
    Table,
    Id,
    Configuration,
}

#[derive(Iden)]
pub enum TrackingPixel {
    Table,
    Id,
    EventName,
}

#[derive(Iden)]
pub enum Meta {
    Table,
    Key,
    Value,
}

#[derive(Iden)]
pub enum TokenInvalidation {
    Table,
    AccountId,
    InvalidatedAt,
}
