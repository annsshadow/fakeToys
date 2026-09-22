// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{cache_detail, echo_get, openapi_info};

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/base/echo/get", get(echo_get))
        .route("/api/base/echo", get(echo_get))
        .route("/api/base/cache/detail", get(cache_detail))
        .route("/api/base/openapi/info", get(openapi_info))
        // plan002 U2：o2server 全集对齐（x_base_core_project o2server，补齐 5 条）
        .route("/api/base/cache", post(crate::cache_receive))
        .route(
            "/api/base/cache/config/flush",
            get(crate::cache_config_flush),
        )
        .route(
            "/api/base/cache/commonscript/flush",
            get(crate::cache_commonscript_flush),
        )
        .route(
            "/api/base/fireschedule/classname/{className}",
            get(crate::fireschedule_execute),
        )
        .route(
            "/api/base/sysresource/filePath/{filePath}",
            get(crate::sysresource_list),
        )
        .layer(axum::Extension(pool))
}
