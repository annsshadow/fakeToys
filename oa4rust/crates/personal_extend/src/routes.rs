use axum::{
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use shared::session::SessionManager;

use crate::avatar;
use crate::personal;

/// 构建 personal_extend 模块路由
///
/// 注册个人信息详情查询/更新与头像契约路径（Java PersonAction 契约）。
/// 头像契约路径：
/// - `PUT /jaxrs/person/icon`        上传当前用户头像（formData）
/// - `GET /jaxrs/person/icon`        获取当前用户头像
/// - `GET /jaxrs/icon/{person}`      获取指定用户头像（flag: unique_id/name/id）
///
/// 已移除自造路径：/jaxrs/password/change|reset|verify、/jaxrs/personal/avatar/*。
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
        .route(
            "/jaxrs/personal/update",
            axum::routing::put(personal::update_info),
        )
        .route("/jaxrs/personal/detail/{id}", get(personal::get_detail))
        // 头像接口（契约路径）
        .route("/jaxrs/person/icon", axum::routing::put(avatar::upload))
        .route("/jaxrs/person/icon", get(avatar::get_current_icon))
        .route("/jaxrs/icon/{person}", get(avatar::get_icon))
        // ── 签名 / 人脸 斜杠路径家族（补齐 KNOWN_BACKEND_GAPS）──
        .route(
            "/jaxrs/person/signature/save",
            axum::routing::post(personal::save_signature),
        )
        .route("/jaxrs/personal/face/list", get(personal::face_list))
        .route("/jaxrs/personal/face/create", post(personal::face_create))
        .route("/jaxrs/personal/face/save/{id}", put(personal::face_save))
        .route("/jaxrs/personal/face/save/{id}", post(personal::face_save))
        .route("/jaxrs/personal/face/delete/{id}", delete(personal::face_delete))
        .route("/jaxrs/personal/face/delete/{id}", post(personal::face_delete))
        .layer(axum::extract::Extension(pool))
        .layer(axum::extract::Extension(session_manager))
}
