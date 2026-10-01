// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, Router};

use crate::{get_control_config, list_control_sections};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    Router::new()
        .route(
            "/api/cms_control/get/control/config",
            get(get_control_config),
        )
        .route(
            "/api/cms_control/list/control/sections",
            get(list_control_sections),
        )
        .layer(Extension(pool))
}
