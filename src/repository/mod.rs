pub mod query_utils;

// use async_trait::async_trait;
// use rocket_db_pools::Connection;
// use sqlx::postgres::PgRow;
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("postgres_fv")]
pub struct FvDb(sqlx::PgPool);

// #[async_trait]
// pub trait FallVacationDB {
//     async fn select_from_id(&mut self, query: String) -> Option<PgRow>;
// }
//
// #[async_trait]
// impl FallVacationDB for Connection<FvDb> {
//     async fn select_from_id(&mut self, query: String) -> Option<PgRow> {
//         sqlx::query(query.as_str())
//             .fetch_one(self)
//             .await
//             .ok()
//     }
// }