extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;
use sqlx::Row;

pub mod farm;
use crate::repository::FvDb;
use crate::farm_service::farm::{Dto, Dao};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Farm Stage", |rocket| async {
        rocket
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/farm",
                   routes![
                       signup,
                   ])
    })
}

#[post("/signup", format = "json", data = "<farm>")]
pub async fn signup(db: Connection<FvDb>, farm: Json<Dto>) -> Value {
    let farm = farm.into_inner();
    let farm = farm.to_dao();
    let insert = farm.insert(db).await;
    // if farm.check_role_validation(){
    //     let farm = farm.to_dao();
    //     return match user.insert(db).await {
    //         Some(result) => {
    //             json!({
    //                 "user_id": result.get::<i32, _>("user_id"),
    //                 "user_nickname": result.get::<String, _>("user_nickname")
    //             })
    //         },
    //         None => {
    //             json!({
    //                 "user_id": -1,
    //                 "user_nickname": "db insert error"
    //             })
    //         },
    //     }
    // }
    json!({
        "user_id": -1,
        "user_nickname": "validation error"
    })
}

#[get("/<id>")]
pub async fn get_farm(db: Connection<FvDb>, id: i32) -> Value {
    json!({
        "user_id": -1,
        "user_nickname": "validation error"
    })
}