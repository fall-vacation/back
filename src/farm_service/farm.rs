use std::fmt::Display;
use rocket::log::private::log;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveTime;
use sqlx::Row;
use crate::repository::FvDb;

#[derive(Debug)]
pub struct Dao{
    farm_id: i32,
    farm_name: String,
    farm_address: String,
    farm_address_div: i32,
    farm_owner_name: String,
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
    farm_name: String,
    farm_address: String,
    farm_address_div: i32,
    farm_owner_name: String,
    farm_owner_phone: Option<String>,
    price: Option<String>,
    stars: f64,
    available_use_start: Option<String>,
    available_use_end: Option<String>,
    available_lesson: Option<bool>,
    etc: Option<String>,
}

impl Dao {
    pub fn toDto(&self) -> Dto {
        return Dto {
            farm_id: Some(self.farm_id),
            farm_name: self.farm_name.clone(),
            farm_address: self.farm_address.clone(),
            farm_address_div: self.farm_address_div,
            farm_owner_name: self.farm_owner_name.clone(),
            farm_owner_phone: self.farm_owner_phone.clone(),
            price: self.price.clone(),
            stars: self.stars,
            available_use_start: naive_time_to_string(&self.available_use_start),
            available_use_end: naive_time_to_string(&self.available_use_end),
            available_lesson: self.available_lesson,
            etc: self.etc.clone(),
        }
    }

    pub async fn insert(&self, mut db: Connection<FvDb>) -> Option<PgRow>{
        let query = format!("\
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
            VALUES('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}')\
            RETURNING farm_id, farm_name",
                            self.farm_name,
                            self.farm_address,
                            self.farm_address_div,
                            self.farm_owner_name,
                            to_query(&self.farm_owner_phone),
                            to_query(&self.price),
                            self.stars,
                            to_query(&self.available_use_start),
                            to_query(&self.available_use_end),
                            to_query(&self.available_lesson),
                            to_query(&self.etc),
        );

        return sqlx::query(&query)
            // .execute(&mut *db)
            .fetch_one(&mut *db)
            .await
            .ok()
    }

}

impl Dto {
    pub fn new() -> Dto {
        return Dto {
            farm_id: None,
            farm_name: "".to_string(),
            farm_address: "".to_string(),
            farm_address_div: 0,
            farm_owner_name: "".to_string(),
            farm_owner_phone: None,
            price: None,
            stars: 0.0,
            available_use_start: None,
            available_use_end: None,
            available_lesson: None,
            etc: None,
        }
    }

    pub fn to_dao(&self) -> Dao {
        return Dao {
            farm_id: self.farm_id.unwrap_or(0),
            farm_name: self.farm_name.clone(),
            farm_address: self.farm_address.clone(),
            farm_address_div: self.farm_address_div,
            farm_owner_name: self.farm_owner_name.clone(),
            farm_owner_phone: self.farm_owner_phone.clone(),
            price: self.price.clone(),
            stars: self.stars,
            available_use_start: string_to_naive_time(&self.available_use_start),
            available_use_end: string_to_naive_time(&self.available_use_end),
            available_lesson: self.available_lesson,
            etc: self.etc.clone(),
        }
    }
}

fn string_to_naive_time(opt_data:&Option<String>) -> Option<NaiveTime> {
    return match opt_data {
        Some(data) => {
            match NaiveTime::parse_from_str(data, "%H:%M:%S") {
                Ok(naive) => { Some(naive) },
                Err(e) => {
                    println!("[LOG] string_to_naive_time : {}", e.to_string());
                    None
                }
            }
        },
        None => { None },
    };
}

fn naive_time_to_string(opt_naive:&Option<NaiveTime>) -> Option<String> {
    return match opt_naive {
        Some(naive) => {
            Some(naive.format("%H:%M:%S").to_string())
        },
        None => { None },
    };
}

fn to_query<T:Display>(data: &Option<T>) -> String {
    return match data {
        Some(data) => {
            format!("'{}'", data)
        },
        None => "NULL".to_string(),
    };
}