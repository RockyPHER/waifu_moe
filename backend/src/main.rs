mod api;
mod database;
mod repository;
mod migration;

use actix_web::{App, HttpServer};
use api::routes::*;
use database::db;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_client = db::get_client_db().await.unwrap();
    let _server = HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    api::routes::get_animes
    db_client.close();
    Ok(())
}