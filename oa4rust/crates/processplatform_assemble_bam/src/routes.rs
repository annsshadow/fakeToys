use axum::Router;

use crate::processplatform_assemble_bam_router;

/// 创建BAM装配路由
pub fn router() -> Router {
    processplatform_assemble_bam_router()
}
