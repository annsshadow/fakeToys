use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{app_list, config_get, conversation_list, list_enable_model, model_list, sync_to_knowledge};

pub fn ai_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/config/get", get(config_get))
        .route("/jaxrs/ai/config/list/enable/model", get(list_enable_model))
        .route("/jaxrs/ai/index/sync/to/knowledge", get(sync_to_knowledge))
        .route("/jaxrs/ai/app/list", get(app_list))
        .route("/jaxrs/ai/model/list", get(model_list))
        .route("/jaxrs/ai/conversation/list", get(conversation_list))
        .layer(Extension(pool))
}
