use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::services::{
    get_my_profile::get_my_profile_service, list_users_service::list_users_service,
    login_service::login_service, register_user_service::register_user_service,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/register", post(register_user_service))
        .route("/user", get(list_users_service))
        .route("/user/me", get(get_my_profile_service))
        .route("/login", post(login_service))
}
