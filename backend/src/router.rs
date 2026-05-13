use crate::tools::image::convert;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

const MAX_BODY_SIZE: usize = 1024 * 1024 * 10;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async {}))
        .nest("/api", api_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(DefaultBodyLimit::max(MAX_BODY_SIZE)),
        )
}

fn api_routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/image", image_routes())
        .nest("/auth", auth_routes())
}
async fn health() -> &'static str {
    "Healthy"
}

fn image_routes() -> Router {
    Router::new().route("/convert", post(convert))
}

fn auth_routes() -> Router {
    Router::new()
        .route("/register", post(""))
        .route("/login", post(""))
}