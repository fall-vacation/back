use rocket::serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::enums::farm_url_division::FarmUrlDivision;
use crate::repository::query_utils::ToQuery;

#[derive(Debug)]
pub struct Dao {
    farm_urls_id: i32,
    farm_id: i32,
    url_division: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Dto {
    farm_urls_id: Option<i32>,
    farm_id: Option<i32>,
    url_division: FarmUrlDivision,
    url: Option<String>,
}

impl Dao {
    pub fn to_dto(self) -> Dto {
        Dto {
            farm_urls_id: Some(self.farm_urls_id),
            farm_id: Some(self.farm_id),
            url_division: FarmUrlDivision::get_enum(&self.url_division),
            url: Some(self.url),
        }
    }

    pub fn insert_query(&self) -> String {
        format!("INSERT INTO farm_urls(\
        farm_id, url_division, url)\
        VALUES({}, {}, {})",
        self.farm_id,
        self.url_division.to_query_string(),
        self.url.to_query_string())
    }

    pub fn match_pg_row(row:PgRow) -> Dao {
        return Dao{
            farm_urls_id : row.get::<i32, _>("farm_urls_id"),
            farm_id : row.get::<i32, _>("farm_id"),
            url_division : row.get::<String, _>("url_division"),
            url : row.get::<String, _>("url"),
        }
    }

    pub fn urls_from_farm_id_query(farm_ids: i32) -> String {
        format!("SELECT \
            farm_urls_id \
            farm_id \
            url_division \
            url \
        FROM FARM_URLS \
        WHERE farm_id = {}", farm_ids)
    }

    pub fn urls_in_farm_id_query(farm_ids: &Vec<i32>) -> String {
        format!("SELECT \
            farm_urls_id \
            farm_id \
            url_division \
            url \
        FROM FARM_URLS \
        WHERE farm_id IN ({})", farm_ids.to_query_string())
    }
}

impl Dto {
    pub fn set_farm_id(&mut self, farm_id: i32) {
        self.farm_id = Some(farm_id);
    }

    pub fn to_dao(self) -> Dao {
        Dao {
            farm_urls_id: self.farm_urls_id.unwrap_or(0),
            farm_id: self.farm_id.unwrap_or(0),
            url_division: self.url_division.to_string(),
            url: self.url.unwrap()
        }
    }

    pub fn new() -> Dto {
        Dto {
            farm_urls_id: None,
            farm_id: None,
            url_division: FarmUrlDivision::ETC,
            url: None,
        }
    }
}