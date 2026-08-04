use axum::response::IntoResponse;
use serde::Serialize;
use thiserror::Error;

// ──────────────────────────────────────────────────────────────────────────────
// AppError
//
// 统一的应用程序错误枚举。所有 HTTP 处理层抛出的错误都应映射到此类型，
// 使得 IntoResponse 实现可以将错误转换为带状态码和 JSON 体的 HTTP 响应。
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库操作失败（来自 sqlx）
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    /// 内部服务器错误（未预期的运行时异常）
    #[error("internal server error")]
    Internal,

    /// 客户端请求参数错误
    #[error("bad request: {0}")]
    BadRequest(String),

    /// 身份验证失败（缺少或无效 token）
    #[error("unauthorized")]
    Unauthorized,

    /// 请求的资源不存在
    #[error("not found")]
    NotFound,
}

// 将 AppError 转换为 HTTP 响应：
//   - Database / Internal  → 500 Internal Server Error
//   - BadRequest           → 400 Bad Request
//   - Unauthorized         → 401 Unauthorized
//   - NotFound             → 404 Not Found
//
// 响应体统一使用 ActionResult 格式的 JSON，错误时 data 字段为 null。
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::Database(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            AppError::NotFound => axum::http::StatusCode::NOT_FOUND,
        };

        // 错误响应与成功响应使用相同的 JSON 结构，便于前端统一处理。
        let body = axum::Json(serde_json::json!({
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
