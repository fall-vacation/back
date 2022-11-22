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
                       // test
                   ])
    })
}

#[post("/signup", format = "json", data = "<login>")]
pub async fn signup(db: Connection<FvDb>, login: Json<Dto>) -> Value {
    let user = login.into_inner();
    if user.check_role_validation(){
        let user = user.to_dao();
        return match user.insert(db).await {
            Some(result) => {
                json!({
                    "user_id": result.get::<i32, _>("user_id"),
                    "user_nickname": result.get::<String, _>("user_nickname")
                })
            },
            None => {
                json!({
                    "user_id": -1,
                    "user_nickname": "db insert error"
                })
            },
        }
    }
    json!({
        "user_id": -1,
        "user_nickname": "validation error"
    })
}

#[get("/<id>")]
pub async fn get_member_by_id(db: Connection<FvDb>, id: i32) -> Json<Dto> {
    return match Dao::select_from_id(db, id).await{
        Some(result) => {
            let dto = Dao::match_pg_row(result).to_dto();
            Json(dto)
        },
        None => {
            Json(Dto::new())
        },
    }
}

// #[get("/test/insert")]
// pub async fn test(mut db: Connection<FvDb>) -> String {
//     sqlx::query("INSERT INTO user_service(email_address, user_nickname, user_role, user_image, user_farm_id, access_token, access_expired, refresh_token, refresh_expired)\
//                  VALUES('email', 'nickname', 'admin', 'aaa.jpg', 0, 'access', '2022-10-10', 'refresh', '2022-11-11')")
//         .execute(&mut *db)
//         .await
//         .ok();
//     return String::from("user_service list call")
// }
