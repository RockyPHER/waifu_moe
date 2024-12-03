mod routes;
mod controllers;
mod services;
mod repository;

mod database;
mod model;



use actix_web::{web, App, HttpServer};
use database::db;
use routes::{routes_animes::init_anime_routes, routes_characters::init_characters_routes};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_client = db::get_client_db().await.unwrap();
    let _server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .configure(init_anime_routes)
            .configure(init_characters_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
