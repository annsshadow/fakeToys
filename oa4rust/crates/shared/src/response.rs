use axum::response::{IntoResponse, Json};
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

// ──────────────────────────────────────────────────────────────────────────────
// error_response
//
// 中间件层（认证/授权/限流）统一生成 ActionResult 格式的错误响应。
// 与 AppError::IntoResponse 保持相同 JSON 结构，便于前端统一处理。
// ──────────────────────────────────────────────────────────────────────────────
pub fn error_response(status: axum::http::StatusCode, message: impl Into<String>) -> axum::response::Response {
    let body = Json(serde_json::json!({
        "data": None::<serde_json::Value>,
        "type": "error",
        "message": message.into(),
        "date": None::<Option<String>>,
        "spent": None::<Option<i64>>,
        "size": None::<Option<i64>>,
        "count": None::<Option<i64>>,
        "position": None::<Option<String>>,
        "prompt": None::<Option<String>>,
    }));

    (status, body).into_response()
}

// ──────────────────────────────────────────────────────────────────────────────
// ActionResult<T>
//
// 所有 API 响应的统一 JSON 结构。前端据此字段区分成功/错误并读取数据。
//
// 字段说明：
//   data      — 业务数据（成功时填充，失败时为 None）
//   type      — "success" 或 "error"，前端用于分支处理
//   message   — 错误描述或额外提示（成功时通常为 None）
//   date      — 可选的时间戳字符串（如 API 调用时间）
//   spent     — 可选的处理耗时（毫秒）
//   size      — 可选的数据字节大小
//   count     — 可选的记录总数（用于分页）
//   position  — 可选的光标/偏移量（用于分页或流式响应）
//   prompt    — 可选的原始输入文本（LLM 相关接口使用）
// ──────────────────────────────────────────────────────────────────────────────
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
    // 构造一个成功响应，仅填充 data 和 type 字段
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

    // 构造一个错误响应，仅填充 type 和 message 字段
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
