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
pub async fn test_routes() -> Router {
    Router::new()
    .route("/version", serve_check_status().await)
    .route("/db", server_check_db_status().await)
}