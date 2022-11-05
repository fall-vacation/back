use serde::{Serialize, Deserialize};
use rocket::form::{FromForm};

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct FvUserDto {
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