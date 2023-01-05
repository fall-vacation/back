use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveTime;
use sqlx::Row;
use crate::repository::query_to_string::{naive_time_to_string, string_to_naive_time, ToQuery};
use crate::farm_service::farm_urls;
use crate::farm_service::farm_review;
use crate::repository::trait_dao_dto::{DaoStruct, DtoStruct};

#[derive(Debug)]
pub struct Dao{
    farm_id: i32,
    farm_name: String,
    farm_address: String,
    farm_address_div: i32,
    farm_owner_name: Option<String>,
    farm_owner_phone: Option<String>,
    price: Option<String>,
    stars: f64,
    available_use_start: Option<NaiveTime>,
    available_use_end: Option<NaiveTime>,
    available_lesson: Option<bool>,
    etc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Dto {
    farm_id: Option<i32>,
    farm_name: Option<String>,
    farm_address: Option<String>,
    farm_address_div: Option<i32>,
    farm_owner_name: Option<String>,
    farm_owner_phone: Option<String>,
    price: Option<String>,
    stars: Option<f64>,
    available_use_start: Option<String>,
    available_use_end: Option<String>,
    available_lesson: Option<bool>,
    etc: Option<String>,

    farm_urls: Option<Vec<farm_urls::Dto>>,
    farm_reviews: Option<Vec<farm_review::Dto>>,
}

impl DaoStruct for Dao {
    type Dto = Dto;

    fn match_pg_row(row: &PgRow) -> Self {
        Dao{
            farm_id : row.get::<i32, _>("farm_id"),
            farm_name : row.get::<String, _>("farm_name"),
            farm_address : row.get::<String, _>("farm_address"),
            farm_address_div : row.get::<i32, _>("farm_address_div"),
            farm_owner_name : row.get::<Option<String>, _>("farm_owner_name"),
            farm_owner_phone : row.get::<Option<String>, _>("farm_owner_phone"),
            price : row.get::<Option<String>, _>("price"),
            stars : row.get::<f64, _>("stars"),
            available_use_start : row.get::<Option<NaiveTime>, _>("available_use_start"),
            available_use_end : row.get::<Option<NaiveTime>, _>("available_use_end"),
            available_lesson : row.get::<Option<bool>, _>("available_lesson"),
            etc : row.get::<Option<String>, _>("etc"),
        }
    }

    fn to_dto(self) -> Self::Dto {
        Dto {
            farm_id: Some(self.farm_id),
            farm_name: Some(self.farm_name),
            farm_address: Some(self.farm_address),
            farm_address_div: Some(self.farm_address_div),
            farm_owner_name: self.farm_owner_name,
            farm_owner_phone: self.farm_owner_phone,
            price: self.price,
            stars: Some(self.stars),
            available_use_start: naive_time_to_string(&self.available_use_start),
            available_use_end: naive_time_to_string(&self.available_use_end),
            available_lesson: self.available_lesson,
            etc: self.etc,
            farm_urls: None,
            farm_reviews: None,
        }
    }
}

impl Dao {
    // pub fn to_dto_with_urls(self, farm_urls: Vec<farm_urls::Dto>) -> Dto {
    //     let mut dto = self.to_dto();
    //     dto.set_farm_urls(farm_urls);
    //     dto
    // }

    pub fn to_dto_with_urls_reviews(
        self,
        farm_urls: Option<Vec<farm_urls::Dto>>,
        farm_review: Option<Vec<farm_review::Dto>>,
    ) -> Dto {
        let mut dto = self.to_dto();
        dto .set_farm_urls(farm_urls)
            .set_farm_reviews(farm_review);
        dto
    }

    pub fn get_farm_id(&self) -> i32 {
        self.farm_id
    }

    pub fn insert_query(&self) -> String {
        format!("\
            INSERT INTO farm(\
                farm_name, \
                farm_address, \
                farm_address_div, \
                farm_owner_name, \
                farm_owner_phone, \
                price, \
                stars, \
                available_use_start, \
                available_use_end, \
                available_lesson, \
                etc \
            )\
            VALUES({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})\
            RETURNING farm_id, farm_name",
                            self.farm_name.to_query_string(),
                            self.farm_address.to_query_string(),
                            self.farm_address_div,
                            self.farm_owner_name.to_query_string(),
                            self.farm_owner_phone.to_query_string(),
                            self.price.to_query_string(),
                            self.stars,
                            self.available_use_start.to_query_string(),
                            self.available_use_end.to_query_string(),
                            self.available_lesson.to_query_string(),
                            self.etc.to_query_string(),
        )
    }

    pub fn select_from_id_query(farm_id: i32) -> String {
        format!("\
            SELECT \
                farm_id, \
                farm_name, \
                farm_address, \
                farm_address_div, \
                farm_owner_name, \
                farm_owner_phone, \
                price, \
                stars, \
                available_use_start, \
                available_use_end, \
                available_lesson, \
                etc \
            FROM farm \
            WHERE farm_id = {}", farm_id)
    }
}

impl DtoStruct for Dto {
    type Dao = Dao;

    fn to_dao(self) -> Self::Dao {
        Dao {
            farm_id: self.farm_id.unwrap_or(0),
            farm_name: self.farm_name.unwrap_or("".to_string()),
            farm_address: self.farm_address.unwrap_or("".to_string()),
            farm_address_div: self.farm_address_div.unwrap_or(0),
            farm_owner_name: self.farm_owner_name,
            farm_owner_phone: self.farm_owner_phone,
            price: self.price,
            stars: self.stars.unwrap_or(0.0),
            available_use_start: string_to_naive_time(&self.available_use_start),
            available_use_end: string_to_naive_time(&self.available_use_end),
            available_lesson: self.available_lesson,
            etc: self.etc,
        }
    }

    fn new() -> Self {
        Dto {
            farm_id: None,
            farm_name: None,
            farm_address: None,
            farm_address_div: None,
            farm_owner_name: None,
            farm_owner_phone: None,
            price: None,
            stars: None,
            available_use_start: None,
            available_use_end: None,
            available_lesson: None,
            etc: None,
            farm_urls: None,
            farm_reviews: None,
        }
    }
}

impl Dto {
    pub fn get_dao_and_urls(mut self) -> (Dao, Option<Vec<farm_urls::Dto>>) {
        let farm_urls = self.farm_urls;
        self.farm_urls = None;
        let dao = self.to_dao();
        return (dao, farm_urls)
    }

    pub fn set_farm_urls(&mut self, farm_urls: Option<Vec<farm_urls::Dto>>) -> &mut Dto {
        self.farm_urls = farm_urls;
        self
    }

    pub fn set_farm_reviews(&mut self, farm_review: Option<Vec<farm_review::Dto>>) -> &mut Dto {
        self.farm_reviews = farm_review;
        self
    }

    pub fn get_id(&self) -> Option<i32>{
        self.farm_id
    }

    // pub async fn set_urls_review(db: &mut Query<'_, FvDb, None>, dtos: &mut Vec<Dto>) {
    //     let mut ids = Vec::new();
    //     for each in dtos {
    //         ids.push(each.get_id().unwrap_or(0));
    //     }
    //
    //     match sqlx::query(farm_urls::Dao::urls_in_farm_id_query(&ids).as_str())
    //         .fetch_all(&*db)
    //         .await {
    //         Ok(_result) => {},
    //         Err(_error) => {}
    //     }
    // }
}