use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    cancel_process_instance, create_process, execute_process,
    get_process, get_process_instance, list_processes,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/service/processing/get/{id}", get(get_process))
        .route("/jaxrs/processplatform/service/processing/create", post(create_process))
        .route("/jaxrs/processplatform/service/processing/list/{category}", get(list_processes))
        .route("/jaxrs/processplatform/service/processing/execute/{id}", post(execute_process))
        .route("/jaxrs/processplatform/service/processing/instance/{executionId}", get(get_process_instance))
        .route("/jaxrs/processplatform/service/processing/cancel/{executionId}", post(cancel_process_instance))
        .layer(Extension(pool))
}
