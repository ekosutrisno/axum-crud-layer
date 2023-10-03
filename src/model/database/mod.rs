use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Database = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Database, sqlx::Error> {
    let max_connections = if cfg!(test) { 1 } else { 5 };
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
}
