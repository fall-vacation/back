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
                       review_signup,
                       get_farm_review,
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
            let dao = Dao::match_pg_row(&result);
            let farm_urls_query = farm_urls::Dao::urls_from_farm_id_query(dao.get_farm_id());
            let farm_urls : Vec<farm_urls::Dto>
                = match sqlx::query(farm_urls_query.as_str())
                .fetch_all(&mut *db)
                .await{
                Ok(result) => {
                    farm_urls::Dao::to_vec_dto(result)
                },
                Err(error) => {
                    println!("farm_urls_query error : {}", error);
                    Vec::new()
                }
            };
            let farm_review_query = farm_review::Dao::reviews_from_farm_id_query(dao.get_farm_id());
            let farm_reviews : Vec<farm_review::Dto>
                = match sqlx::query(farm_review_query.as_str())
                .fetch_all(&mut *db)
                .await{
                Ok(result) => {
                    farm_review::Dao::to_vec_dto(result)
                },
                Err(error) => {
                    println!("farm_review_query error : {}", error);
                    Vec::new()
                }
            };
            let dto = dao.to_dto_with_urls_reviews(farm_urls, farm_reviews);
            Json(dto)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            Json(Dto::new())
        }
    }
}

#[post("/review/signup", format = "json", data = "<farm_review>")]
pub async fn review_signup(mut db: Connection<FvDb>, farm_review: Json<farm_review::Dto>) -> Value {
    let farm_review = farm_review.into_inner();
    let farm_review = farm_review.to_dao();
    return match sqlx::query(farm_review.insert_query().as_str())
        .fetch_one(&mut *db)
        .await {
        Ok(result) => {
            json!({
                    "review_id": result.get::<i32, _>("review_id"),
                    "farm_id": result.get::<i32, _>("farm_id"),
                })
        },
        Err(error) => {
            json!({
                    "review_id": -1,
                    "error_message": error.to_string(),
                })
        },
    }
}


#[get("/review/<farm_id>")]
pub async fn get_farm_review(mut db: Connection<FvDb>, farm_id: i32) -> Json<Vec<farm_review::Dto>> {
    match sqlx::query(farm_review::Dao::reviews_from_farm_id_query(farm_id).as_str())
        .fetch_all(&mut *db)
        .await{
        Ok(result) => {
            let data = farm_review::Dao::to_vec_dto(result);
            Json(data)
        }
        Err(error) => {
            println!("error : {}", error.to_string());
            Json(Vec::new())
        }
    }
}