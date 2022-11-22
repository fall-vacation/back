use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use crate::repository::FvDb;
use crate::enums::user_role::UserRole;

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
    user_id: Option<i32>,
    email_address : String,
    user_nickname: String,
    user_role: UserRole,
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
            user_id: Option::from(self.user_id),
            email_address: self.email_address.clone(),
            user_nickname: self.user_nickname.clone(),
            user_role: UserRole::get_enums(&self.user_role),
            user_image: self.user_image.clone(),
            user_farm_id: self.user_farm_id.clone(),
            access_token: self.access_token.clone(),
            access_expired: self.access_expired.clone().to_string(),
            refresh_token: self.refresh_token.clone(),
            refresh_expired: self.refresh_expired.clone().to_string(),
        }
    }

    pub async fn insert(&self, mut db: Connection<FvDb>) -> Option<PgRow>{
        let query = format!("\
            INSERT INTO fv_user(\
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

    pub async fn select(mut db: Connection<FvDb>, user_id: i32) -> Option<PgRow>{
        let query = format!("\
            SELECT \
                user_id, \
                email_address, \
                user_nickname, \
                user_role, \
                user_image, \
                user_farm_id, \
                access_token, \
                access_expired, \
                refresh_token, \
                refresh_expired \
            FROM fv_user
            WHERE user_id = {}", user_id
        );

        return sqlx::query(&query)
            .fetch_one(&mut *db)
            .await
            .ok()
    }

    pub fn match_pg_row(row:PgRow) -> Dao {
        return Dao{
            user_id : row.get::<i32, _>("user_id"),
            email_address : row.get::<String, _>("email_address"),
            user_nickname : row.get::<String, _>("user_nickname"),
            user_role : row.get::<String, _>("user_role"),
            user_image : row.get::<String, _>("user_image"),
            user_farm_id : row.get::<i32, _>("user_farm_id"),
            access_token : row.get::<String, _>("access_token"),
            access_expired : row.get::<NaiveDateTime, _>("access_expired"),
            refresh_token : row.get::<String, _>("refresh_token"),
            refresh_expired : row.get::<NaiveDateTime, _>("refresh_expired"),
        }
    }
}

impl Dto {
    pub fn to_dao(&self) -> Dao {
        Dao{
            user_id: self.user_id.unwrap_or(0),
            email_address: self.email_address.clone(),
            user_nickname: self.user_nickname.clone(),
            user_role: self.user_role.get_string(),
            user_image: self.user_image.clone(),
            user_farm_id: self.user_farm_id.clone(),
            access_token: self.access_token.clone(),
            access_expired: NaiveDateTime::parse_from_str(&*self.access_expired, "%Y-%m-%d %H:%M:%S").unwrap(),
            refresh_token: self.refresh_token.clone(),
            refresh_expired: NaiveDateTime::parse_from_str(&*self.refresh_expired, "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }

    pub fn new() -> Dto {
        return Dto{
            user_id: None,
            email_address: "".to_string(),
            user_nickname: "".to_string(),
            user_role: UserRole::NONE,
            user_image: "".to_string(),
            user_farm_id: -1,
            access_token: "".to_string(),
            access_expired: "".to_string(),
            refresh_token: "".to_string(),
            refresh_expired: "".to_string()
        }
    }

    pub fn check_role_validation(&self) -> bool {
        return self.user_role.validation();
    }
}