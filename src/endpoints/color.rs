use actix_web::{post, web::Bytes, HttpMessage, HttpRequest, HttpResponse, error::PayloadError, ResponseError, http::StatusCode};
use image::{ImageFormat, ImageError};
use serde_json::json;
use thiserror::Error;

#[post("/color")]
pub async fn calculate_color(bytes: Bytes, request: HttpRequest) -> Result<HttpResponse, ColorError> {
    let format = match ImageFormat::from_mime_type(request.content_type()) {
        Some(format) => format,
        None => {
            let error = match image::guess_format(&bytes) {
                Ok(guess) => ColorError::InvalidImageFormatWithGuess {
                    guess: guess.extensions_str().first().unwrap().to_string(),
                },
                Err(_) => ColorError::InvalidImageFormat,
            };
            return Err(error);
        }
    };
    let image = image::load_from_memory_with_format(&bytes, format)?;
    let img =
        color_thief::get_palette(image.as_bytes(), color_thief::ColorFormat::Rgb, 10, 2).unwrap();
    let colors: Vec<u32> = img
        .iter()
        .map(|rgb| {
            let r = rgb.r as u32;
            let g = rgb.g as u32;
            let b = rgb.b as u32;
            r << 16 | g << 8 | b
        })
        .collect();
    let json = json!({ "colors": colors });
    Ok(HttpResponse::Ok().json(json))
}

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Invalid image format")]
    InvalidImageFormat,
    #[error("Invalid image format.\nIf the image you submitted is of type {guess} set your content-type header accordingly.\nSee https://www.iana.org/assignments/media-types/media-types.xhtml#image.")]
    InvalidImageFormatWithGuess { guess: String },
    #[error("Invalid payload")]
    PayloadError {
        #[from]
        source: PayloadError,
    },
    #[error("Invalid image data")]
    ImageError {
        #[from]
        source: ImageError,
    },
}

impl ResponseError for ColorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::BAD_REQUEST
    }
}
