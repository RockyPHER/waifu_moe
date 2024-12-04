use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::services::characters_services;


pub async fn get_characters(db: web::Data<DatabaseConnection>) -> impl Responder {
    print!("hello");
    match characters_services::get_characters_service(&**db).await {
        Ok(characters) => {
            HttpResponse::Ok().json(characters)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
    
}