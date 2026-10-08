// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    clear_cache, config_create, config_delete, config_update, execute_command, get_logs,
    get_metric, get_status, get_system_info, send_message, server_deploy_create,
    server_deploy_delete, server_deploy_list, server_deploy_save,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/console/status", get(get_status))
        .route("/api/console/logs/{type}", get(get_logs))
        .route("/api/console/send/message", post(send_message))
        .route("/api/console/cache/clear/{type}", post(clear_cache))
        .route("/api/console/metric/{name}", get(get_metric))
        .route("/api/console/command/execute", post(execute_command))
        .route("/api/console/system/info", get(get_system_info))
        // ── config/server-deploy 斜杠路径家族（补齐 KNOWN_BACKEND_GAPS）──
        .route("/api/config/create", post(config_create))
        .route("/api/config/update/{id}", put(config_update))
        .route("/api/config/delete/{id}", delete(config_delete))
        .route("/api/server/deploy/list", get(server_deploy_list))
        .route("/api/server/deploy/create", post(server_deploy_create))
        .route("/api/server/deploy/save/{id}", put(server_deploy_save))
        .route("/api/server/deploy/save/{id}", post(server_deploy_save))
        .route(
            "/api/server/deploy/delete/{id}",
            delete(server_deploy_delete),
        )
        .route("/api/server/deploy/delete/{id}", post(server_deploy_delete))
        .layer(Extension(pool))
}
