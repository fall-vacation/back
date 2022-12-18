use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use crate::repository::FvDb;
use crate::repository::query_utils::ToQuery;
use crate::enums::user_role::UserRole;
use crate::utils::{naive_date_time_to_string, string_to_naive_date_time_default_now};

#[derive(Debug)]
pub struct Dao {
    user_id: i32,
    email_address : String,
    user_nickname: String,
    user_role: String,
    user_image: Option<String>,
    user_farm_id: Option<i32>,
    access_token: String,
    access_expired: NaiveDateTime,
    refresh_token: String,
    refresh_expired: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Dto {
    user_id: Option<i32>,
    email_address : Option<String>,
    user_nickname: Option<String>,
    user_role: UserRole,
    user_image: Option<String>,
    user_farm_id: Option<i32>,
    access_token: Option<String>,
    access_expired: Option<String>,
    refresh_token: Option<String>,
    refresh_expired: Option<String>,
}

impl Dao {
    pub fn to_dto(self) -> Dto {
        Dto {
            user_id: Some(self.user_id),
            email_address: Some(self.email_address),
            user_nickname: Some(self.user_nickname),
            user_role: UserRole::get_enums(&self.user_role),
            user_image: self.user_image,
            user_farm_id: self.user_farm_id,
            access_token: Some(self.access_token),
            access_expired: naive_date_time_to_string(&Some(self.access_expired)),
            refresh_token: Some(self.refresh_token),
            refresh_expired: naive_date_time_to_string(&Some(self.refresh_expired)),
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
                            self.user_image.to_query_string(),
                            self.user_farm_id.to_query_string(),
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

    pub async fn select_from_id(mut db: Connection<FvDb>, user_id: i32) -> Option<PgRow>{
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
            FROM fv_user \
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
            user_image : row.get::<Option<String>, _>("user_image"),
            user_farm_id : row.get::<Option<i32>, _>("user_farm_id"),
            access_token : row.get::<String, _>("access_token"),
            access_expired : row.get::<NaiveDateTime, _>("access_expired"),
            refresh_token : row.get::<String, _>("refresh_token"),
            refresh_expired : row.get::<NaiveDateTime, _>("refresh_expired"),
        }
    }

    pub async fn is_dup_email(mut db: Connection<FvDb>, email_address: &str) -> bool {
        let query = format!("\
            SELECT count(1)
            FROM fv_user
            WHERE email_address = {}", email_address
        );

        let data : Option<PgRow> = sqlx::query(&query)
            .fetch_one(&mut *db)
            .await
            .ok();

        match data {
            None  => { false },
            _some => { true  },
        }
    }
}

impl Dto {
    pub fn to_dao(self) -> Dao {
        Dao{
            user_id: self.user_id.unwrap_or(0),
            email_address: self.email_address.unwrap(),
            user_nickname: self.user_nickname.unwrap(),
            user_role: self.user_role.get_string(),
            user_image: self.user_image,
            user_farm_id: self.user_farm_id,
            access_token: self.access_token.unwrap(),
            access_expired: string_to_naive_date_time_default_now(&self.access_expired),
            refresh_token: self.refresh_token.unwrap(),
            refresh_expired: string_to_naive_date_time_default_now(&self.refresh_expired),
        }
    }

    pub fn new() -> Dto {
        return Dto{
            user_id: None,
            email_address: None,
            user_nickname: None,
            user_role: UserRole::NONE,
            user_image: None,
            user_farm_id: None,
            access_token: None,
            access_expired: None,
            refresh_token: None,
            refresh_expired: None,
        }
    }

    pub fn check_validation(&self) -> bool {
        self.email_address.is_some() &&
        self.user_nickname.is_some() &&
        self.access_token.is_some() &&
        self.access_expired.is_some() &&
        self.refresh_token.is_some() &&
        self.refresh_expired.is_some() &&
        self.user_role.validation()
        // !(Dao::is_dup_email(&db, &self.email_address)) &&
    }
}