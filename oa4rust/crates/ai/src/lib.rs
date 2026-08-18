use axum::{
    extract::{Extension, Path},
    Json, routing::get,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;
mod config;
mod chat;
mod index;
mod file;
mod app;

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
pub use config::{
    config_get, config_base_config, config_list_model_paging, config_get_model,
    config_list_mcp_paging, config_get_mcp, list_enable_model,
};
pub use chat::{chat_list_paging, chat_list_completion_paging, chat_delete};
pub use index::{index_cms_doc, index_cms_doc_with_app, index_delete};
pub use file::{file_get, file_download, file_download_scale, file_delete};
pub use app::{sync_to_knowledge, app_list, model_list, conversation_list};
