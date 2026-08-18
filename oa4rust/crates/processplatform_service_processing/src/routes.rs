use axum::{
    extract::Extension,
    routing::{get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    cancel_process_instance, create_process, execute_process,
    get_process, get_process_instance, list_processes,
    work_id_processing, work_v2_id_terminate, work_v2_id_retract,
    work_list, process_id_complex,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/processplatform/service/processing/get/{id}", get(get_process))
        .route("/jaxrs/processplatform/service/processing/create", post(create_process))
        .route("/jaxrs/processplatform/service/processing/list/{category}", get(list_processes))
        .route("/jaxrs/processplatform/service/processing/execute/{id}", post(execute_process))
        .route("/jaxrs/processplatform/service/processing/instance/{executionId}", get(get_process_instance))
        .route("/jaxrs/processplatform/service/processing/cancel/{executionId}", post(cancel_process_instance))
        .route("/jaxrs/work/{id}/processing", put(work_id_processing))
        .route("/jaxrs/work/{id}/terminate", post(work_v2_id_terminate))
        .route("/jaxrs/work/{id}/retract", post(work_v2_id_retract))
        .route("/jaxrs/processplatform/service/processing/work/list", get(work_list))
        .route("/jaxrs/processplatform/service/processing/process/{id}/complex", get(process_id_complex))
        .layer(Extension(pool))
}
