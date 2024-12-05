use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::services::{animes_services, characters_services};

pub async fn get_characters(db: web::Data<DatabaseConnection>) -> impl Responder {
    match characters_services::get_characters_service(&**db).await {
        Ok(characters) => HttpResponse::Ok().json(characters),
        
        Err(_) => {
            HttpResponse::InternalServerError().body("Internal Server Error")
        },
    }
}
// pub async fn get_character_from_anime(path: web::Path<i64>, db: web::Data<DatabaseConnection>) -> impl Responder {
//     let path_id: i64 = path.into_inner();
//     match animes_services::get_anime_services(path_id, &**db).await {
//         Ok(anime) => HttpResponse::Ok().json(anime),
//         Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
//     }
// }
