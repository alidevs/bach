use std::env;

use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;

use crate::state::AppState;

pub fn build_app_state() -> AppState {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must bet set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    AppState { pool }
}
