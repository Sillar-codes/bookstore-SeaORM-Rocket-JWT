use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Email).unique_key().not_null())
                    .col(string(User::Password).not_null())
                    .col(string(User::Firstname).not_null())
                    .col(string(User::Lastname).not_null())
                    .col(timestamp(User::CreatedAt).extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .col(timestamp(User::UpdatedAt).extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    Firstname,
    Lastname,
    CreatedAt,
    UpdatedAt,
}
