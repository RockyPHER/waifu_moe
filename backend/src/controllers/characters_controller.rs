use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::services::characters_services;


pub async fn get_characters(db: web::Data<DatabaseConnection>) -> impl Responder {
    print!("hello");
    match characters_services::get_characters_service(&**db).await {
        Ok(characters) => {
            log::info!("Successfully fetched {} characters.", characters.len());
            HttpResponse::Ok().json(characters)
        }
        Err(err) => {
            log::error!("Failed to fetch characters: {:?}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
    
}