use axum::Router;

/// 构建快递查询模块路由
///
/// 注意：/health 已由 shared::router::router() 全局注册，此处不再重复注册。
pub fn express_router() -> Router {
    Router::new()
}
