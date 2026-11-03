// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{routing::get, Router};
use deadpool_postgres::Pool;

use crate::{count, get_component, list_all};

pub fn component_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/component/list/all", get(list_all))
        .route("/api/component/count", get(count))
        .route("/api/component/{id}", get(get_component))
        // O9：此前 layer 被注释，三个 handler 的 Extension<Pool> 提取恒定 500
        .layer(axum::extract::Extension(pool))
}
