extern crate rocket;

pub mod farm;
pub mod farm_review;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;
use sqlx::Row;

use crate::repository::FvDb;
use crate::farm_service::farm::{Dto, Dao};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Farm Stage", |rocket| async {
        rocket
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/farm",
                   routes![
                       signup,
                       get_farm,
                   ])
    })
}

#[post("/signup", format = "json", data = "<farm>")]
pub async fn signup(db: Connection<FvDb>, farm: Json<Dto>) -> Value {
    let farm = farm.into_inner();
    let farm = farm.to_dao();
    return match farm.insert(db).await {
        Some(result) => {
            json!({
                "farm_id": result.get::<i32, _>("farm_id"),
                "farm_name": result.get::<String, _>("farm_name"),
            })
        },
        None => {
            json!({
                "farm_id": -1,
                "farm_name": "db insert error",
            })
        },
    }
}

#[get("/<id>")]
pub async fn get_farm(db: Connection<FvDb>, id: i32) -> Json<Dto> {
    match Dao::select_from_id(db, id).await {
        Some(result) => {
            let dto = Dao::match_pg_row(result).to_dto();
            Json(dto)
        }
        None => {
            Json(Dto::new())
        }
    }
}