use actix_web::{get, http::uri::Authority, post, HttpResponse, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("AM 109")
}
#[get("/animes")]
async fn get_animes() -> impl Responder{
}
