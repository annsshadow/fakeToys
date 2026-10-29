// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use deadpool_postgres::Pool;
use sea_orm::DatabaseConnection;

pub fn router(db: DatabaseConnection, pool: Pool) -> axum::Router {
    crate::meeting_core_entity_router(db, pool)
}
