// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::message_core_entity_router(pool)
}
