use crate::model::ModelManager;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::controller::note_routes_controller::note_routes;

mod controller;
mod model;
mod shcema;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().without_time().init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let mm = ModelManager::new().await?;

    let app = Router::new().merge(note_routes(mm)).layer(cors);

    info!("{:<6} - {}", "LISTENING", 8081);
    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
