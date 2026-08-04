use axum::{Router, routing::get};

/// 构建快递查询模块路由
///
/// 注册一个 `/health` 健康检查端点，返回简单文本确认服务正常。
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn express_router() -> Router {
    Router::new().route("/health", get(|| async { "express: ok" }))
}
