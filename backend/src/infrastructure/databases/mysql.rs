use anyhow::{format_err, Result};
use sea_orm::{Database, DatabaseConnection};

pub async fn create_connection() -> Result<DatabaseConnection> {
    let opt = format!(
        "mysql://{}:{}@{}:{}/{}",
        std::env::var("MYSQL_USER").unwrap(),
        std::env::var("MYSQL_PASSWORD").unwrap(),
        std::env::var("MYSQL_HOST").unwrap(),
        std::env::var("MYSQL_PORT").unwrap(),
        std::env::var("MYSQL_DATABASE").unwrap(),
    );
    Database::connect(opt).await.map_err(|e| format_err!(e))
}
