use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;
use crate::rxtx::network::AppError;
// Error response

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            ),

            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Internal server error".to_string(),
            ),

            AppError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "PAYLOAD_TOO_LARGE",
                "Payload too large".to_string(),
            ),

            AppError::InvalidInput(message) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
                format!("Invalid input: {}", message),
            ),
        };
        error_res(status, code, message)
    }
}
pub fn error_res(status: StatusCode, code: &str, message: String) -> Response {
    let res = json!({
        "ok": false,
        "data" : null,
        "error" : {
            "code" : code,
            "message": message,
        }
    });
    (status, Json(res)).into_response()
}
// Success response
pub fn success_res<T: Serialize>(data: T) -> Response {
    let res = json!({
        "ok": true,
        "data": data,
        "error": null
    });

    (StatusCode::OK, Json(res)).into_response()
}
