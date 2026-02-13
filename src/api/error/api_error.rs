use std::fmt;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::api::model::response::ErrorResponse;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    #[allow(dead_code)]
    BadRequest(String),
    #[allow(dead_code)]
    Internal(String),
    Database(sqlx::Error),
    Validation(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::Database(err) => write!(f, "Database error: {}", err),
            ApiError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Database(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_key, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal", msg.clone()),
            ApiError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database",
                "A database error occurred".to_string(),
            ),
            ApiError::Validation(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "validation", msg.clone())
            }
        };

        (status, Json(ErrorResponse::new(error_key, message))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_not_found() {
        let err = ApiError::NotFound("Todo not found".to_string());
        assert_eq!(err.to_string(), "Not found: Todo not found");
    }

    #[test]
    fn display_bad_request() {
        let err = ApiError::BadRequest("Invalid input".to_string());
        assert_eq!(err.to_string(), "Bad request: Invalid input");
    }

    #[test]
    fn display_internal() {
        let err = ApiError::Internal("Something broke".to_string());
        assert_eq!(err.to_string(), "Internal error: Something broke");
    }

    #[test]
    fn display_validation() {
        let err = ApiError::Validation("Title required".to_string());
        assert_eq!(err.to_string(), "Validation error: Title required");
    }

    #[test]
    fn debug_not_found() {
        let err = ApiError::NotFound("missing".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("NotFound"));
        assert!(debug.contains("missing"));
    }

    #[test]
    fn into_response_not_found_returns_404() {
        let err = ApiError::NotFound("Todo not found".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn into_response_bad_request_returns_400() {
        let err = ApiError::BadRequest("Bad".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn into_response_internal_returns_500() {
        let err = ApiError::Internal("Oops".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn into_response_validation_returns_422() {
        let err = ApiError::Validation("Invalid".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn from_sqlx_error() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let api_err: ApiError = sqlx_err.into();
        assert!(matches!(api_err, ApiError::Database(_)));
    }
}
