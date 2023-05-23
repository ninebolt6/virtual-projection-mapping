use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::broadcast::Broadcaster;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/events").route(web::get().to(events)));
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../resources/index.html"))
}

async fn events(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster
        .register_client()
        .await
        .map_err(actix_web::error::ErrorNotAcceptable)
}
