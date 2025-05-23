use sea_orm_migration::{prelude::*, schema::*};

use crate::migrator::m20220101_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(pk_auto(Author::Id))
                    .col(integer(Author::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-author-user_id")
                            .from(Author::Table, Author::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(string(Author::Firstname).not_null())
                    .col(string(Author::Lastname).not_null())
                    .col(string(Author::Bio).not_null())
                    .col(timestamp(Author::CreatedAt).extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .col(timestamp(Author::UpdatedAt).extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Author {
    Table,
    Id,
    UserId,
    Firstname,
    Lastname,
    Bio,
    CreatedAt,
    UpdatedAt,
}
