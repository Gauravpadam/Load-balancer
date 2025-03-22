use axum::{Router, routing::get};
use crate::modules::services;
use services::hello_service::hello_world;
use services::simulate_load::simulate_load;

pub fn init_router() -> Router{
    Router::new()
        .route("/", get(hello_world))
        .route("/simulate_load", get(simulate_load))
}