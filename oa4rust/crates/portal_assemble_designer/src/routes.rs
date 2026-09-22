// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::portal_assemble_designer_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_designer_router().layer(axum::extract::Extension(pool))
}
