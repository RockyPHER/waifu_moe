use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::services::animes_services;

pub async fn get_animes(db: web::Data<DatabaseConnection>) -> impl Responder {
    print!("hello");
    match animes_services::get_animes_services(&**db).await {
        Ok(animes) => {
            HttpResponse::Ok().json(animes)
        }
        Err(err) => {
            log::error!("Failed to fetch characters: {:?}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
