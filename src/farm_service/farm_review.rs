use rocket::serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use crate::repository::query_to_string::{naive_date_time_to_string, string_to_naive_date_time, ToQuery};
use crate::repository::trait_dao_dto::{DaoStruct, DtoStruct};

#[derive(Debug)]
pub struct Dao{
    review_id: i32,
    farm_id: i32,
    user_id: i32,
    contents: Option<String>,
    hit: i32,
    stars: i32,
    create_time: Option<NaiveDateTime>,
    modify_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
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
            hit : row.get::<i32, _>("hit"),
            stars : row.get::<i32, _>("stars"),
            create_time : row.get::<Option<NaiveDateTime>, _>("create_time"),
            modify_time : row.get::<Option<NaiveDateTime>, _>("modify_time"),
            delete_time : row.get::<Option<NaiveDateTime>, _>("delete_time"),
        }
    }

    fn to_dto(self) -> Self::Dto {
        Dto {
            review_id : Some(self.review_id),
            farm_id : Some(self.farm_id),
            user_id : Some(self.user_id),
            contents : self.contents,
            hit : Some(self.hit),
            stars : Some(self.stars),
            create_time : naive_date_time_to_string(&self.create_time),
            modify_time : naive_date_time_to_string(&self.modify_time),
            delete_time : naive_date_time_to_string(&self.delete_time),
        }
    }
}

impl Dao {
    pub fn reviews_from_farm_id_query(farm_id: i32) -> String {
        format!("SELECT \
            review_id,
            farm_id,
            user_id,
            contents,
            hit,
            stars,
            create_time,
            modify_time,
            delete_time
        FROM FARM_REVIEW \
        WHERE farm_id = {}", farm_id)
    }

    pub fn reviews_in_farm_id_query(farm_ids: &Vec<i32>) -> String {
        format!("SELECT \
            review_id,
            farm_id,
            user_id,
            contents,
            hit,
            stars,
            create_time,
            modify_time,
            delete_time
        FROM FARM_REVIEW \
        WHERE farm_id IN ({})", farm_ids.to_query_string())
    }

    pub fn insert_query(&self) -> String {
        format!("INSERT INTO FARM_REVIEW(\
            farm_id, \
            user_id, \
            contents, \
            hit, \
            stars, \
            create_time, \
            modify_time, \
            delete_time) \
        VALUES({}, {}, {}, {}, {}, {}, {}, {})\
        RETURNING review_id, farm_id",
        self.farm_id,
        self.user_id,
        self.contents.to_query_string(),
        self.hit,
        self.stars,
        self.create_time.to_query_string(),
        self.modify_time.to_query_string(),
        self.delete_time.to_query_string())
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
            hit : self.hit.unwrap_or(0),
            stars : self.stars.unwrap_or(0),
            create_time : string_to_naive_date_time(&self.create_time),
            modify_time : string_to_naive_date_time(&self.modify_time),
            delete_time : string_to_naive_date_time(&self.delete_time),
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

impl Dto {
    pub fn get_farm_id(&self) -> i32 {
        self.farm_id.unwrap_or(0)
    }
}