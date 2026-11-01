// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use sea_orm::DatabaseConnection;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::jpush_core_entity_router(db)
}
