mod controllers;
mod repository;
mod routes;
mod services;

mod database;
mod model;
use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};
use database::db;
use routes::{routes_animes::init_anime_routes, routes_characters::init_characters_routes};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_client = db::get_client_db().await.unwrap();
    let _server = HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
            .max_age(3600)
        )
            .app_data(web::Data::new(db_client.clone()))
            .configure(init_anime_routes)
            .configure(init_characters_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
