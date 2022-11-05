use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use rocket::form::{FromForm};

// pub(crate) struct fv_user{
#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct fv_user{
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