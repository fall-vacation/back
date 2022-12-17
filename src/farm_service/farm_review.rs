use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveTime;
use sqlx::Row;
use crate::repository::FvDb;
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

impl Dao {
    pub fn to_dto(self) -> Dto {
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

impl Dto {
    pub fn to_dao(self) -> Dao {
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

    pub fn new() -> Dto {
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