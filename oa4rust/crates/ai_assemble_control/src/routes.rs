use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};

use crate::{
    // 既有扩展端点（非 Java 对应，保留向后兼容）
    get_ai_control_config, list_ai_models, update_ai_control_config, get_usage_stats,
    // config（Java ConfigAction，15 端点）
    config_get,
    config_base_config,
    config_save,
    config_list_model_paging_page_size_size,
    config_create_model,
    config_update_model_flag,
    config_get_model_flag,
    config_delete_model_flag,
    config_list_mcp_paging_page_size_size,
    config_create_mcp,
    config_update_mcp_flag,
    config_get_mcp_flag,
    config_get_mcp_ext_flag,
    config_delete_mcp_flag,
    config_list_enable_model,
    // file（Java FileAction，8 端点）
    file_flag,
    file_upload,
    file_copy_file,
    file_id_download,
    file_id_download_scale,
    file_list_paging_page_size_size,
    file_delete_flag,
    file_list_with_ids,
    // index（Java IndexAction，5 端点）
    index_cms_doc_docId,
    index_cms_doc_with_app_appId,
    index_delete_flag,
    index_list_paging_page_size_size,
    index_sync_to_knowledge,
    // chat（Java ChatAction，5 端点 + 流式扩展）
    chat_completion,
    chat_completion_stream,
    chat_list_paging_page_size_size,
    chat_list_completion_clue_id_paging_page_size_size,
    chat_delete_clue_id,
    chat_write_completion_extra,
};

/// Java x_ai_assemble_control jaxrs 全量端点对齐表。
///
/// 类级 @Path 拼接方法级 @Path 后的 33 个唯一端点全部注册，
/// 动词与路径段与 Java 注解一一对应：
///   - Java {flag}/{id}/{docId}/{appId}/{clueId} → 同名参数段
///     （file 的 {flag} 与 {id} 在同一位置，统一为 {flag} 以避免参数名冲突）
///   - POST /chat/completion/stream 为本 crate 既有 SSE 扩展端点
///   - /jaxrs/ai_assemble_control/{get,list,update}/... 与 get/usage/stats
///     为本 crate 早期扩展端点，保留兼容
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        // ── chat：Java ChatAction ────────────────────────────────────────────
        .route("/jaxrs/ai_assemble_control/chat/completion", post(chat_completion))
        .route("/jaxrs/ai_assemble_control/chat/completion/stream", post(chat_completion_stream))
        .route("/jaxrs/ai_assemble_control/chat/list/paging/{page}/size/{size}", get(chat_list_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/chat/list/completion/{clueId}/paging/{page}/size/{size}", get(chat_list_completion_clue_id_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/chat/delete/{clueId}", get(chat_delete_clue_id))
        .route("/jaxrs/ai_assemble_control/chat/write/completion/extra", post(chat_write_completion_extra))
        // ── config：Java ConfigAction ────────────────────────────────────────
        .route("/jaxrs/ai_assemble_control/config/get", get(config_get))
        .route("/jaxrs/ai_assemble_control/config/base/config", get(config_base_config))
        .route("/jaxrs/ai_assemble_control/config/save", post(config_save))
        .route("/jaxrs/ai_assemble_control/config/list/model/paging/{page}/size/{size}", get(config_list_model_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/config/create/model", post(config_create_model))
        .route("/jaxrs/ai_assemble_control/config/update/model/{flag}", post(config_update_model_flag))
        .route("/jaxrs/ai_assemble_control/config/get/model/{flag}", get(config_get_model_flag))
        .route("/jaxrs/ai_assemble_control/config/delete/model/{flag}", get(config_delete_model_flag))
        .route("/jaxrs/ai_assemble_control/config/list/mcp/paging/{page}/size/{size}", get(config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/config/create/mcp", post(config_create_mcp))
        .route("/jaxrs/ai_assemble_control/config/update/mcp/{flag}", post(config_update_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/get/mcp/{flag}", get(config_get_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/get/mcp/ext/{flag}", get(config_get_mcp_ext_flag))
        .route("/jaxrs/ai_assemble_control/config/delete/mcp/{flag}", get(config_delete_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/list/enable/model", get(config_list_enable_model))
        // ── file：Java FileAction ────────────────────────────────────────────
        .route("/jaxrs/ai_assemble_control/file/{flag}", get(file_flag))
        .route("/jaxrs/ai_assemble_control/file/upload", post(file_upload))
        .route("/jaxrs/ai_assemble_control/file/copy/file", post(file_copy_file))
        .route("/jaxrs/ai_assemble_control/file/{flag}/download", get(file_id_download))
        .route("/jaxrs/ai_assemble_control/file/{flag}/download/scale", get(file_id_download_scale))
        .route("/jaxrs/ai_assemble_control/file/list/paging/{page}/size/{size}", post(file_list_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/file/delete/{flag}", get(file_delete_flag))
        .route("/jaxrs/ai_assemble_control/file/list", post(file_list_with_ids))
        // ── index：Java IndexAction ──────────────────────────────────────────
        .route("/jaxrs/ai_assemble_control/index/cms/doc/{docId}", get(index_cms_doc_docId))
        .route("/jaxrs/ai_assemble_control/index/cms/doc/with/app/{appId}", get(index_cms_doc_with_app_appId))
        .route("/jaxrs/ai_assemble_control/index/delete/{flag}", get(index_delete_flag))
        .route("/jaxrs/ai_assemble_control/index/list/paging/{page}/size/{size}", post(index_list_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/index/sync/to/knowledge", get(index_sync_to_knowledge))
        // ── 本 crate 既有扩展端点（保留） ───────────────────────────────────
        .route("/jaxrs/ai_assemble_control/get/ai/control/config", get(get_ai_control_config))
        .route("/jaxrs/ai_assemble_control/list/ai/models", get(list_ai_models))
        .route("/jaxrs/ai_assemble_control/update/ai/control/config", get(update_ai_control_config))
        .route("/jaxrs/ai_assemble_control/get/usage/stats", get(get_usage_stats))
        // ── /jaxrs/ai/assemble/control 别名前缀（保留） ─────────────────────
        .route("/jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}", get(config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/config/create/mcp", post(config_create_mcp))
        .route("/jaxrs/ai/assemble/control/config/get/mcp/{id}", get(config_get_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/update/mcp/{id}", post(config_update_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/delete/mcp/{id}", post(config_delete_mcp_flag))
        .layer(Extension(pool))
}
