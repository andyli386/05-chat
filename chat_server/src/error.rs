use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),
    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::ToStrError),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let status = match &self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
        };
        // (status, Json(json!({"error": self.to_string()}))).into_response()
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
