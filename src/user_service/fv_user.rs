use chrono::{NaiveDateTime};
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
// use rocket_db_pools::sqlx::postgres::PgQueryResult;
use rocket_db_pools::sqlx::postgres::PgRow;
use crate::repository::FvDb;

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

    pub async fn insert(&self, mut db: Connection<FvDb>) -> Option<PgRow>{
        let query = format!("INSERT INTO fv_user(\
                    email_address, \
                    user_nickname, \
                    user_role, \
                    user_image, \
                    user_farm_id, \
                    access_token, \
                    access_expired, \
                    refresh_token, \
                    refresh_expired\
                )\
                VALUES('{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}')\
                RETURNING user_id, user_nickname",
            self.email_address,
            self.user_nickname,
            self.user_role,
            self.user_image,
            self.user_farm_id,
            self.access_token,
            self.access_expired,
            self.refresh_token,
            self.refresh_expired
        );

        return sqlx::query(&query)
            // .execute(&mut *db)
            .fetch_one(&mut *db)
            .await
            .ok()
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