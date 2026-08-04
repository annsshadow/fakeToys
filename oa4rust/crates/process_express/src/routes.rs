use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn process_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/process/task/count/:credential", get(super::task_count))
        .route("/jaxrs/process/read/count/:credential", get(super::read_count))
        .route("/jaxrs/process/application/list", get(super::application_list))
        .layer(Extension(pool))
}
