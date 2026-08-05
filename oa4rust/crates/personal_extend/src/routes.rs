use axum::{routing::get, Json, Router};
use deadpool_postgres::Pool;

use shared::session::SessionManager;

use crate::avatar;
use crate::password;
use crate::personal;

/// 构建 personal_extend 模块路由
///
/// 注册所有个人扩展接口，包括个人信息查询/更新、密码管理、头像上传/获取。
/// 使用认证中间件确保所有接口需要登录才能访问（health_check 除外）。
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `session_manager`: 会话管理器（由 main.rs 注入单一实例）
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn personal_extend_router(pool: Pool, session_manager: SessionManager) -> Router {
    Router::new()
        // 个人信息接口
        .route("/jaxrs/personal/info", get(personal::get_info))
        .route("/jaxrs/personal/update", axum::routing::put(personal::update_info))
        .route("/jaxrs/personal/detail/{id}", get(personal::get_detail))
        // 密码管理接口
        .route("/jaxrs/password/change", axum::routing::post(password::change))
        .route("/jaxrs/password/reset", axum::routing::post(password::reset))
        .route("/jaxrs/password/verify", axum::routing::post(password::verify))
        // 头像管理接口
        .route("/jaxrs/personal/avatar/upload", axum::routing::post(avatar::upload))
        .route("/jaxrs/personal/avatar/{id}", get(avatar::get_avatar))
        .layer(axum::extract::Extension(pool))
        .layer(axum::extract::Extension(session_manager))
}
