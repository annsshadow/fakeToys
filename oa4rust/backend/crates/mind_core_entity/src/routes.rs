// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

pub fn router(db: sea_orm::DatabaseConnection) -> axum::Router {
    crate::mind_core_entity_router(db)
}
