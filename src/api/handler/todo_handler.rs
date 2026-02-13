use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sqlx::SqlitePool;

use crate::api::error::api_error::ApiError;
use crate::api::model::request::{CreateTodoRequest, UpdateTodoRequest};
use crate::api::model::todo::Todo;
use crate::database::repository::todo_repository;

pub async fn create(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    if body.title().trim().is_empty() {
        return Err(ApiError::Validation("Title must not be empty".to_string()));
    }

    let todo = todo_repository::create(&pool, body.title(), body.description()).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = todo_repository::find_all(&pool).await?;

    Ok(Json(todos))
}

pub async fn get_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, ApiError> {
    let todo = todo_repository::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Todo with id {} not found", id)))?;

    Ok(Json(todo))
}

pub async fn update(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, ApiError> {
    if body.title().trim().is_empty() {
        return Err(ApiError::Validation("Title must not be empty".to_string()));
    }

    let todo = todo_repository::update(
        &pool,
        id,
        body.title(),
        body.description(),
        body.completed(),
    )
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Todo with id {} not found", id)))?;

    Ok(Json(todo))
}

pub async fn delete(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let deleted = todo_repository::delete(&pool, id).await?;

    if !deleted {
        return Err(ApiError::NotFound(format!("Todo with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
