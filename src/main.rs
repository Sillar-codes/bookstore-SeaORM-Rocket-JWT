use controllers::{Response, SuccessResponse};
use fairings::cors::CORS;
use migrator::Migrator;
use rocket::http::Status;
use sea_orm_migration::MigratorTrait;

#[macro_use]
extern crate rocket;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("5432".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("postgres".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD")
                .unwrap_or("sillarpostgre123".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello Wolrd".to_string())))
}

#[rocket::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::default();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    match Migrator::up(&db, None).await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }

    let _ = rocket::build()
        .attach(CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![
                controllers::auth::sign_in,
                controllers::auth::sign_up,
                controllers::auth::me
            ],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
                controllers::authors::get_books
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete,
            ],
        )
        .launch()
        .await;
}
