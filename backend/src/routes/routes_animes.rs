use actix_web::web::{self, route};

use crate::controllers::animes_controller;

pub fn init_anime_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/animes")
            .route("", web::get().to(animes_controller::get_animes))
            .route("/{id}", web::get().to(animes_controller::get_anime))
            .route("/{id}/characters", web::get().to(animes_controller::get_anime_character))
            .route("/{id}/characters/{characters_name}", web::patch().to(animes_controller::patch_character_likes)),
    );
}
