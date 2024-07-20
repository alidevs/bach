use crate::{
    commons::auth::Claims,
    schema::users,
    state::AppState,
    user::{models::User, schemas::UserResponse},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;

pub async fn get_my_profile_service(
    State(state): State<AppState>,
    claims: Claims,
) -> impl IntoResponse {
    let conn = state.get_connection().await.unwrap();

    let result = conn
        .interact(move |conn| {
            use self::users::dsl::*;

            conn.transaction::<_, diesel::result::Error, _>(|conn| {
                let result = users.filter(id.eq(claims.sub)).load::<User>(conn)?;

                Ok(result.into_iter().next())
            })
        })
        .await;

    match result {
        Ok(Ok(Some(user))) => Json(UserResponse {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            timestamp: user.timestamp,
        })
        .into_response(),

        Ok(Ok(None)) => (StatusCode::NOT_FOUND, "user not found").into_response(),

        Ok(Err(e)) => {
            tracing::error!("failed to retrieve user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to retrieve user").into_response()
        }

        Err(e) => {
            tracing::error!("failed to interact with the database: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to interact with the database",
            )
                .into_response()
        }
    }
}
