use actix_web::web;

use crate::controllers::characters_controller;

pub fn init_characters_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/get_characters") // Base path
            .route("", web::get().to(characters_controller::get_characters)),
    );
}