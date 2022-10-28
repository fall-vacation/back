extern crate rocket;

use rocket::{get, post};
use rocket::form::Form;
use super::dto::UserLogin;

#[get("/")]
pub fn index() -> &'static str {
    println!("get root request...");
    "hello world"
}

#[get("/world")]
pub fn print_world() -> &'static str {
    println!("get hello/world request...");
    "world"
}

// #[post("/login", data = "<login>")]
// pub fn login(login: Form<UserLogin>) -> String {
//     format!("Hello, {}", login.name)
// }