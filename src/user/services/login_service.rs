use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Serialize;

use crate::{
    commons::{auth::Claims, schema::ErrorResponse},
    schema::users,
    state::AppState,
    user::{
        models::User,
        schemas::{LoginRequest, UserResponse},
    },
};

#[derive(Serialize)]
#[serde(untagged)]
enum LoginServiceResponse {
    Success(UserResponse),
    Error(ErrorResponse),
}

pub async fn login_service(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let conn = state.get_connection().await.unwrap();

    let users = conn
        .interact(move |conn| {
            conn.transaction(|conn| {
                users::table
                    .filter(users::username.eq(body.username))
                    .load::<User>(conn)
            })
        })
        .await;

    match users {
        Ok(Ok(users)) => {
            let user: Option<User> = users
                .into_iter()
                .find(|user| user.verify_password(body.password.clone()));

            match user {
                Some(user) => {
                    let token = Claims::new(user.id.clone());
                    let raw_token = token.encode();

                    into_response(
                        StatusCode::OK,
                        LoginServiceResponse::Success(UserResponse {
                            id: user.id,
                            username: user.username,
                            first_name: user.first_name,
                            last_name: user.last_name,
                            email: user.email,
                            timestamp: user.timestamp,
                        }),
                        Some(raw_token),
                    )
                }

                None => {
                    tracing::error!("invalid credentials");

                    into_response(
                        StatusCode::UNAUTHORIZED,
                        LoginServiceResponse::Error(ErrorResponse {
                            error: "invalid credentials".to_string(),
                            details: "invalid credentials".to_string(),
                        }),
                        None,
                    )
                }
            }
        }
        Ok(Err(e)) => {
            tracing::error!("failed to retrieve users: {}", e);

            let error_response = ErrorResponse {
                error: "server error".to_string(),
                details: e.to_string(),
            };

            into_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                LoginServiceResponse::Error(error_response),
                None,
            )
        }

        Err(e) => {
            tracing::error!("failed to interact with the database: {}", e);

            let error_response = ErrorResponse {
                error: "server error".to_string(),
                details: e.to_string(),
            };

            into_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                LoginServiceResponse::Error(error_response),
                None,
            )
        }
    }
}

fn into_response<T: Serialize>(
    status: StatusCode,
    body: T,
    token: Option<String>,
) -> Response<Body> {
    let mut response = Response::builder()
        .status(status)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    if let Some(token) = token {
        response
            .headers_mut()
            .insert("Authorization", token.parse().unwrap());
    }

    response
}
