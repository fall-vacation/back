#[macro_use] extern crate rocket;
use rocket_db_pools::Database;

use fall_vacation_back::repository::FvDb;
use fall_vacation_back::fv_user;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    println!("fall_vacation_back running...!");

    let _rocket = rocket::build()
        .attach(FvDb::init())
        .mount("/", routes![health_check])
        .attach(fv_user::stage())
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
pub fn health_check() -> &'static str {
    println!("check health...");
    "= alive ="
}