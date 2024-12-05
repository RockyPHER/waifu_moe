use actix_web::{http::header::HttpDate, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::services::{animes_services, characters_services};

pub async fn get_animes(db: web::Data<DatabaseConnection>) -> impl Responder {
    match animes_services::get_animes_services(&**db).await {
        Ok(animes) => HttpResponse::Ok().json(animes),
        
        Err(_) =>{ 
        HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
pub async fn get_anime(path: web::Path<i64>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let path_id: i64 = path.into_inner();
    match animes_services::get_anime_services(path_id, &**db).await {
        Ok(anime) => HttpResponse::Ok().json(anime),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
pub async fn get_anime_character(path: web::Path<i64>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let path_id: i64 = path.into_inner();
    match animes_services::get_anime_character_service(path_id, &**db).await{
        Ok(characters) => HttpResponse::Ok().json(characters),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error")
    }
}
