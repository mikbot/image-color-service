use crate::endpoints::{calculate_color, get_index};
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::info;

mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let port: u16 = std::env::var("PORT")
        .map(|env| env.parse())
        .unwrap_or(Ok(8080))
        .unwrap();
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(calculate_color)
            .service(get_index)
    })
    .bind(("0.0.0.0", port))?
    .run();
    info!("Listening on http://0.0.0.0:{}", port);
    server.await
}
