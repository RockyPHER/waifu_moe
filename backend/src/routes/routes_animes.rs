use actix_web::web;

use crate::controllers::animes_controller;

pub fn init_anime_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/animes")
            .route("", web::get().to(animes_controller::get_animes))
            .route("/{id}", web::get().to(animes_controller::get_anime)),
    );
}
