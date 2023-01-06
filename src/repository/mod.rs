pub mod query_to_string;
pub mod trait_dao_dto;

use rocket_db_pools::{sqlx, Database};
use sqlx::pool::PoolConnection;
use sqlx::{Error, Postgres};

use async_trait::async_trait;
use sqlx::postgres::PgRow;

#[derive(Database)]
#[database("postgres_fv")]
pub struct FvDb(sqlx::PgPool);

#[async_trait]
pub trait FallVacationDB {
    async fn query_one(&mut self, query: String) -> Result<PgRow, Error>;
    async fn query_all(&mut self, query: String) -> Result<Vec<PgRow>, Error>;
}

#[async_trait]
impl FallVacationDB for PoolConnection<Postgres> {
    async fn query_one(&mut self, query: String) -> Result<PgRow, Error> {
        sqlx::query(query.as_str())
            .fetch_one(self)
            .await
    }

    async fn query_all(&mut self, query: String) -> Result<Vec<PgRow>, Error> {
        sqlx::query(query.as_str())
            .fetch_all(self)
            .await
    }
}