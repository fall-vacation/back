extern crate rocket;

mod farm;
mod farm_urls;
mod farm_review;
mod request_form;

use itertools::Itertools;
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
                   ])
    })
}

// FARM ============================================================================================
// =================================================================================================
#[post("/signup", format = "json", data = "<farm>")]
pub async fn signup(mut db: Connection<FvDb>, farm: Json<Dto>) -> Value {
    let farm = farm.into_inner();
    let (farm, farm_urls)= farm.get_dao_and_urls();

    return match db.query_one(farm.insert_query())
        .await {
            Ok(result) => {
                let farm_id = result.get::<i32, _>("farm_id");

                if farm_urls.is_some() {
                    for mut each in farm_urls.unwrap() {
                        each.set_farm_id(farm_id);
                        let each = each.to_dao();
                        db.query_one(each.insert_query())
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

#[get("/<id>")]
pub async fn get_farm(mut db: Connection<FvDb>, id:i32) -> Json<Dto>{
    match db.query_one(Dao::select_from_id_query(id))
        .await {
        Ok(result) => {
            let dao = Dao::match_pg_row(&result);
            let farm_urls_query = farm_urls::Dao::urls_from_farm_id_query(dao.get_farm_id());
            let farm_urls = match db.query_all(farm_urls_query)
                .await {
                Ok(result) => {
                    Some(farm_urls::Dao::to_vec_dto(result))
                },
                Err(error) => {
                    println!("farm_urls_query error : {}", error);
                    None
                },
            };
            let farm_review_query = farm_review::Dao::reviews_from_farm_id_query(dao.get_farm_id());
            let farm_reviews = match db.query_all(farm_review_query)
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
    match db.query_all(param.get_farm_list_query())
        .await {
        Ok(result) => {
            let dtos = Dao::to_vec_dto(result);
            let ids: Vec<i32> = dtos.iter().map(|dto| dto.get_farm_id()).collect();

            let data = merge_farm_data(
                dtos,
                farm_urls::Dao::to_vec_dto(
                    db.query_all(farm_urls::Dao::urls_in_farm_id_query(&ids))
                        .await.unwrap_or(Vec::new())
                ),
                farm_review::Dao::to_vec_dto(
                    db.query_all(farm_review::Dao::reviews_in_farm_id_query(&ids))
                        .await.unwrap_or(Vec::new())
                )
            );

            Json(data)
        },
        Err(error) => {
            println!("error : {}", error.to_string());
            let data = Vec::new();
            Json(data)
        },
    }
}

fn merge_farm_data(farm: Vec<Dto>, farm_urls: Vec<farm_urls::Dto>, farm_review: Vec<farm_review::Dto>) -> Vec<Dto> {
    let mut urls = farm_urls.into_iter()
        .into_group_map_by(|url| url.get_farm_id());
    let mut reviews = farm_review.into_iter()
        .into_group_map_by(|review| review.get_farm_id());

    farm.into_iter().map( |mut each_farm| {
        let farm_id = each_farm.get_farm_id();
        each_farm.set_farm_urls(urls.remove(&farm_id));
        each_farm.set_farm_reviews(reviews.remove(&farm_id));
        each_farm
    }).collect()
}

// REVIEWS =========================================================================================
// =================================================================================================
#[post("/review/signup", format = "json", data = "<farm_review>")]
pub async fn review_signup(mut db: Connection<FvDb>, farm_review: Json<farm_review::Dto>) -> Value {
    let farm_review = farm_review.into_inner();
    let farm_review = farm_review.to_dao();
    return match db.query_one(farm_review.insert_query())
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


#[get("/review/<farm_id>?<param..>")]
pub async fn get_farm_review(mut db: Connection<FvDb>, farm_id: i32, param: request_form::ListForm) -> Json<Vec<farm_review::Dto>> {
    match db.query_all(param.get_farm_review_list_query(farm_id))
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