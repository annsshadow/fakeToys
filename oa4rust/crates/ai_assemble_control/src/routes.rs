use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/ai/assemble/control/config/get", get(super::get_ai_control_config))
        .route("/jaxrs/ai/assemble/control/models", get(super::list_ai_models))
        .route("/jaxrs/ai/assemble/control/config/update", get(super::update_ai_control_config))
        .route("/jaxrs/ai/assemble/control/usage/stats", get(super::get_usage_stats))
        .layer(Extension(pool))
}
