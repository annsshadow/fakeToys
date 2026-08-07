use axum::Router;

use crate::query_service_processing_router;

/// 创建查询服务处理路由
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_service_processing_router().layer(axum::extract::Extension(pool))
}

