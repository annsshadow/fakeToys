// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::query_express_router;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_express_router(pool.clone()).layer(axum::extract::Extension(pool))
}
