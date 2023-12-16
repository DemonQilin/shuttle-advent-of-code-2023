use axum::{extract::Multipart, response, routing::post, Router};
use image::{GenericImageView, ImageError, ImageFormat};
use tower_http::services::ServeDir;

async fn get_magical_red_pixels_total(mut multipart: Multipart) -> response::Result<String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name != "image" {
            continue;
        }

        let data = field.bytes().await?;
        let image = match image::load_from_memory_with_format(&data, ImageFormat::Png) {
            Ok(image) => image,
            Err(error) => {
                if let ImageError::Decoding(_) = error {
                    return Err("The data in field \"recipe\" is not valid png image".into());
                } else {
                    return Err("Something went wrong while process the image in \"recipe\"".into());
                }
            }
        };

        let magical_red_pixels = image
            .pixels()
            .filter(|(_, _, rgba)| {
                let red = rgba.0[0];
                let green = rgba.0[1];
                let blue = rgba.0[2];

                red as u16 > (blue as u16 + green as u16)
            })
            .count()
            .to_string();

        return Ok(magical_red_pixels);
    }

    Err("Field \"recipe\" was not founded".into())
}

pub fn get_imagery_routes() -> Router {
    Router::new()
        .route("/red_pixels", post(get_magical_red_pixels_total))
        .nest_service("/assets", ServeDir::new("assets"))
}
