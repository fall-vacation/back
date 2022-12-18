use rocket::serde::{Serialize, Deserialize};
use crate::enums::farm_url_division::FarmUrlDivision;

#[derive(Debug)]
pub struct Dao {
    farm_urls_id: i32,
    farm_id: i32,
    url_division: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
}

impl Dto {
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