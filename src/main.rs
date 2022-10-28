#[macro_use] extern crate rocket;

use fall_vacation_back::controller;
use fall_vacation_back::config;
use fall_vacation_back::repository;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    println!("fall_vacation_back running...!");

    let _rocket = rocket::build()
        .mount("/", routes![controller::index])
        .mount("/hello", routes![controller::print_world])
        .launch()
        .await?;
    Ok(())
}

// fn main() {
//     let profile = String::from("local");
//     let db_config = config::get_db_connection_config(&profile);
//     // println!("{}", &configs);
//     let mut db_manager = repository::DBManager::new(
//         &db_config
//     );
//
//     println!("db_manager create...");
//
//     let query = format!("\
//         SELECT user_id, email_address, user_role, user_image, access_token, access_expired, refresh_token, refresh_expired \
//         FROM fv_user;
//     ");
//
//     println!("query before...");
//
//     let result = db_manager.query(&query);
//
//     println!("query ok");
//
//     for row in result {
//         let data:i32 = row.get(0);
//         println!("data : {}", data);
//     }
// }