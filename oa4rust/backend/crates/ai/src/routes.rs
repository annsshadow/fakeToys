// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{routing::get, Router};
use deadpool_postgres::Pool;

use crate::{
    app_list, chat_delete, chat_list_completion_paging, chat_list_paging, config_base_config,
    config_get, config_get_model, config_list_model_paging, conversation_list, file_delete,
    file_download, file_download_scale, file_get, index_cms_doc, index_cms_doc_with_app,
    index_delete, list_enable_model, model_list, sync_to_knowledge,
};

pub fn ai_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/ai/config/get", get(config_get))
        .route("/api/ai/config/base/config", get(config_base_config))
        .route(
            "/api/ai/config/list/model/paging/{page}/size/{size}",
            get(config_list_model_paging),
        )
        .route("/api/ai/config/get/model/{flag}", get(config_get_model))
        // O20：/api/ai/config/list|get/mcp 曾挂在本 crate 的空桩（返回空数组 /
        // "mcp not found"），前端 AIChatApp 实际调用的是这条旧词路径。真实现
        // 早已存在（ai_assemble_control/src/lib.rs:971/794），故旧词路由改挂
        // 在 ai_assemble_control::router，见该 crate routes.rs 末尾别名段。
        .route("/api/ai/config/list/enable/model", get(list_enable_model))
        .route("/api/ai/index/sync/to/knowledge", get(sync_to_knowledge))
        .route("/api/ai/app/list", get(app_list))
        .route("/api/ai/model/list", get(model_list))
        .route("/api/ai/conversation/list", get(conversation_list))
        .route(
            "/api/ai/chat/list/paging/{page}/size/{size}",
            get(chat_list_paging),
        )
        .route(
            "/api/ai/chat/list/completion/{clue_id}/paging/{page}/size/{size}",
            get(chat_list_completion_paging),
        )
        .route("/api/ai/chat/delete/{clue_id}", get(chat_delete))
        .route("/api/ai/index/cms/doc/{docId}", get(index_cms_doc))
        .route(
            "/api/ai/index/cms/doc/with/app/{appId}",
            get(index_cms_doc_with_app),
        )
        .route("/api/ai/index/delete/{flag}", get(index_delete))
        .route("/api/ai/file/{flag}", get(file_get))
        .route("/api/ai/file/{id}/download", get(file_download))
        .route("/api/ai/file/{id}/download/scale", get(file_download_scale))
        .route("/api/ai/file/delete/{flag}", get(file_delete))
        .layer(axum::extract::Extension(pool))
}
