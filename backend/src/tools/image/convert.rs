use std::io::Cursor;
use axum::{
    extract::{Multipart, multipart::Field},
    response::Response,
};
use crate::{
    rxtx::network::{AppError, multipart::extract},
    tools::image::ImageFormat,
};
use serde::Deserialize;
use image::ImageReader;

#[derive(Deserialize)]
pub struct ImageConvertOptions {
    pub target: ImageFormat,
}
pub async fn convert(mut multipart: Multipart) -> Result<Response, AppError> {

    let (opts, file): (ImageConvertOptions, Field) = extract(&mut multipart).await?;

    // Image
    let bytes = file.bytes().await
        .map_err(|_| AppError::InvalidInput("Failed to read image".to_string()))?;

    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| AppError::InvalidInput("Failed to guess format".to_string()))?
        .decode()
        .map_err(|_| AppError::InvalidInput("Failed to decode image".to_string()))?;

    let mut output = Vec::new();
    let mut cursor = Cursor::new(&mut output);
    img.write_to(&mut cursor, opts.target.into())
        .map_err(|_| AppError::InternalError)?;
    drop(cursor);

    let mime = opts.target.mime_type();

    Ok(Response::builder()
        .header("Content-Type", mime)
        .body(output.into())
        .unwrap())
}
