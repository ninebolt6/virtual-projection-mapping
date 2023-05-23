use actix_web::{middleware::Logger, web, App, HttpServer};
use broadcast::Broadcaster;
use env_logger::Env;

mod broadcast;
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(Broadcaster::new()))
            .configure(route::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
