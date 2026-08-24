use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{cache_detail, echo_get, openapi_info};

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/base/echo/get", get(echo_get))
        .route("/jaxrs/base/echo", get(echo_get))
        .route("/jaxrs/base/cache/detail", get(cache_detail))
        .route("/jaxrs/base/openapi/info", get(openapi_info))
        // plan002 U2：Java 全集对齐（x_base_core_project jaxrs，补齐 5 条）
        .route("/jaxrs/base/cache", post(crate::cache_receive))
        .route("/jaxrs/base/cache/config/flush", get(crate::cache_config_flush))
        .route("/jaxrs/base/cache/commonscript/flush", get(crate::cache_commonscript_flush))
        .route("/jaxrs/base/fireschedule/classname/{className}", get(crate::fireschedule_execute))
        .route("/jaxrs/base/sysresource/filePath/{filePath}", get(crate::sysresource_list))
        .layer(axum::Extension(pool))
}
