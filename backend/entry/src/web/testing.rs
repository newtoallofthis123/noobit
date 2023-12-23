//? This file is used to create routes for testing purposes only
//? These might make into Prod, but for now, they are only for testing
//? They are the most inefficient way to do things, but they are the easiest to write
//? and they are the easiest to understand
//? So, if you want a overview of how to do things, this is the place to look

use axum::{
    routing::{get, MethodRouter},
    Router,
};
use tracing::info;

use crate::{config::config, model};

async fn serve_check_status() -> MethodRouter {
    info!("->> {:<12} {}", "GET", "/version");
    get(|| async { format!("OK: v.{}", config().version) })
}

async fn server_check_db_status() -> MethodRouter{
    get(|| async {
        let test_model = model::base::Model::new().await.unwrap();
        test_model.test_db().await.unwrap();
        "OK: Database connection".to_string()
    })
}

/// Returns a router with all the testing routes
/// 
/// Including:
/// * /version
/// * /db
/// 
/// To be used for **testing** purposes only
/// 
/// Might or might not make it into production
pub async fn test_routes() -> Router {
    Router::new()
    .route("/version", serve_check_status().await)
    .route("/db", server_check_db_status().await)
}