use actix_web::web;

use crate::controllers::animes_controller;

pub fn init_anime_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/get_animes").route(
        "",
        web::get().to(animes_controller::get_animes),
    ));
}
