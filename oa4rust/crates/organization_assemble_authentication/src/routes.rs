use axum::Router;
use deadpool_postgres::Pool;
use shared::session::SessionManager;

use crate::organization_assemble_authentication_router;

pub fn router(pool: Pool) -> Router {
    // 会话管理器按库构建（auth_session 表为共享存储），保证本 crate 内
    // 需要 SessionManager 扩展的处理器（扫码登录、绑定确认等）可用。
    let session_manager = SessionManager::with_pool(pool.clone());
    organization_assemble_authentication_router()
        .layer(axum::extract::Extension(session_manager))
        .layer(axum::extract::Extension(pool))
}
