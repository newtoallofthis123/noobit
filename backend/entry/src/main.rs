use crate::config::config;
use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod config;
mod errors;
mod helpers;
mod web;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time() // for now
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let model = model::base::Model::new().await.unwrap();
    model.init_db().await.unwrap();

    let app = Router::new()
        .merge(web::testing::test_routes().await) // The testing and status check routes
        .merge(web::users::base::user_routes().await) // The user routes
        .nest_service("/assets", ServeDir::new(&config().static_folder)); // static assets

    let addr = SocketAddr::from(([127, 0, 0, 1], config().main_port.parse::<u16>().expect("Invalid port")));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("->> {:<12} {}", "Listening on", addr);
    axum::serve(listener, app).await.unwrap();
}
