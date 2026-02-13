use crate::config::database::database_configuration::DatabaseConfiguration;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

pub async fn create_pool(config: &DatabaseConfiguration) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(config.url())?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections())
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
