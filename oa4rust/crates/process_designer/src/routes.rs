use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn process_designer_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/jaxrs/process/application/list/summary",
            get(super::application_list_summary),
        )
        .route(
            "/jaxrs/process/designer/route/{id}",
            get(super::designer_get_route),
        )
        .route(
            "/jaxrs/process/designer/application/list",
            get(super::application_list),
        )
        .route(
            "/jaxrs/process/designer/application/{id}",
            get(super::application_get),
        )
        .route(
            "/jaxrs/process/designer/application/create",
            post(super::application_create),
        )
        .route(
            "/jaxrs/process/designer/application/update",
            post(super::application_update),
        )
        .route(
            "/jaxrs/process/designer/application/remove",
            post(super::application_remove),
        )
        .layer(Extension(pool))
}
