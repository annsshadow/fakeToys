use axum::{extract::Extension, routing::get, routing::post, Router};
use deadpool_postgres::Pool;

use crate::{device_create, device_get, device_list, hello, template_get, template_list};

pub fn jpush_router(pool: Pool) -> Router {
    Router::new()
        .route("/hello/world", get(hello))
        .route("/api/jpush/device/list", get(device_list))
        .route("/api/jpush/device/{id}", get(device_get))
        .route("/api/jpush/device/create", post(device_create))
        .route("/api/jpush/template/list", get(template_list))
        .route("/api/jpush/template/{id}", get(template_get))
        .layer(Extension(pool))
}
