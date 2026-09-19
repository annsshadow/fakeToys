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
/// 注册个人信息详情查询/更新与头像契约路径（o2server PersonAction 契约）。
/// 头像契约路径：
/// - `PUT /api/person/icon`        上传当前用户头像（formData）
/// - `GET /api/person/icon`        获取当前用户头像
/// - `GET /api/icon/{person}`      获取指定用户头像（flag: unique_id/name/id）
///
/// 已移除自造路径：/api/password/change|reset|verify、/api/personal/avatar/*。
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
        .route("/api/personal/info", get(personal::get_info))
        .route(
            "/api/personal/update",
            axum::routing::put(personal::update_info),
        )
        .route("/api/personal/detail/{id}", get(personal::get_detail))
        // 头像接口（契约路径）
        .route("/api/person/icon", axum::routing::put(avatar::upload))
        .route("/api/person/icon", get(avatar::get_current_icon))
        .route("/api/icon/{person}", get(avatar::get_icon))
        // ── 签名 / 人脸 斜杠路径家族（补齐 KNOWN_BACKEND_GAPS）──
        .route(
            "/api/person/signature/save",
            axum::routing::post(personal::save_signature),
        )
        .route("/api/personal/face/list", get(personal::face_list))
        .route("/api/personal/face/create", post(personal::face_create))
        .route("/api/personal/face/save/{id}", put(personal::face_save))
        .route("/api/personal/face/save/{id}", post(personal::face_save))
        .route(
            "/api/personal/face/delete/{id}",
            delete(personal::face_delete),
        )
        .route(
            "/api/personal/face/delete/{id}",
            post(personal::face_delete),
        )
        .layer(axum::extract::Extension(pool))
        .layer(axum::extract::Extension(session_manager))
}
