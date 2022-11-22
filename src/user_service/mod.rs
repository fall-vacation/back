extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::Json;
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
                       login,
                       test,
                       get_member_by_id
                   ])
    })
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(db: Connection<FvDb>, login: Json<Dto>) -> String {
    let user = login.into_inner();
    let user = user.to_dao();
    match user.insert(db).await{
        Some(result) => {
            format!("{} - {}", result.get::<i32, _>("user_id"), result.get::<String, _>("user_nickname"))
        },
        None => { String::from("return none") },
    }
}

#[get("/<id>")]
pub async fn get_member_by_id(db: Connection<FvDb>, id: i32) -> Json<Dto> {
    // sqlx::query("SELECT content FROM logs WHERE id = ?").bind(id)
    //     .fetch_one(&db.0).await
    //     .and_then(|r| Ok(Log(r.try_get(0)?)))
    //     .ok();

    return match Dao::select(db, id).await{
        Some(result) => {
            let dto = Dao::match_pg_row(result).to_dto();
            Json(dto)
        },
        None => {
            // String::from("return none")
            Json(Dto::new())
        },
    }
}

#[get("/test/insert")]
pub async fn test(mut db: Connection<FvDb>) -> String {
    sqlx::query("INSERT INTO user_service(email_address, user_nickname, user_role, user_image, user_farm_id, access_token, access_expired, refresh_token, refresh_expired)\
                 VALUES('email', 'nickname', 'admin', 'aaa.jpg', 0, 'access', '2022-10-10', 'refresh', '2022-11-11')")
        .execute(&mut *db)
        .await
        .ok();
    return String::from("user_service list call")
}
