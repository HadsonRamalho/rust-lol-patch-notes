use axum::{Router, http::HeaderValue, routing::get};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers;

pub async fn get_router() -> Router<()> {
    let router = Router::new()
        .route("/list_champions", get(controllers::list_champions))
        .route("/champion_info/", get(controllers::champion_info))
        .route("/champion_notes/", get(controllers::champion_notes))
        .layer(
            CorsLayer::new()
                .allow_origin(vec![
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                ])
                .allow_methods(Any)
                .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE]),
        );

    router
}
