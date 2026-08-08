use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    app_list, chat_delete, chat_list_completion_paging, chat_list_paging, config_base_config,
    config_get, config_get_model, config_get_mcp, config_list_mcp_paging, config_list_model_paging,
    conversation_list, file_delete, file_download, file_download_scale, file_get, index_cms_doc,
    index_cms_doc_with_app, index_delete, list_enable_model, model_list, sync_to_knowledge,
};

pub fn ai_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/config/get", get(config_get))
        .route("/jaxrs/ai/config/base/config", get(config_base_config))
        .route("/jaxrs/ai/config/list/model/paging/{page}/size/{size}", get(config_list_model_paging))
        .route("/jaxrs/ai/config/get/model/{flag}", get(config_get_model))
        .route("/jaxrs/ai/config/list/mcp/paging/{page}/size/{size}", get(config_list_mcp_paging))
        .route("/jaxrs/ai/config/get/mcp/{flag}", get(config_get_mcp))
        .route("/jaxrs/ai/config/list/enable/model", get(list_enable_model))
        .route("/jaxrs/ai/index/sync/to/knowledge", get(sync_to_knowledge))
        .route("/jaxrs/ai/app/list", get(app_list))
        .route("/jaxrs/ai/model/list", get(model_list))
        .route("/jaxrs/ai/conversation/list", get(conversation_list))
        .route("/jaxrs/ai/chat/list/paging/{page}/size/{size}", get(chat_list_paging))
        .route("/jaxrs/ai/chat/list/completion/{clueId}/paging/{page}/size/{size}", get(chat_list_completion_paging))
        .route("/jaxrs/ai/chat/delete/{clueId}", get(chat_delete))
        .route("/jaxrs/ai/index/cms/doc/{docId}", get(index_cms_doc))
        .route("/jaxrs/ai/index/cms/doc/with/app/{appId}", get(index_cms_doc_with_app))
        .route("/jaxrs/ai/index/delete/{flag}", get(index_delete))
        .route("/jaxrs/ai/file/{flag}", get(file_get))
        .route("/jaxrs/ai/file/{id}/download", get(file_download))
        .route("/jaxrs/ai/file/{id}/download/scale", get(file_download_scale))
        .route("/jaxrs/ai/file/delete/{flag}", get(file_delete))
        .layer(axum::extract::Extension(pool))
}
