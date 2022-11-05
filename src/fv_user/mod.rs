extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Row;

use crate::repository::FvDb;
pub mod fv_user_dto;
use fv_user_dto::FvUserDto;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Member Stage", |rocket| async {
        rocket
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/fv_user",
                   routes![
                       login,
                       member_list,
                       get_member_by_id
                   ])
    })
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<FvUserDto>) -> String {
    let user = login.into_inner();
    format!("Hello, {:?}", user)
}

#[get("/list")]
pub fn member_list() -> String {
    let member_list = String::from("fv_user list call");

    return member_list
}

#[get("/<id>")]
pub async fn get_member_by_id(_db: &FvDb, id: i64) -> String {
    // sqlx::query("SELECT content FROM logs WHERE id = ?").bind(id)
    //     .fetch_one(&db.0).await
    //     .and_then(|r| Ok(Log(r.try_get(0)?)))
    //     .ok();

    return format!("test {}", id)
}