use axum::Router;
use deadpool_postgres::Pool;

use crate::query_core_express_router;

/// 创建查询核心Express路由
pub fn router(pool: Pool) -> axum::Router {
    query_core_express_router(pool)
}

