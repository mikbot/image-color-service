use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn get_index() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "documentation": "https://mikbot.github.io/image-color-service/"
    }))
}
