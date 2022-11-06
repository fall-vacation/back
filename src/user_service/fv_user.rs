use chrono::{NaiveDateTime};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Dao {
    user_id: i32,
    email_address : String,
    user_nickname: String,
    user_role: String,
    user_image: String,
    user_farm_id: i32,
    access_token: String,
    access_expired: NaiveDateTime,
    refresh_token: String,
    refresh_expired: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Dto {
    user_id: i32,
    email_address : String,
    user_nickname: String,
    user_role: String,
    user_image: String,
    user_farm_id: i32,
    access_token: String,
    access_expired: String,
    refresh_token: String,
    refresh_expired: String,
}

impl Dao {
    pub fn to_dto(&self) -> Dto {
        Dto {
            user_id: self.user_id.clone(),
            email_address: self.email_address.clone(),
            user_nickname: self.user_nickname.clone(),
            user_role: self.user_role.clone(),
            user_image: self.user_image.clone(),
            user_farm_id: self.user_farm_id.clone(),
            access_token: self.access_token.clone(),
            access_expired: self.access_expired.clone().to_string(),
            refresh_token: self.refresh_token.clone(),
            refresh_expired: self.refresh_expired.clone().to_string(),
        }
    }
}

impl Dto {
    pub fn to_dao(&self) -> Dao {
        Dao{
            user_id: self.user_id,
            email_address: self.email_address.clone(),
            user_nickname: self.user_nickname.clone(),
            user_role: self.user_role.clone(),
            user_image: self.user_image.clone(),
            user_farm_id: self.user_farm_id.clone(),
            access_token: self.access_token.clone(),
            access_expired: NaiveDateTime::parse_from_str(&*self.access_expired, "%Y-%m-%d %H:%M:%S").unwrap(),
            refresh_token: self.refresh_token.clone(),
            refresh_expired: NaiveDateTime::parse_from_str(&*self.refresh_expired, "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }
}