use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Unauthorized,
    Io(std::io::Error),
    Sqlx(sqlx::Error),
    Multipart(axum::extract::multipart::MultipartError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(message) | AppError::NotFound(message) => write!(f, "{message}"),
            AppError::Unauthorized => write!(f, "missing or invalid x-api-key"),
            AppError::Io(err) => write!(f, "{err}"),
            AppError::Sqlx(err) => write!(f, "{err}"),
            AppError::Multipart(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "missing or invalid x-api-key".to_string(),
            ),
            AppError::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Sqlx(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "resource not found".to_string())
            }
            AppError::Sqlx(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Multipart(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Sqlx(value)
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        AppError::Multipart(value)
    }
}
