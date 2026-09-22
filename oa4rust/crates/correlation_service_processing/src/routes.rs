// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::correlation_service_processing_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    correlation_service_processing_router().layer(axum::extract::Extension(pool))
}
