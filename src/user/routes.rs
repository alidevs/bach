use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::services::{
    create_user_service::create_user_service, list_users_service::list_users_service,
    login_service::login_service,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user", post(create_user_service))
        .route("/user", get(list_users_service))
        .route("/login", post(login_service))
}
