extern crate rocket;

use rocket::{get, post};

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

// #[post("/")]
// pub fn login() -> &'static str {}