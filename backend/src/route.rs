use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../resources/index.html"))
}
