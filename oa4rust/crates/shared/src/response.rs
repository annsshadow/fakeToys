use axum::response::{IntoResponse, Json};
use deadpool_postgres::tokio_postgres::types::{FromSql, Type};
use deadpool_postgres::tokio_postgres::Row;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

/// 将一行 PostgreSQL 记录转换为 JSON 对象，供通用真实化 handler 复用。
///
/// 支持核心标量类型（bool / 整数 / 浮点 / 文本）。未知类型或 NULL 列映射为
/// `Value::Null`。该函数的目的是让"通用 SELECT" handler 在不知道具体列名的情况下
/// 也能返回真实数据，避免 `Value::Null` 桩。复杂/带时间类型的列建议在具体 handler
/// 中显式映射。
pub fn row_to_json(row: &Row) -> Value {
    let mut map = serde_json::Map::new();
    for col in row.columns() {
        let name = col.name();
        let val: Option<Value> = match *col.type_() {
            Type::BOOL => row.get::<_, Option<bool>>(name).map(Value::Bool),
            Type::INT2 => row.get::<_, Option<i16>>(name).map(|v| Value::Number((v as i64).into())),
            Type::INT4 => row.get::<_, Option<i32>>(name).map(|v| Value::Number((v as i64).into())),
            Type::INT8 => row.get::<_, Option<i64>>(name).map(|v| Value::Number(v.into())),
            Type::FLOAT4 => row
                .get::<_, Option<f32>>(name)
                .and_then(|v| serde_json::Number::from_f64(v as f64).map(Value::Number)),
            Type::FLOAT8 => row
                .get::<_, Option<f64>>(name)
                .and_then(|v| serde_json::Number::from_f64(v).map(Value::Number)),
            Type::TEXT | Type::VARCHAR | Type::NAME | Type::BPCHAR => {
                row.get::<_, Option<String>>(name).map(Value::String)
            }
            _ => None,
        };
        if let Some(v) = val {
            map.insert(name.to_string(), v);
        }
    }
    Value::Object(map)
}

/// 将 Option<T> 序列化为 JSON：Some(v) → Value::from(v)，None → 省略字段（不插入 map）
pub fn option_to_json<T: Serialize>(opt: Option<T>) -> Option<Value> {
    opt.map(|v| serde_json::to_value(v).unwrap_or(Value::Null))
}

/// 从 row 中安全提取 Option<T>，避免 Value::Null。
/// 与 row.get::<_, Option<T>>() 类似，但返回 Option<Value> 而非 Value::Null
pub fn row_opt_json<T: Serialize + for<'a> FromSql<'a>>(
    row: &Row,
    col: &str,
) -> Option<Value> {
    row.get::<_, Option<T>>(col).map(|v| serde_json::to_value(v).unwrap())
}

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
// 错误体与 Java 错误信封实测形状一致：无 data 字段，元数据字段恒填充，
// prompt 承载 Java 异常类名风格字符串（见 java_exception_for）。
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, prompt_kind) = match &self {
            AppError::Database(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "ExceptionInternal"),
            AppError::BadRequest(_) => (axum::http::StatusCode::BAD_REQUEST, "ExceptionBadRequest"),
            AppError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, "ExceptionUnauthorized"),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "ExceptionEntityNotExist"),
        };

        let body = Json(serde_json::json!({
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

// ──────────────────────────────────────────────────────────────────────────────
// error_response
//
// 中间件层（认证/授权/限流）统一生成 ActionResult 格式的错误响应。
// 与 AppError::IntoResponse 保持相同 JSON 结构（Java 实测形状）。
// ──────────────────────────────────────────────────────────────────────────────
pub fn error_response(status: axum::http::StatusCode, message: impl Into<String>) -> axum::response::Response {
    // 恒填 prompt（Java ResponseFactory 多数路径行为，净差异最小策略）。
    let prompt_kind = match status.as_u16() {
        401 => "ExceptionUnauthorized",
        404 => "ExceptionEntityNotExist",
        _ => "ExceptionInternal",
    };
    let body = Json(serde_json::json!({
        "type": "error",
        "message": message.into(),
        "date": java_date_now(),
        "spent": 0,
        "size": -1,
        "count": 0,
        "position": 0,
        "prompt": java_exception_for(prompt_kind),
    }));

    (status, body).into_response()
}

// ──────────────────────────────────────────────────────────────────────────────
// ActionResult<T>
//
// 所有 API 响应的统一 JSON 结构。前端据此字段区分成功/错误并读取数据。
//
// 序列化对齐（2026-08-25 行为对比实跑结论，基准 o2server v9 Gson 实测）：
// Java (Gson) 对 null 字段一律省略不输出，因此所有 Option 字段在 None 时
// 跳过序列化；成功信封的元数据字段（message/date/spent/size/count/position）
// 由 Java 恒填充，故 success() 默认填充同形状默认值（对比器只比较字段名与
// 标量类型，spent/date 的具体值本就随请求变化）。
//
// 字段说明：
//   data      — 业务数据（成功时填充，失败时省略）
//   type      — "success" 或 "error"，前端用于分支处理
//   message   — 错误描述或额外提示（成功时为空串，与 Java 一致）
//   date      — 服务器时间戳 "yyyy-MM-dd HH:mm:ss"（Java 恒填）
//   spent     — 处理耗时毫秒数（此处恒 0，精确值属 R1 影子流量范畴）
//   size      — 分页页大小（Java 默认 -1 表示未分页 / 0）
//   count     — 记录总数（用于分页）
//   position  — O2OA v9 信封中为数字（实测恒为 0），用 Value 承载
//   prompt    — 仅错误信封携带的 Java 异常类名；成功信封省略
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Serialize)]
pub struct ActionResult<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub r#type: Option<String>,
    pub message: Option<String>,
    pub date: Option<String>,
    pub spent: Option<i64>,
    pub size: Option<i64>,
    pub count: Option<i64>,
    /// O2OA v9 Java 信封中 position 为数字（实测恒为 0），因此用 Value 承载。
    pub position: Option<Value>,
    /// Java 成功与错误信封均恒填充 prompt（错误为异常类名，成功为空串），
    /// 故 success()/java_success() 恒填 Some("") 以避免字段缺失。
    pub prompt: Option<String>,
}

impl<T> ActionResult<T> {
    // 构造一个成功响应：元数据默认值与 Java 成功信封实测形状一致
    // （message 为空串、date 为服务器时间、count/position 为 0、size 为 0）。
    // 分页端点应优先使用 java_success(count, size) 提供真实计数。
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            r#type: Some("success".to_string()),
            message: Some(String::new()),
            date: Some(java_date_now()),
            spent: Some(0),
            size: Some(0),
            count: Some(0),
            position: Some(Value::Number(serde_json::Number::from(0))),
            prompt: Some(String::new()),
        }
    }

    // 构造一个错误响应：与 Java 错误信封实测形状一致——无 data 字段，
    // prompt 承载 Java 异常类名风格字符串。实测 O2OA ResponseFactory 在
    // 绝大多数 war 的错误路径填充 prompt（个别模块省略，见 allowlist 留档
    // 「java-error-prompt-inconsistent」），恒填为净差异最小的对齐策略。
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            data: None,
            r#type: Some("error".to_string()),
            message: Some(message.into()),
            date: Some(java_date_now()),
            spent: Some(0),
            size: Some(-1),
            count: Some(0),
            position: Some(Value::Number(serde_json::Number::from(0))),
            prompt: Some(java_exception_for("ExceptionEntityNotExist")),
        }
    }

    /// O2OA v9 Java 兼容成功信封（plan002 U2 行为对齐，基准实测见
    /// docs/audits/behavior-compare-first-run.md）。
    ///
    /// Java 信封所有元数据字段恒存在：message 为空串、date 为服务器时间
    /// （"yyyy-MM-dd HH:mm:ss"）、spent 为耗时毫秒数（此处置 0）、size/count
    /// 为数字、position 为数字 0。分页端点传 (total, data.len())，
    /// 非分页端点按 Java 实测传 (0, -1)。
    pub fn java_success(data: T, count: i64, size: i64) -> Self {
        Self {
            data: Some(data),
            r#type: Some("success".to_string()),
            message: Some(String::new()),
            date: Some(java_date_now()),
            spent: Some(0),
            size: Some(size),
            count: Some(count),
            position: Some(Value::Number(serde_json::Number::from(0))),
            prompt: Some(String::new()),
        }
    }
}

/// O2OA v9 Java 信封 date 字段格式："yyyy-MM-dd HH:mm:ss"
pub fn java_date_now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 按 Rust 错误类别返回 O2OA 风格的 Java 异常类名（错误信封 prompt 字段）。
/// Java 将异常类全名放入 prompt（如 ExceptionUnauthorized / ExceptionEntityNotExist）。
pub fn java_exception_for(kind: &str) -> String {
    let class = match kind {
        "ExceptionEntityNotExist" => "ExceptionEntityNotExist",
        "ExceptionBadRequest" => "ExceptionBadRequest",
        "ExceptionUnauthorized" => "ExceptionUnauthorized",
        "ExceptionAccessDenied" => "ExceptionAccessDenied",
        "ExceptionNotImplemented" => "ExceptionNotImplemented",
        _ => "ExceptionInternal",
    };
    format!("com.x.base.core.project.exception.{class}")
}
