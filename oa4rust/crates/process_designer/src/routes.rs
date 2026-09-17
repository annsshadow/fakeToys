use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn process_designer_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/process/application/list/summary",
            get(super::application_list_summary),
        )
        .route(
            "/api/process/designer/route/{id}",
            get(super::designer_get_route),
        )
        .route(
            "/api/process/designer/application/list",
            get(super::application_list),
        )
        .route(
            "/api/process/designer/application/{id}",
            get(super::application_get),
        )
        .route(
            "/api/process/designer/application/create",
            post(super::application_create),
        )
        .route(
            "/api/process/designer/application/update",
            post(super::application_update),
        )
        .route(
            "/api/process/designer/application/remove",
            post(super::application_remove),
        )
        .layer(Extension(pool))
}
