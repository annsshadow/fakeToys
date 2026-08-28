use axum::response::IntoResponse;
use thiserror::Error;

use super::response::{java_date_now, java_exception_for};

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
// 错误体与 Java 错误信封实测形状一致（2026-08-25 行为对比实跑结论）：
// 无 data 字段，date/spent/size/count/position 恒填充，
// prompt 承载 Java 异常类名风格字符串。
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, prompt_kind) = match &self {
            AppError::Database(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::Redis(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::InternalAnyhow(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::BadRequest(_) => (axum::http::StatusCode::BAD_REQUEST, "ExceptionBadRequest"),
            AppError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, "ExceptionUnauthorized"),
            AppError::Forbidden => (axum::http::StatusCode::FORBIDDEN, "ExceptionAccessDenied"),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "ExceptionEntityNotExist"),
            AppError::NotImplemented => (axum::http::StatusCode::NOT_IMPLEMENTED, "ExceptionNotImplemented"),
        };

        // 恒填 prompt：O2OA ResponseFactory 多数 war 的错误路径填充异常类名
        // （个别模块省略，见 allowlist 留档「java-error-prompt-inconsistent」）。
        let body = axum::Json(serde_json::json!({
            "type": "error",
            "message": self.to_string(),
            "date": java_date_now(),
            "spent": 0,
            "size": -1,
            "count": 0,
            "position": 0,
            "prompt": java_exception_for(prompt_kind),
        }));

        (status, body).into_response()
    }
}
