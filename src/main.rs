#[macro_use] extern crate rocket;

use fall_vacation_back::controller;

#[get("/")]
pub fn index() -> &'static str {
    println!("get root request...");
    "hello world"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    println!("fall_vacation_back running...!");

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![controller::print_world])
        .launch()
        .await?;
     Ok(())
}
