use axum::{
    extract::Extension,
    routing::get,
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
            "/jaxrs/process/application/list",
            get(super::application_list),
        )
        .route(
            "/jaxrs/process/designer/route/:id",
            get(super::designer_get_route),
        )
        .layer(Extension(pool))
}
