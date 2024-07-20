use crate::commons::schema::{ErrorResponse, ErrorResponseWithStatus};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool_diesel::postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub async fn get_connection(&self) -> Result<deadpool_diesel::postgres::Connection, Response> {
        match self.pool.get().await {
            Ok(conn) => Ok(conn),

            Err(e) => {
                tracing::error!("failed to get connection: {}", e);

                let error_response = ErrorResponse {
                    error: "failed to get connection".to_string(),
                    details: e.to_string(),
                };

                let error_response_with_status = ErrorResponseWithStatus {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    error_response,
                };

                Err(error_response_with_status.into_response())
            }
        }
    }
}
