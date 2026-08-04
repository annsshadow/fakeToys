use axum::Router;

use crate::query_core_express_router;

/// 创建查询核心Express路由
pub fn router() -> Router {
    query_core_express_router()
}
