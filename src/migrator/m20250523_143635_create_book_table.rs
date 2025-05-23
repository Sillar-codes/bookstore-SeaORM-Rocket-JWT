use sea_orm_migration::{prelude::*, schema::*};

use super::{
    m20220101_000001_create_user_table::User, m20250523_142601_create_author_table::Author,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(pk_auto(Book::Id))
                    .col(integer(Book::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-user_id")
                            .from(Book::Table, Book::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(integer(Book::AuthorId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-author_id")
                            .from(Book::Table, Book::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .col(string(Book::Title))
                    .col(string(Book::Year))
                    .col(string(Book::Cover))
                    .col(timestamp(Book::CreatedAt).extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(timestamp(Book::UpdatedAt).extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    UserId,
    AuthorId,
    Title,
    Year,
    Cover,
    CreatedAt,
    UpdatedAt,
}
