// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

pub async fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool)
}
