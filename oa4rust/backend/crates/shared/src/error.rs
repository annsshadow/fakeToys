// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::response::IntoResponse;
use thiserror::Error;
use tracing::{error, warn};

use super::response::{legacy_date_now, legacy_exception_for};

// ──────────────────────────────────────────────────────────────────────────────
// RequestContext / REQUEST_CONTEXT
//
// 5xx 日志必须能回答「哪个请求、什么方法、能否与上游网关对账」。
// `IntoResponse` 是同步的、拿不到原请求，因此由 trace_middleware 在进入 handler
// 前把 method/path/trace id 注入 task-local；没有注入的场景（单元测试、
// 启动期直调）退化为 `-`，日志仍可读。
// ──────────────────────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Default)]
pub struct RequestContext {
    pub method: String,
    pub path: String,
    /// 优先取上游 `x-request-id`，否则为中间件生成的 UUID。
    pub trace_id: Option<String>,
}

tokio::task_local! {
    pub static REQUEST_CONTEXT: RequestContext;
}

fn current_request_context() -> RequestContext {
    REQUEST_CONTEXT
        .try_with(|ctx| ctx.clone())
        .unwrap_or_else(|_| RequestContext {
            method: "-".to_string(),
            path: "-".to_string(),
            trace_id: None,
        })
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

    /// 其他内部错误（来自 anyhow）
    #[error("internal error: {0}")]
    InternalAnyhow(#[from] anyhow::Error),

    /// 内部服务器错误（未预期的运行时异常）
    #[error("internal server error")]
    Internal,

    /// 内部服务器错误 + 原始错误链。
    ///
    /// `context` 记录出错位置（通常 `file:line`），`cause` 保存原始错误的
    /// Debug 展示（anyhow 的 Debug 会带完整 `Caused by` 链）。
    /// 对外响应体仍是固定的 `internal server error`，不泄内部信息。
    #[error("internal server error at {context}")]
    InternalWithSource { context: String, cause: String },

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

    /// 服务暂时不可用（如连接池排队超时）——可重试，语义区别于 500。
    #[error("service unavailable: {0}")]
    Unavailable(String),

    /// 外部连接器/服务缺少必要配置（环境变量缺失、配置行未写入）——503。
    ///
    /// 与 500 分桶：这是「部署没配好」，不是「代码炸了」。`item` 写明缺哪一项
    ///（如 `ANDFX_KEY`、`ZWDINGDING_API_BASE`），便于运维直接定位。
    #[error("service not configured: missing {item}")]
    MissingConfig { item: String, context: String },
}

impl AppError {
    /// 构造带原始错误链的内部错误；`map_err` 禁止再用 `|_|` 丢弃 source。
    ///
    /// `context` 建议传 `concat!(file!(), ":", line!())`。
    pub fn internal_with_source<C, E>(context: C, source: E) -> Self
    where
        C: Into<String>,
        E: std::fmt::Debug,
    {
        Self::InternalWithSource {
            context: context.into(),
            cause: format!("{source:?}"),
        }
    }

    /// 缺少配置项（环境变量/配置行），映射 503 并在响应体中写明缺失项。
    #[track_caller]
    pub fn missing_config(item: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self::MissingConfig {
            item: item.into(),
            context: format!("{}:{}", loc.file(), loc.line()),
        }
    }

    /// 500 日志中的原始错误链；无 source 的变体返回 `-`。
    fn source_for_log(&self) -> String {
        match self {
            AppError::Database(e) => format!("{e:?}"),
            AppError::InternalAnyhow(e) => format!("{e:?}"),
            // O24：file:line 必须进日志。此前只记 cause，`at` 上下文只活在对象里，
            // 日志无法定位「哪一行丢的 source」。
            AppError::InternalWithSource { context, cause } => {
                format!("{context}: {cause}")
            }
            // 503 没有 source 链，但同样要给出「哪一行读的配置」。
            AppError::MissingConfig { context, .. } => context.clone(),
            _ => "-".to_string(),
        }
    }

    /// 响应体 message。新变体固定文案，避免把内部 source 泄给客户端。
    fn response_message(&self) -> String {
        match self {
            AppError::InternalWithSource { .. } => "internal server error".to_string(),
            _ => self.to_string(),
        }
    }
}

/// O7：tokio-postgres 查询/事务错误经 `?` 自动带上 O7 分类
/// （08xxx/57014/53300/53200/57P0x/断连 → 503 可重试；23505/42P01/语法错/
/// ToSql/FromSql 等代码缺陷 → 500）。
impl From<deadpool_postgres::tokio_postgres::Error> for AppError {
    fn from(e: deadpool_postgres::tokio_postgres::Error) -> Self {
        crate::db::classify_query_error(e)
    }
}

/// O8：sea-orm DbErr 经 `?` 自动带上分类（与 O7 共用 SQLSTATE 判据）：
/// sqlx Database 错误按 SQLSTATE 表；PoolTimedOut/PoolClosed/Io → 503；
/// sea-orm Internal/类型转换/RowNotFound → 500。
impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        crate::db::classify_orm_error(e)
    }
}

// 将 AppError 转换为 HTTP 响应：
//   - Database / Internal / InternalAnyhow        → 500 Internal Server Error
//   - BadRequest           → 400 Bad Request
//   - Unauthorized         → 401 Unauthorized
//   - Forbidden            → 403 Forbidden
//   - NotFound             → 404 Not Found
//
// 错误体与 o2server 错误信封实测形状一致（2026-08-25 行为对比实跑结论）：
// 无 data 字段，date/spent/size/count/position 恒填充，
// prompt 承载 o2server 异常类名风格字符串。
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, prompt_kind) = match &self {
            AppError::Database(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "ExceptionInternal",
            ),
            AppError::Internal => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "ExceptionInternal",
            ),
            AppError::InternalAnyhow(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "ExceptionInternal",
            ),
            AppError::InternalWithSource { .. } => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "ExceptionInternal",
            ),
            AppError::BadRequest(_) => (axum::http::StatusCode::BAD_REQUEST, "ExceptionBadRequest"),
            AppError::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                "ExceptionUnauthorized",
            ),
            AppError::Forbidden => (axum::http::StatusCode::FORBIDDEN, "ExceptionAccessDenied"),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "ExceptionEntityNotExist"),
            AppError::NotImplemented => (
                axum::http::StatusCode::NOT_IMPLEMENTED,
                "ExceptionNotImplemented",
            ),
            AppError::Unavailable(_) => (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "ExceptionInternal",
            ),
            AppError::MissingConfig { .. } => (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "ExceptionInternal",
            ),
        };

        // O23：5xx 必须留一条带请求上下文与原始 source 链的日志。
        // 500 用 ERROR；其他 5xx（如 503 未配置/池满）用 WARN——它们通常是
        // 环境或容量问题，不是代码缺陷，但不留记录同样无法定位。
        if status.is_server_error() {
            let ctx = current_request_context();
            let trace_id = ctx.trace_id.as_deref().unwrap_or("-");
            let source = self.source_for_log();
            if status == axum::http::StatusCode::INTERNAL_SERVER_ERROR {
                error!(
                    method = %ctx.method,
                    path = %ctx.path,
                    trace_id = %trace_id,
                    status = status.as_u16(),
                    error = %self.response_message(),
                    source = %source,
                    "5xx response"
                );
            } else {
                warn!(
                    method = %ctx.method,
                    path = %ctx.path,
                    trace_id = %trace_id,
                    status = status.as_u16(),
                    error = %self.response_message(),
                    source = %source,
                    "5xx response"
                );
            }
        }

        // 恒填 prompt：O2OA ResponseFactory 多数 war 的错误路径填充异常类名
        // （个别模块省略，见 allowlist 留档「legacy-error-prompt-inconsistent」）。
        let body = axum::Json(serde_json::json!({
            "type": "error",
            "message": self.response_message(),
            "date": legacy_date_now(),
            "spent": 0,
            "size": -1,
            "count": 0,
            "position": 0,
            "prompt": legacy_exception_for(prompt_kind),
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod unavailable_tests {
    use super::AppError;
    use axum::response::IntoResponse;

    /// R6：池满/建连超时映射 503 而不是 500——客户端可据此重试，
    /// 监控也能把「容量不足」与「代码缺陷」分开。
    #[test]
    fn unavailable_maps_to_503() {
        let resp = AppError::Unavailable("database pool timeout (Wait)".into()).into_response();
        assert_eq!(
            resp.status(),
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "池超时必须映射 503"
        );
    }
}

#[cfg(test)]
mod o23_tests {
    use super::{AppError, RequestContext, REQUEST_CONTEXT};
    use axum::response::IntoResponse;
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    /// 把 tracing 输出收进内存，用于断言 500 日志的字段。
    #[derive(Clone, Default)]
    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for BufferWriter {
        type Writer = BufferWriter;

        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    async fn body_json(resp: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("response body must be readable");
        serde_json::from_slice(&bytes).expect("error body must be JSON")
    }

    /// O23 断言：`internal_with_source` 必须保留原始 source（根因可追溯）。
    /// 破坏方式：换成 `AppError::Internal` 或把 cause 置空 → 此断言红。
    #[test]
    fn internal_with_source_keeps_root_cause() {
        let err = AppError::internal_with_source(
            "unit:42",
            anyhow::anyhow!("db connection refused: marker-xyz"),
        );
        match err {
            AppError::InternalWithSource { context, cause } => {
                assert_eq!(context, "unit:42");
                assert!(
                    cause.contains("marker-xyz"),
                    "原始 source 链必须保留根因，实际: {cause}"
                );
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    /// O23 断言：携带 source 的 500 对外仍是不泄密的固定文案。
    #[tokio::test]
    async fn internal_with_source_maps_500_without_leaking() {
        let resp = AppError::internal_with_source("unit:7", "secret-detail").into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let body = body_json(resp).await;
        assert_eq!(body["message"], "internal server error");
        assert!(
            !body.to_string().contains("secret-detail"),
            "响应体不能泄露内部 source"
        );
    }

    /// O23 断言：缺配置必须是 503 且写明缺哪一项——与代码缺陷 500 分桶。
    /// 破坏方式：改回 `AppError::Internal` → 状态码断言红。
    #[tokio::test]
    async fn missing_config_maps_503_and_names_item() {
        let resp = AppError::missing_config("ANDFX_KEY").into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::SERVICE_UNAVAILABLE);
        let body = body_json(resp).await;
        let message = body["message"].as_str().unwrap_or_default();
        assert!(
            message.contains("ANDFX_KEY"),
            "503 文案必须写明缺哪一项，实际: {message}"
        );
    }

    /// O23 断言：500 日志一条 ERROR，含 method/path/trace id 与 source。
    /// 破坏方式：日志里去掉 source 或上下文 → 此断言红。
    #[tokio::test]
    async fn five_hundred_logs_context_and_source() {
        let buffer = BufferWriter::default();
        let subscriber = tracing_subscriber::fmt()
            .with_writer(buffer.clone())
            .with_ansi(false)
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);

        let err = AppError::internal_with_source("unit:9", "marker-source");
        let ctx = RequestContext {
            method: "GET".to_string(),
            path: "/api/demo".to_string(),
            trace_id: Some("trace-123".to_string()),
        };
        let _resp = REQUEST_CONTEXT
            .scope(ctx, async move { err.into_response() })
            .await;

        let logged = String::from_utf8(buffer.0.lock().unwrap().clone()).expect("utf8 log");
        assert!(logged.contains("ERROR"), "必须是 ERROR 级别: {logged}");
        assert!(logged.contains("GET"), "日志缺 method: {logged}");
        assert!(logged.contains("/api/demo"), "日志缺 path: {logged}");
        assert!(logged.contains("trace-123"), "日志缺 trace id: {logged}");
        assert!(
            logged.contains("marker-source"),
            "日志缺原始 source 链: {logged}"
        );
        assert!(
            logged.contains("unit:9"),
            "日志缺 file:line 上下文: {logged}"
        );
    }
}
