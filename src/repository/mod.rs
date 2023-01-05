pub mod query_to_string;
pub mod trait_dao_dto;

use rocket_db_pools::{sqlx, Database};

// use async_trait::async_trait;
// use rocket_db_pools::Connection;
// use sqlx::postgres::PgRow;
// use sqlx::Executor;

#[derive(Database)]
#[database("postgres_fv")]
pub struct FvDb(sqlx::PgPool);

// pub struct DBController{
//     is_init: bool,
//     connection: Option<Connection<FvDb>>
// }
//
// impl DBController{
//     pub fn new() -> DBController {
//         DBController{
//             is_init: false,
//             connection: None
//         }
//     }
//     pub fn init(&mut self, connection: Connection<FvDb>) {
//         self.is_init = true;
//         self.connection = Some(connection);
//     }
//
//     async fn query_one(&mut self, query: String) -> Option<PgRow> {
//         sqlx::query(query.as_str())
//             .fetch_one(self.connection.unwrap())
//             .await
//             .ok()
//     }
// }