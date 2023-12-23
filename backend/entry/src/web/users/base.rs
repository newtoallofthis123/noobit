use axum::Router;

use super::create::handle_create_user;

pub async fn user_routes() -> Router {
    Router::new()
        .route("/add/user", axum::routing::post(handle_create_user))
}