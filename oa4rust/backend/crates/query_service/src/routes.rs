// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::Router;
use deadpool_postgres::Pool;

use crate::{neural_generate_model, neural_list_model, processing_execute};

pub fn build_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/api/query/service/neural/generate/{model_flag}",
            axum::routing::post(neural_generate_model),
        )
        .route(
            "/api/query/service/neural/list",
            axum::routing::get(neural_list_model),
        )
        .route(
            "/api/query/service/processing/execute",
            axum::routing::post(processing_execute),
        )
        .layer(axum::Extension(pool))
}
