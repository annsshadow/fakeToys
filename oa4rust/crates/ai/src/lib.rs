use deadpool_postgres::Pool;

mod app;
mod chat;
mod config;
mod file;
mod index;
mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn ai_router(pool: Pool) -> axum::Router {
    routes::ai_router(pool)
}

// Alias for backward compatibility with main.rs
pub fn router(pool: Pool) -> axum::Router {
    ai_router(pool)
}

// Re-export handlers for route registration
pub use app::{app_list, conversation_list, model_list, sync_to_knowledge};
pub use chat::{chat_delete, chat_list_completion_paging, chat_list_paging};
pub use config::{
    config_base_config, config_get, config_get_mcp, config_get_model, config_list_mcp_paging,
    config_list_model_paging, list_enable_model,
};
pub use file::{file_delete, file_download, file_download_scale, file_get};
pub use index::{index_cms_doc, index_cms_doc_with_app, index_delete};
