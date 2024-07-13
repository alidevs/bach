use crate::{
    commons::schema::ErrorResponse,
    schema::users,
    state::AppState,
    user::{models::NewUserOwned, schemas::CreateUserRequest},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::{insert_into, Connection, RunQueryDsl};
use serde::Serialize;
use uuid::Uuid;

pub async fn create_user_service(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get connection: {}", e);
            let error_response = ErrorResponse {
                error: "Failed to get connection".to_string(),
                details: e.to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response();
        }
    };

    let new_user = NewUserOwned {
        id: Uuid::new_v4().to_string(),
        username: body.username,
        first_name: body.first_name,
        last_name: body.last_name,
        email: body.email,
        timestamp: chrono::Utc::now().naive_utc(),
    };

    let result = conn
        .interact(move |conn| {
            conn.transaction::<_, diesel::result::Error, _>(|conn| {
                let new_user_ref = new_user.as_new_user();

                let insert_result = insert_into(users::table)
                    .values(&new_user_ref)
                    .execute(conn);

                match insert_result {
                    Ok(_) => {
                        tracing::info!("User insertion succeeded");
                        Ok(new_user)
                    }
                    Err(e) => {
                        tracing::error!("User insertion failed: {}", e);
                        Err(e)
                    }
                }
            })
        })
        .await;

    match result {
        Ok(Ok(user)) => {
            tracing::info!("User created successfully");
            create_response(StatusCode::CREATED, user)
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

fn create_response<T: Serialize>(status_code: StatusCode, body: T) -> Response {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .unwrap()
        .into_response()
}
