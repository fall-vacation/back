extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::form::Form;

pub mod fv_user_dto;
use fv_user_dto::FvUserDto;


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Member Stage", |rocket| async {
        rocket
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/member", routes![login, member_list])
    })
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Form<FvUserDto>) -> String {
    let user = login.into_inner();
    format!("Hello, {:?}", user)
}

#[get("/list")]
pub fn member_list() -> String {
    let member_list = String::from("member list call");

    return member_list
}