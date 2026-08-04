use axum::response::{IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("internal server error")]
    Internal,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::Database(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            AppError::NotFound => axum::http::StatusCode::NOT_FOUND,
        };

        let body = Json(serde_json::json!({
            "data": None::<serde_json::Value>,
            "type": "error",
            "message": self.to_string(),
            "date": None::<Option<String>>,
            "spent": None::<Option<i64>>,
            "size": None::<Option<i64>>,
            "count": None::<Option<i64>>,
            "position": None::<Option<String>>,
            "prompt": None::<Option<String>>,
        }));

        (status, body).into_response()
    }
}

#[derive(Serialize)]
pub struct ActionResult<T> {
    pub data: Option<T>,
    pub r#type: Option<String>,
    pub message: Option<String>,
    pub date: Option<String>,
    pub spent: Option<i64>,
    pub size: Option<i64>,
    pub count: Option<i64>,
    pub position: Option<String>,
    pub prompt: Option<String>,
}

impl<T> ActionResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            r#type: Some("success".to_string()),
            message: None,
            date: None,
            spent: None,
            size: None,
            count: None,
            position: None,
            prompt: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            data: None,
            r#type: Some("error".to_string()),
            message: Some(message.into()),
            date: None,
            spent: None,
            size: None,
            count: None,
            position: None,
            prompt: None,
        }
    }
}
