use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    NotImplemented,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message, expl) = match self {
            Self::InternalServerError(expl) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                expl,
            ),
            Self::BadRequest(expl) => (StatusCode::BAD_REQUEST, "Bad Request", expl,),
            Self::NotFound(expl) => (StatusCode::NOT_FOUND, "Not Found", expl),
            Self::NotImplemented => (StatusCode::INTERNAL_SERVER_ERROR, "Not Implemented", String::new())
        };

        (status, Json(json!({"error": error_message, "explanation": expl}))).into_response()
    }
}

pub async fn not_found() -> AppError {
    AppError::InternalServerError(String::new())
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound(String::from("Not Found")),
            _ => AppError::InternalServerError(value.to_string()),
        }
    }
}

impl From<minijinja::Error> for AppError {
    fn from(value: minijinja::Error) -> Self {
        Self::InternalServerError(String::from("Minijinja error: ") + &value.to_string())
    }
}