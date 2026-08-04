use axum::Router;

use crate::{count, get_component, list_all};

pub fn component_router() -> Router {
    Router::new()
        .route("/jaxrs/component/list/all", axum::routing::get(list_all))
        .route("/jaxrs/component/count", axum::routing::get(count))
        .route("/jaxrs/component/{flag}", axum::routing::get(get_component))
}
