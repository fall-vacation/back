use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("postgres_fv")]
pub struct FvDb(sqlx::PgPool);