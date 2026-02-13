use axum::Router;
use axum::routing::{delete, get, post, put};
use sqlx::SqlitePool;

use crate::api::handler::todo_handler;

pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/todos", post(todo_handler::create))
        .route("/todos", get(todo_handler::list))
        .route("/todos/{id}", get(todo_handler::get_by_id))
        .route("/todos/{id}", put(todo_handler::update))
        .route("/todos/{id}", delete(todo_handler::delete))
        .with_state(pool)
}
