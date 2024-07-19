use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: String,
}

pub struct ErrorResponseWithStatus {
    pub status: StatusCode,
    pub error_response: ErrorResponse,
}

impl IntoResponse for ErrorResponseWithStatus {
    fn into_response(self) -> Response {
        let ErrorResponseWithStatus {
            status,
            error_response,
        } = self;
        let body = Json(error_response);
        (status, body).into_response()
    }
}
