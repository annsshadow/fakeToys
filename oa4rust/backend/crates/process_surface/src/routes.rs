// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, Router};
use deadpool_postgres::Pool;

pub fn process_surface_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/process/list/ids", get(super::list_ids))
        .route("/api/process/{flag}", get(super::get_by_flag))
        .route(
            "/api/process/record/list/workorworkcompleted/{workOrWorkCompleted}",
            get(super::record_list),
        )
        .layer(Extension(pool))
}
