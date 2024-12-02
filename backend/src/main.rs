mod api;
mod database;
mod repository;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::routes::*;
use database::db;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let client_db = match db::create_database().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Database connection failed: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    let _server = HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;

    client_db.close();
    Ok(())
}
