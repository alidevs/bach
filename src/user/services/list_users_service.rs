use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    commons::schema::ErrorResponse,
    schema::users,
    state::AppState,
    user::{
        models::User,
        schemas::{ListUsersResponse, UserSchema},
    },
};

pub async fn list_users_service(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.get_connection().await.unwrap();

    let users = conn
        .interact(move |conn| {
            conn.transaction::<_, diesel::result::Error, _>(|conn| {
                users::table
                    .order_by(users::timestamp.desc())
                    .load::<User>(conn)
            })
        })
        .await;

    match users {
        Ok(Ok(users)) => {
            tracing::info!("Users retrieved successfully");
            let response_users: Vec<UserSchema> = users.into_iter().map(Into::into).collect();
            into_response(
                StatusCode::OK,
                ListUsersResponse {
                    users: response_users,
                },
            )
        }

        Ok(Err(e)) => {
            tracing::error!("Failed to retrieve users: {}", e);
            let error_response = ErrorResponse {
                error: "Failed to retrieve users".to_string(),
                details: e.to_string(),
            };
            into_response(StatusCode::INTERNAL_SERVER_ERROR, error_response)
        }

        Err(e) => {
            tracing::error!("Failed to interact with the database: {}", e);
            let error_response = ErrorResponse {
                error: "Database interaction failed".to_string(),
                details: e.to_string(),
            };
            into_response(StatusCode::INTERNAL_SERVER_ERROR, error_response)
        }
    }
}

fn into_response<T: serde::Serialize>(status: StatusCode, body: T) -> Response {
    (status, Json(body)).into_response()
}
