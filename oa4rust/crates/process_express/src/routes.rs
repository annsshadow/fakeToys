use axum::{extract::Extension, routing::get, Router};
use deadpool_postgres::Pool;

pub fn process_express_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/process/task/count/{credential}",
            get(super::task_count),
        )
        .route(
            "/api/process/read/count/{credential}",
            get(super::read_count),
        )
        .route(
            "/api/process/application/list",
            get(super::application_list),
        )
        .layer(Extension(pool))
}
