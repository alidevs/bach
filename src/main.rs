mod commons;
pub mod schema;
mod state;
mod user;

use axum::{
    routing::{get, post},
    Router,
};
use commons::app_state;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user::services::{
    create_user_service::create_user_service, list_users_service::list_users_service,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = app_state::build_app_state();

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/user", post(create_user_service))
        .route("/user", get(list_users_service))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server is up and running on port 3000 ...");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
