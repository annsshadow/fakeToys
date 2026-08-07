use axum::Router;
use deadpool_postgres::Pool;

use crate::{neural_generate_model, neural_list_model};

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/jaxrs/query/service/neural/generate/{model_flag}",
            axum::routing::post(neural_generate_model),
        )
        .route(
            "/jaxrs/query/service/neural/list",
            axum::routing::get(neural_list_model),
        )
        .layer(axum::Extension(pool))
}
