use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    clear_cache, config_create, execute_command, get_logs, get_metric, get_status,
    get_system_info, send_message, server_deploy_create, server_deploy_delete,
    server_deploy_list, server_deploy_save,
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
        // ── config/server-deploy 斜杠路径家族（补齐 KNOWN_BACKEND_GAPS）──
        .route("/jaxrs/config/create", post(config_create))
        .route("/jaxrs/server/deploy/list", get(server_deploy_list))
        .route("/jaxrs/server/deploy/create", post(server_deploy_create))
        .route("/jaxrs/server/deploy/save/{id}", put(server_deploy_save))
        .route("/jaxrs/server/deploy/save/{id}", post(server_deploy_save))
        .route("/jaxrs/server/deploy/delete/{id}", delete(server_deploy_delete))
        .route("/jaxrs/server/deploy/delete/{id}", post(server_deploy_delete))
        .layer(Extension(pool))
}
