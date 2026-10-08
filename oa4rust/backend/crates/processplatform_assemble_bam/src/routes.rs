// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::processplatform_assemble_bam_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    processplatform_assemble_bam_router().layer(axum::extract::Extension(pool))
}
