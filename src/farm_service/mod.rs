extern crate rocket;

pub mod farm;
pub mod farm_urls;
pub mod farm_review;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;
use sqlx::Row;

use crate::repository::FvDb;
use crate::farm_service::farm::{Dto, Dao};
use crate::repository::trait_dao_dto::{DaoStruct, DtoStruct};

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
pub async fn signup(mut db: Connection<FvDb>, farm: Json<Dto>) -> Value {
    let farm = farm.into_inner();
    let farm_urls = farm.get_farm_urls_clone();

    let farm = farm.to_dao();
    return match sqlx::query(farm.insert_query().as_str())
        .fetch_one(&mut *db)
        .await {
            Ok(result) => {
                let farm_id = result.get::<i32, _>("farm_id");

                for mut each in farm_urls{
                    each.set_farm_id(farm_id);
                    let each = each.to_dao();
                    sqlx::query(each.insert_query().as_str() )
                        .fetch_one(&mut *db)
                        .await
                        .ok();
                }

                json!({
                    "farm_id": farm_id,
                    "farm_name": result.get::<String, _>("farm_name"),
                })
            },
            Err(error) => {
                json!({
                    "farm_id": -1,
                    "error_message": error.to_string(),
                })
            },
    }
}

#[get("/<id>")]
pub async fn get_farm(mut db: Connection<FvDb>, id: i32) -> Json<Dto> {
    match sqlx::query(Dao::select_from_id_query(id).as_str())
        .fetch_one(&mut *db)
        .await {
        Ok(result) => {
            let dao = Dao::match_pg_row(result);
            let farm_urls_query = farm_urls::Dao::urls_from_farm_id_query(dao.get_farm_id());
            let farm_urls : Vec<farm_urls::Dto>
                = match sqlx::query(farm_urls_query.as_str())
                .fetch_all(&mut *db)
                .await{
                Ok(result) => {
                    farm_urls::Dao::to_vec_dto(result)
                },
                Err(error) => {
                    println!("error : {}", error);
                    Vec::new()
                }
            };
            let dto = dao.to_dto_with_urls(farm_urls);
            Json(dto)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            Json(Dto::new())
        }
    }
}