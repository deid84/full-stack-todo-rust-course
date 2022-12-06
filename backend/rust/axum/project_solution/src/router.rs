use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    routes::{hello_world::hello_world, users::create_user::create_user},
    utilities::app_state::AppState,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .with_state(app_state)
}
