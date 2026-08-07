use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{config_get, list_enable_model, sync_to_knowledge};

pub fn ai_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/config/get", get(config_get))
        .route("/jaxrs/ai/config/list/enable/model", get(list_enable_model))
        .route("/jaxrs/ai/index/sync/to/knowledge", get(sync_to_knowledge))
        .layer(Extension(pool))
}
