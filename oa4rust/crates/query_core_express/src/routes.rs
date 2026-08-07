use axum::Router;

use crate::query_core_express_router;

/// 创建查询核心Express路由
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_core_express_router().layer(axum::extract::Extension(pool))
}

