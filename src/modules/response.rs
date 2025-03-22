use serde::Serialize;
use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};

#[derive(Serialize)]
pub struct Message{
    pub message: String,
}

pub enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self{
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}