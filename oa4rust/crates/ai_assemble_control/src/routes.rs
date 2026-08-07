use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_ai_control_config, list_ai_models, update_ai_control_config, get_usage_stats,
    config_list_mcp_paging_page_size_size,
    config_get_mcp_flag,
    config_create_mcp,
    config_update_mcp_flag,
    config_delete_mcp_flag,
    config_base_config,
    config_create_model,
    config_get_model_flag,
    config_delete_model_flag,
    config_get_mcp_ext_flag,
    config_list_enable_model,
    config_list_model_paging_page_size_size,
    config_save,
    config_update_model_flag,
    file_copy_file,
    file_delete_flag,
    file_list,
    file_list_paging_page_size_size,
    file_upload,
    file_flag,
    file_id_download,
    file_id_download_scale,
    index_cms_doc_with_app_appId,
    index_cms_doc_docId,
    index_delete_flag,
    index_list_paging_page_size_size,
    index_sync_to_knowledge,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route("/jaxrs/ai_assemble_control/get/ai/control/config", get(get_ai_control_config))
        .route("/jaxrs/ai_assemble_control/list/ai/models", get(list_ai_models))
        .route("/jaxrs/ai_assemble_control/update/ai/control/config", get(update_ai_control_config))
        .route("/jaxrs/ai_assemble_control/get/usage/stats", get(get_usage_stats))
        .route("/jaxrs/ai_assemble_control/config/list/mcp/paging/page/size/size", get(config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/config/get/mcp/flag", get(config_get_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/create/mcp", get(config_create_mcp))
        .route("/jaxrs/ai_assemble_control/config/update/mcp/flag", get(config_update_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/delete/mcp/flag", get(config_delete_mcp_flag))
        .route("/jaxrs/ai_assemble_control/config/base/config", get(config_base_config))
        .route("/jaxrs/ai_assemble_control/config/create/model", get(config_create_model))
        .route("/jaxrs/ai_assemble_control/config/get/model/flag", get(config_get_model_flag))
        .route("/jaxrs/ai_assemble_control/config/delete/model/flag", get(config_delete_model_flag))
        .route("/jaxrs/ai_assemble_control/config/get/mcp/ext/flag", get(config_get_mcp_ext_flag))
        .route("/jaxrs/ai_assemble_control/config/list/enable/model", get(config_list_enable_model))
        .route("/jaxrs/ai_assemble_control/config/list/model/paging/page/size/size", get(config_list_model_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/config/save", get(config_save))
        .route("/jaxrs/ai_assemble_control/config/update/model/flag", get(config_update_model_flag))
        .route("/jaxrs/ai_assemble_control/file/copy/file", get(file_copy_file))
        .route("/jaxrs/ai_assemble_control/file/delete/flag", get(file_delete_flag))
        .route("/jaxrs/ai_assemble_control/file/list", get(file_list))
        .route("/jaxrs/ai_assemble_control/file/list/paging/page/size/size", get(file_list_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/file/upload", get(file_upload))
        .route("/jaxrs/ai_assemble_control/file/flag", get(file_flag))
        .route("/jaxrs/ai_assemble_control/file/id/download", get(file_id_download))
        .route("/jaxrs/ai_assemble_control/file/id/download/scale", get(file_id_download_scale))
        .route("/jaxrs/ai_assemble_control/index/cms/doc/with/app/appId", get(index_cms_doc_with_app_appId))
        .route("/jaxrs/ai_assemble_control/index/cms/doc/docId", get(index_cms_doc_docId))
        .route("/jaxrs/ai_assemble_control/index/delete/flag", get(index_delete_flag))
        .route("/jaxrs/ai_assemble_control/index/list/paging/page/size/size", get(index_list_paging_page_size_size))
        .route("/jaxrs/ai_assemble_control/index/sync/to/knowledge", get(index_sync_to_knowledge))
        .layer(Extension(pool))
}

