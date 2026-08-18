use axum::{routing::get, Router};
use deadpool_postgres::Pool;

use crate::{cache_detail, echo_get, openapi_info};

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/base/echo/get", get(echo_get))
        .route("/jaxrs/base/cache/detail", get(cache_detail))
        .route("/jaxrs/base/openapi/info", get(openapi_info))
        .layer(axum::Extension(pool))
}
