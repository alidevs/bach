use crate::{
    commons::schema::ErrorResponse,
    schema::users,
    state::AppState,
    user::{models::NewUser, schemas::CreateUserRequest},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::{insert_into, result::Error, Connection, PgConnection, RunQueryDsl};
use serde::Serialize;

pub async fn create_user_service(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let conn = state.get_connection().await.unwrap();

    let new_user = NewUser::new(
        body.username,
        body.password,
        body.first_name,
        body.last_name,
        body.email,
    );

    let result = conn
        .interact(move |conn| conn.transaction(|conn| insert_new_user(new_user, conn)))
        .await;

    match result {
        Ok(Ok(new_user)) => {
            tracing::info!("User created successfully: {:?}", new_user);

            create_response(StatusCode::CREATED, new_user)
        }

        Ok(Err(e)) => {
            tracing::error!("Failed to create user: {}", e);

            let error_response = ErrorResponse {
                error: "Failed to create user".to_string(),
                details: e.to_string(),
            };

            create_response(StatusCode::INTERNAL_SERVER_ERROR, error_response)
        }

        Err(e) => {
            tracing::error!("Failed to interact with the database: {}", e);

            let error_response = ErrorResponse {
                error: "Database interaction failed".to_string(),
                details: e.to_string(),
            };

            create_response(StatusCode::INTERNAL_SERVER_ERROR, error_response)
        }
    }
}

fn insert_new_user(new_user: NewUser, conn: &mut PgConnection) -> Result<NewUser, Error> {
    match insert_into(users::table).values(&new_user).execute(conn) {
        Ok(_) => {
            tracing::info!("User insertion succeeded");

            Ok(new_user)
        }

        Err(e) => {
            tracing::error!("User insertion failed: {}", e);

            Err(e)
        }
    }
}

fn create_response<T: Serialize>(status_code: StatusCode, body: T) -> Response {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .unwrap()
        .into_response()
}
