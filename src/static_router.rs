use axum::routing::get_service;
use tower_http::services::ServeDir;

pub fn image_dir_router() -> axum::Router {
    // create the data directory if it does not exist
    axum::Router::new().nest_service(
        "/static",
        get_service(ServeDir::new("./public")),
    )
}

