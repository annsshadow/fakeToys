use axum::Router;

use crate::processplatform_assemble_designer_router;

/// 创建设计器装配路由
pub fn router() -> Router {
    processplatform_assemble_designer_router()
}
