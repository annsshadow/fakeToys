use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn process_surface_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/process/list/ids", get(super::list_ids))
        .route("/jaxrs/process/{flag}", get(super::get_by_flag))
        .route("/jaxrs/process/record/list/workorworkcompleted/{workOrWorkCompleted}", get(super::record_list))
        .layer(Extension(pool))
}
