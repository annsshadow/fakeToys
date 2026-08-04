use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn ai_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/config/get", get(super::config_get))
        .route("/jaxrs/ai/config/list/enable/model", get(super::list_enable_model))
        .route("/jaxrs/ai/index/sync/to/knowledge", get(super::sync_to_knowledge))
        .layer(Extension(pool))
}
