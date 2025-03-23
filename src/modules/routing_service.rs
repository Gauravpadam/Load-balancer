use axum::{debug_handler, Router, routing::get, response::{Json, IntoResponse}, http::StatusCode};
use serde::{Deserialize, Serialize};
use axum::extract::Query;
use reqwest::Client;
use super::state::APP_STATE;

#[derive(Deserialize)]
struct QueryParameter {
    query: String,
}

#[derive(Serialize)]
struct ResponseError {
    message: String,
}

pub enum ApiResponse {
    OK(String),
    CREATED(String),
    JsonRes(String),
    Error(String),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::OK(message) => (StatusCode::OK, Json(message)).into_response(),
            ApiResponse::CREATED(message) => (StatusCode::CREATED, Json(message)).into_response(),
            ApiResponse::JsonRes(message) => (StatusCode::OK, Json(message)).into_response(),
            ApiResponse::Error(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseError { message }),
            )
            .into_response(),
        }
    }
}

#[debug_handler]
async fn routing_handler(query: Query<QueryParameter>) -> ApiResponse {
    let server_to_hit = {
        let mut state = APP_STATE.lock().await;
        state.token = (state.token % 3) + 1;
        state.token
    };

    let prompt = query.0;
    let client = Client::new();
    let url = match server_to_hit {
        1 => "http://server1:8080/simulate_load",
        2 => "http://server2:8080/simulate_load",
        3 => "http://server3:8080/simulate_load",
        _ => "http://server1:8080/simulate_load",
    };

    let response_text = match client
        .get(url)
        .query(&[("query", &prompt.query)])
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => return ApiResponse::Error(e.to_string()),
        },
        Err(e) => return ApiResponse::Error(e.to_string()),
    };

    ApiResponse::JsonRes(format!(
        "Request routed to server {} successfully: {}",
        server_to_hit, response_text
    ))
}

pub fn init_router() -> Router {
    Router::new().route("/", get(routing_handler))
}
