use axum::{routing::get, Router};
use deadpool_postgres::Pool;

use crate::{count, get_component, list_all};

pub fn component_router(_pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/component/list/all", get(list_all))
        .route("/jaxrs/component/count", get(count))
        .route("/jaxrs/component/{id}", get(get_component))
    // .layer(axum::extract::Extension(pool))
}
