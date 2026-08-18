use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

use crate::{list_all, count, get_component};

pub fn component_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/component/list/all", get(list_all))
        .route("/jaxrs/component/count", get(count))
        .route("/jaxrs/component/{flag}", get(get_component))
        .layer(axum::extract::Extension(pool))
}
