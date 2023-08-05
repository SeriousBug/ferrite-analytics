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

        // The properties tables stores the actual data attached to the events.
        manager
            .create_table(
                Table::create()
                    .table(PropertyString::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PropertyString::EventKey).string().not_null())
                    .col(ColumnDef::new(PropertyString::Name).string().not_null())
                    .col(ColumnDef::new(PropertyString::Value).string().not_null())
                    .primary_key(
                        Index::create()
                            .unique()
                            .col(PropertyString::EventKey)
                            .col(PropertyString::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PropertyString::Table)
                            .from_col(PropertyString::EventKey)
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
                    .name(&PropertyString::IndexNameValue.to_string())
                    .table(PropertyString::Table)
                    .col(PropertyString::Name)
                    .col(PropertyString::Value)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(PropertyInteger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PropertyInteger::EventKey)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PropertyInteger::Name).string().not_null())
                    .col(ColumnDef::new(PropertyInteger::Value).integer().not_null())
                    .primary_key(
                        Index::create()
                            .unique()
                            .col(PropertyInteger::EventKey)
                            .col(PropertyInteger::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PropertyInteger::Table)
                            .from_col(PropertyInteger::EventKey)
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
                    .name(&PropertyInteger::IndexNameValue.to_string())
                    .table(PropertyInteger::Table)
                    .col(PropertyInteger::Name)
                    .col(PropertyInteger::Value)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(PropertyBoolean::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PropertyBoolean::EventKey)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PropertyBoolean::Name).string().not_null())
                    .col(ColumnDef::new(PropertyBoolean::Value).boolean().not_null())
                    .primary_key(
                        Index::create()
                            .unique()
                            .col(PropertyBoolean::EventKey)
                            .col(PropertyBoolean::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PropertyBoolean::Table)
                            .from_col(PropertyBoolean::EventKey)
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
                    .name(&PropertyBoolean::IndexNameValue.to_string())
                    .table(PropertyBoolean::Table)
                    .col(PropertyBoolean::Name)
                    .col(PropertyBoolean::Value)
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
            .drop_table(Table::drop().table(PropertyString::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PropertyInteger::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PropertyBoolean::Table).to_owned())
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
pub enum PropertyString {
    Table,
    EventKey,
    Name,
    Value,
    IndexNameValue,
}
#[derive(Iden)]
pub enum PropertyInteger {
    Table,
    EventKey,
    Name,
    Value,
    IndexNameValue,
}
#[derive(Iden)]
pub enum PropertyBoolean {
    Table,
    EventKey,
    Name,
    Value,
    IndexNameValue,
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
