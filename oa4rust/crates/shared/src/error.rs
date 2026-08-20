use axum::response::IntoResponse;
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

    /// Redis 操作失败
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),

    /// 其他内部错误（来自 anyhow）
    #[error("internal error: {0}")]
    InternalAnyhow(#[from] anyhow::Error),

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

    /// 权限不足（已认证但无权限访问）
    #[error("forbidden")]
    Forbidden,

    /// 请求的功能尚未实现
    #[error("not implemented")]
    NotImplemented,
}

// 将 AppError 转换为 HTTP 响应：
//   - Database / Internal / Redis / InternalAnyhow  → 500 Internal Server Error
//   - BadRequest           → 400 Bad Request
//   - Unauthorized         → 401 Unauthorized
//   - Forbidden            → 403 Forbidden
//   - NotFound             → 404 Not Found
//
// 响应体统一使用 ActionResult 格式的 JSON，错误时 data 字段为 null。
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AppError::Database(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Redis(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalAnyhow(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden => axum::http::StatusCode::FORBIDDEN,
            AppError::NotFound => axum::http::StatusCode::NOT_FOUND,
            AppError::NotImplemented => axum::http::StatusCode::NOT_IMPLEMENTED,
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
