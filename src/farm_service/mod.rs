extern crate rocket;

pub mod farm;
pub mod farm_urls;
pub mod farm_review;
mod request_form;

use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Connection;
use sqlx::Row;

use crate::repository::{FallVacationDB, FvDb};
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
                       get_farm_list,
                       review_signup,
                       get_farm_review,

                       get_farm_ver2,
                       get_list_farm_ver2,
                   ])
    })
}

#[post("/signup", format = "json", data = "<farm>")]
pub async fn signup(mut db: Connection<FvDb>, farm: Json<Dto>) -> Value {
    let farm = farm.into_inner();
    let (farm, farm_urls)= farm.get_dao_and_urls();

    return match sqlx::query(farm.insert_query().as_str())
        .fetch_one(&mut *db)
        .await {
            Ok(result) => {
                let farm_id = result.get::<i32, _>("farm_id");

                if farm_urls.is_some() {
                    let farm_urls = farm_urls.unwrap();
                    for mut each in farm_urls{
                        each.set_farm_id(farm_id);
                        let each = each.to_dao();
                        sqlx::query(each.insert_query().as_str() )
                            .fetch_one(&mut *db)
                            .await
                            .ok();
                    }
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

#[get("/ver2/<id>")]
pub async fn get_farm_ver2(mut db: Connection<FvDb>, id:i32) -> Json<Dto>{
    match db.fetch_one(Dao::select_from_id_query(id))
        .await {
        Ok(result) => {
            let dto = Dao::match_pg_row(&result).to_dto();
            Json(dto)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            Json(Dto::new())
        }
    }
}

#[get("/ver2/list?<param..>")]
pub async fn get_list_farm_ver2(mut db: Connection<FvDb>, param: request_form::ListForm) -> Json<Vec<Dto>> {
    match db.fetch_all(param.get_list_query())
        .await {
        Ok(result) => {
            let data = Dao::to_vec_dto(result);
            Json(data)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            let data = Vec::new();
            Json(data)
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
            let farm_urls = match sqlx::query(farm_urls_query.as_str())
                .fetch_all(&mut *db)
                .await{
                Ok(result) => {
                    Some(farm_urls::Dao::to_vec_dto(result))
                },
                Err(error) => {
                    println!("farm_urls_query error : {}", error);
                    None
                }
            };
            let farm_review_query = farm_review::Dao::reviews_from_farm_id_query(dao.get_farm_id());
            let farm_reviews = match sqlx::query(farm_review_query.as_str())
                .fetch_all(&mut *db)
                .await{
                Ok(result) => {
                    Some(farm_review::Dao::to_vec_dto(result))
                },
                Err(error) => {
                    println!("farm_review_query error : {}", error);
                    None
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

#[get("/list?<param..>")]
pub async fn get_farm_list(mut db: Connection<FvDb>, param: request_form::ListForm) -> Json<Vec<Dto>> {
    match sqlx::query(param.get_list_query().as_str())
        .fetch_all(&mut *db)
        .await {
        Ok(result) => {
            let data = Dao::to_vec_dto(result);
            Json(data)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            let data = Vec::new();
            Json(data)
        }
    }
}


// REVIEWS ==========================================================================================
// ==================================================================================================
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