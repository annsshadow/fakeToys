// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::query_assemble_surface_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_surface_router().layer(axum::extract::Extension(pool))
}
