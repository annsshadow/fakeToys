use axum::{
    extract::Extension,
    routing::{get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    cancel_process_instance, create_process, execute_process,
    get_process, get_process_instance, list_processes,
    task_claim, task_complete, task_reject, task_transfer,
    work_complete, work_id_processing, work_start,
    gateway_join, gateway_fork,
    work_list, process_id_complex,
    work_v2_id_terminate, work_v2_id_retract,
    start_timer, cancel_timer, timer::TimerRegistry,
};

pub fn router(pool: Pool) -> Router {
    let timer = TimerRegistry::with_pool(pool.clone());
    timer.start_background();
    let timer_for_router = timer.clone();

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
        .route("/jaxrs/work/{id}/start", post(work_start))
        .route("/jaxrs/work/{id}/complete", post(work_complete))
        .route("/jaxrs/task/{id}/claim", post(task_claim))
        .route("/jaxrs/task/{id}/complete", post(task_complete))
        .route("/jaxrs/task/{id}/reject", post(task_reject))
        .route("/jaxrs/task/{id}/transfer/{person}", post(task_transfer))
        .route("/jaxrs/gateway/{work_id}/{activity_token}/join", post(gateway_join))
        .route("/jaxrs/processplatform/service/processing/gateway/fork/{gateway_instance_id}", post(gateway_fork))
        .route("/jaxrs/processplatform/service/processing/timer/start", post(start_timer))
        .route("/jaxrs/processplatform/service/processing/timer/{job_id}/cancel", post(cancel_timer))
        .layer(Extension(pool))
        .layer(Extension(timer_for_router))
}
