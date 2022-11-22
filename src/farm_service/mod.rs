// extern crate rocket;
//
// use rocket::fairing::AdHoc;
// use rocket::{get, post, routes};
// use rocket::serde::json::{Json, Value};
// use rocket::serde::json::serde_json::json;
// use rocket_db_pools::Connection;
// use sqlx::Row;
//
// use crate::repository::FvDb;
//
// pub fn stage() -> AdHoc {
//     AdHoc::on_ignite("Farm Stage", |rocket| async {
//         rocket
//             // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
//             .mount("/farm",
//                    routes![
//                    ])
//     })
// }