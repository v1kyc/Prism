use axum::extract::Multipart;
use axum::extract::multipart::Field;
use serde::de::DeserializeOwned;
use crate::rxtx::network::AppError;

pub async fn extract<T: DeserializeOwned>(
    multipart: &'_ mut Multipart,
) -> Result<(T, Field<'_>), AppError> {
    let options = multipart
        .next_field()
        .await
        .map_err(|_| AppError::InvalidInput("Failed to read field".to_string()))?;
    if let Some(field) = options {
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::InvalidInput("Failed to read bytes".to_string()))?;

        let opts: T = serde_json::from_slice(&bytes)
            .map_err(|_| AppError::InvalidInput("Invalid options format".to_string()))?;

        let file = multipart
            .next_field()
            .await
            .map_err(|_| AppError::InvalidInput("Failed to read field".to_string()))?
            .ok_or(AppError::InvalidInput("Missing file field".to_string()))?;

        Ok((opts, file))
    } else {
        Err(AppError::InvalidInput("Missing options field".to_string()))
    }
}
