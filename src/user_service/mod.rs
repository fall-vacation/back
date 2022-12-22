extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;
use sqlx::Row;

use crate::repository::FvDb;
pub mod fv_user;
use fv_user::Dto;
use crate::user_service::fv_user::Dao;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Member Stage", |rocket| async {
        rocket
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/user",
                   routes![
                       signup,
                       get_member_by_id,
                   ])
    })
}

#[post("/signup", format = "json", data = "<login>")]
pub async fn signup(mut db: Connection<FvDb>, login: Json<Dto>) -> Value {
    let user = login.into_inner();
    if user.check_validation(){
        let user = user.to_dao();

        match sqlx::query(user.is_dup_email_query().as_str())
            .fetch_one(&mut *db)
            .await {
            Ok(result) => {
                let count = result.get::<i32, _>("count");
                if count > 0 {
                    return json!({
                        "user_id": -1,
                        "error_message": "duplicate email address",
                    })
                }
            },
            Err(error) => {
                return json!({
                    "user_id": -1,
                    "error_message": error.to_string(),
                })
            }
        }

        return match sqlx::query(user.insert_query().as_str())
            .fetch_one(&mut *db)
            .await{
            Ok(result) => {
                json!({
                    "user_id": result.get::<i32, _>("user_id"),
                    "user_nickname": result.get::<String, _>("user_nickname"),
                })
            },
            Err(error) => {
                json!({
                    "user_id": -1,
                    "error_message": error.to_string(),
                })
            }
        }
    }
    json!({
        "user_id": -1,
        "error_message": "validation error"
    })
}

#[get("/<id>")]
pub async fn get_member_by_id(mut db: Connection<FvDb>, id: i32) -> Json<Dto> {
    match sqlx::query( Dao::select_from_id_query(id).as_str() )
        .fetch_one(&mut *db)
        .await {
        Ok(result) => {
            let dto = Dao::match_pg_row(result).to_dto();
            Json(dto)
        },
        Err(_) => {
            Json(Dto::new())
        },
    }
}