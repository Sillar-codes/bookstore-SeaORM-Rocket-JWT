use super::Response;
use rocket::{
    State,
    serde::{Deserialize, Serialize, json::Json},
};
use sea_orm::DatabaseConnection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<String> {
    todo!()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignUp {
    token: String,
}

#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>,
) -> Response<String> {
    todo!()
}
