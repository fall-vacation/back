use rocket::serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::types::chrono::NaiveTime;
use crate::repository::trait_dao_dto::{DaoStruct, DtoStruct};
use crate::utils::{naive_time_to_string, string_to_naive_time};

#[derive(Debug)]
pub struct Dao{
    review_id: i32,
    farm_id: i32,
    user_id: i32,
    contents: Option<String>,
    hit: Option<i32>,
    stars: Option<i32>,
    create_time: Option<NaiveTime>,
    modify_time: Option<NaiveTime>,
    delete_time: Option<NaiveTime>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Dto{
    review_id: Option<i32>,
    farm_id: Option<i32>,
    user_id: Option<i32>,
    contents: Option<String>,
    hit: Option<i32>,
    stars: Option<i32>,
    create_time: Option<String>,
    modify_time: Option<String>,
    delete_time: Option<String>,
}

impl DaoStruct for Dao{
    type Dto = Dto;

    fn match_pg_row(row: &PgRow) -> Self {
        Dao{
            review_id : row.get::<i32, _>("review_id"),
            farm_id : row.get::<i32, _>("farm_id"),
            user_id : row.get::<i32, _>("user_id"),
            contents : row.get::<Option<String>, _>("contents"),
            hit : row.get::<Option<i32>, _>("hit"),
            stars : row.get::<Option<i32>, _>("stars"),
            create_time : row.get::<Option<NaiveTime>, _>("create_time"),
            modify_time : row.get::<Option<NaiveTime>, _>("modify_time"),
            delete_time : row.get::<Option<NaiveTime>, _>("delete_time"),
        }
    }

    fn to_dto(self) -> Self::Dto {
        Dto {
            review_id : Some(self.review_id),
            farm_id : Some(self.farm_id),
            user_id : Some(self.user_id),
            contents : self.contents,
            hit : self.hit,
            stars : self.stars,
            create_time : naive_time_to_string(&self.create_time),
            modify_time : naive_time_to_string(&self.modify_time),
            delete_time : naive_time_to_string(&self.delete_time),
        }
    }
}

impl DtoStruct for Dto {
    type Dao = Dao;

    fn to_dao(self) -> Self::Dao {
        Dao {
            review_id : self.review_id.unwrap_or(0),
            farm_id : self.farm_id.unwrap_or(0),
            user_id : self.user_id.unwrap_or(0),
            contents : self.contents,
            hit : self.hit,
            stars : self.stars,
            create_time : string_to_naive_time(&self.create_time),
            modify_time : string_to_naive_time(&self.modify_time),
            delete_time : string_to_naive_time(&self.delete_time),
        }
    }

    fn new() -> Self {
        Dto {
            review_id : None,
            farm_id : None,
            user_id : None,
            contents : None,
            hit : None,
            stars : None,
            create_time : None,
            modify_time : None,
            delete_time : None,
        }
    }
}