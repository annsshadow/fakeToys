use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    clear_cache, execute_command, get_logs, get_metric, get_status,
    get_system_info, send_message,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/console/status", get(get_status))
        .route("/jaxrs/console/logs/{type}", get(get_logs))
        .route("/jaxrs/console/send/message", post(send_message))
        .route("/jaxrs/console/cache/clear/{type}", post(clear_cache))
        .route("/jaxrs/console/metric/{name}", get(get_metric))
        .route("/jaxrs/console/command/execute", post(execute_command))
        .route("/jaxrs/console/system/info", get(get_system_info))
        .layer(Extension(pool))
}
