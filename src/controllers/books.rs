use std::time::SystemTime;

use crate::auth::AuthenticatedUser;
use crate::entities::{book, prelude::*};
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::{
    State,
    serde::{Serialize, json::Json},
};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder};

use super::{ErrorResponse, Response, SuccessResponse};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    author_id: i32,
    title: String,
    year: String,
    cover: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    pub total: usize,
    pub books: Vec<ResBook>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|book| ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title.to_owned(),
            year: book.year.to_owned(),
            cover: book.cover.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id),
        author_id: Set(req_book.author_id.to_owned()),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
        }),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(b) => b,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Cannot find abook with the specified ID.".to_string(),
            )));
        }
    };

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title,
            year: book.year,
            cover: book.cover,
        }),
    )))
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let mut book: book::ActiveModel = match Book::find_by_id(id).one(db).await? {
        Some(b) => b.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No book with that specified ID.".to_string(),
            )));
        }
    };

    book.author_id = Set(req_book.author_id);
    book.title = Set(req_book.title.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.cover = Set(req_book.cover.to_owned());

    book.updated_at = Set(DateTimeUtc::from(SystemTime::now()).naive_local());

    let book = book.update(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title.to_owned(),
            year: book.year.to_owned(),
            cover: book.cover.to_owned(),
        }),
    )))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _auth: AuthenticatedUser,
    id: i32,
) -> Response<String> {
    let db = db as &DatabaseConnection;

    let book = match Book::find_by_id(id).one(db).await? {
        Some(a) => a,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No author with the specified ID.".to_string(),
            )));
        }
    };

    book.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Book deleted".to_string())))
}
