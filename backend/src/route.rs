use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::broadcast::Broadcaster;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/events").route(web::get().to(events)))
        .service(web::resource("/broadcast").route(web::post().to(broadcast)));
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

async fn broadcast(
    body: web::Bytes,
    broadcaster: web::Data<Broadcaster>,
) -> Result<HttpResponse, actix_web::Error> {
    let msg = String::from_utf8(body.to_vec()).map_err(actix_web::error::ErrorBadRequest)?;
    broadcaster.broadcast(msg.as_str()).await;
    Ok(HttpResponse::Ok().finish())
}
