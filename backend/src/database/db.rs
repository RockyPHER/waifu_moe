use std::env;

use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};


pub async fn create_database() -> Result<DatabaseConnection, DbErr>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set on .env file");
    let mut opt = ConnectOptions::new(&database_url);
    opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(30))
        .sqlx_logging(true);
    let db = Database::connect(opt).await;
    return db;
}