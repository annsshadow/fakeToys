// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{routing::get, Router};
use deadpool_postgres::Pool;

use crate::{state_organization, state_running, state_summary};

pub fn process_bam_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/process/state/summary", get(state_summary))
        .route("/api/process/state/running", get(state_running))
        .route("/api/process/state/organization", get(state_organization))
        .layer(axum::Extension(pool))
}
