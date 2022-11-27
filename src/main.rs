#[macro_use] extern crate rocket;
use rocket_db_pools::Database;

use fall_vacation_back::repository::FvDb;
use fall_vacation_back::user_service;
use fall_vacation_back::farm_service;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    println!("fall_vacation_back running...!");

    let _rocket = rocket::build()
        .attach(FvDb::init())
        .mount("/", routes![health_check])
        .attach(user_service::stage())
        .attach(farm_service::stage())
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
pub fn health_check() -> &'static str {
    println!("check health...");
    "= alive ="
}