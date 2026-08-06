use axum::{
    extract::Extension,
    routing::get, routing::post,
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    get_ai_control_config, list_ai_models, update_ai_control_config, get_usage_stats,
    stub_ai_assemble_control_config_list_mcp_paging_page_size_size,
    stub_ai_assemble_control_config_get_mcp_flag,
    stub_ai_assemble_control_config_create_mcp,
    stub_ai_assemble_control_config_update_mcp_flag,
    stub_ai_assemble_control_config_delete_mcp_flag,
    stub_ai_assemble_control_config_base_config,
    stub_ai_assemble_control_config_create_model,
    stub_ai_assemble_control_config_get_model_flag,
    stub_ai_assemble_control_config_delete_model_flag,
    stub_ai_assemble_control_config_get_mcp_ext_flag,
    stub_ai_assemble_control_config_list_enable_model,
    stub_ai_assemble_control_config_list_model_paging_page_size_size,
    stub_ai_assemble_control_config_save,
    stub_ai_assemble_control_config_update_model_flag,
    stub_ai_assemble_control_file_copy_file,
    stub_ai_assemble_control_file_delete_flag,
    stub_ai_assemble_control_file_list,
    stub_ai_assemble_control_file_list_paging_page_size_size,
    stub_ai_assemble_control_file_upload,
    stub_ai_assemble_control_file_flag,
    stub_ai_assemble_control_file_id_download,
    stub_ai_assemble_control_file_id_download_scale,
    stub_ai_assemble_control_index_cms_doc_with_app_appId,
    stub_ai_assemble_control_index_cms_doc_docId,
    stub_ai_assemble_control_index_delete_flag,
    stub_ai_assemble_control_index_list_paging_page_size_size,
    stub_ai_assemble_control_index_sync_to_knowledge,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/assemble/control/config/get", get(get_ai_control_config))
        .route("/jaxrs/ai/assemble/control/models", get(list_ai_models))
        .route("/jaxrs/ai/assemble/control/config/update", get(update_ai_control_config))
        .route("/jaxrs/ai/assemble/control/usage/stats", get(get_usage_stats))
        .route("/jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}", get(stub_ai_assemble_control_config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/config/get/mcp/{id}", get(stub_ai_assemble_control_config_get_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/create/mcp", post(stub_ai_assemble_control_config_create_mcp))
        .route("/jaxrs/ai/assemble/control/config/update/mcp/{id}", post(stub_ai_assemble_control_config_update_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/delete/mcp/{id}", post(stub_ai_assemble_control_config_delete_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/base/config", get(stub_ai_assemble_control_config_base_config))
        .route("/jaxrs/ai/assemble/control/config/create/model", post(stub_ai_assemble_control_config_create_model))
        .route("/jaxrs/ai/assemble/control/config/get/model/{id}", get(stub_ai_assemble_control_config_get_model_flag))
        .route("/jaxrs/ai/assemble/control/config/delete/model/{id}", post(stub_ai_assemble_control_config_delete_model_flag))
        .route("/jaxrs/ai/assemble/control/config/get/mcp/ext", get(stub_ai_assemble_control_config_get_mcp_ext_flag))
        .route("/jaxrs/ai/assemble/control/config/list/enable/model", get(stub_ai_assemble_control_config_list_enable_model))
        .route("/jaxrs/ai/assemble/control/config/list/model/paging/{page}/size/{size}", get(stub_ai_assemble_control_config_list_model_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/config/save", post(stub_ai_assemble_control_config_save))
        .route("/jaxrs/ai/assemble/control/config/update/model/{id}", post(stub_ai_assemble_control_config_update_model_flag))
        .route("/jaxrs/ai/assemble/control/file/copy", post(stub_ai_assemble_control_file_copy_file))
        .route("/jaxrs/ai/assemble/control/file/delete/{id}", post(stub_ai_assemble_control_file_delete_flag))
        .route("/jaxrs/ai/assemble/control/file/list", get(stub_ai_assemble_control_file_list))
        .route("/jaxrs/ai/assemble/control/file/list/paging/{page}/size/{size}", get(stub_ai_assemble_control_file_list_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/file/upload", post(stub_ai_assemble_control_file_upload))
        .route("/jaxrs/ai/assemble/control/file/{id}", get(stub_ai_assemble_control_file_flag))
        .route("/jaxrs/ai/assemble/control/file/{id}/download", get(stub_ai_assemble_control_file_id_download))
        .route("/jaxrs/ai/assemble/control/file/{id}/download/scale", get(stub_ai_assemble_control_file_id_download_scale))
        .route("/jaxrs/ai/assemble/control/index/cms/doc/with/app/{appId}", get(stub_ai_assemble_control_index_cms_doc_with_app_appId))
        .route("/jaxrs/ai/assemble/control/index/cms/doc/{docId}", get(stub_ai_assemble_control_index_cms_doc_docId))
        .route("/jaxrs/ai/assemble/control/index/delete/{id}", post(stub_ai_assemble_control_index_delete_flag))
        .route("/jaxrs/ai/assemble/control/index/list/paging/{page}/size/{size}", get(stub_ai_assemble_control_index_list_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/index/sync/to/knowledge", post(stub_ai_assemble_control_index_sync_to_knowledge))
        .layer(Extension(pool))
}
