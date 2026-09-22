// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, Router};
use deadpool_postgres::Pool;

use crate::{exists_check, get_by_id, list_by_application_and_info_id};

pub fn hotpic_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/hotpic/user/hotpic/exists/check", get(exists_check))
        .route("/api/hotpic/user/hotpic/{id}", get(get_by_id))
        .route(
            "/api/hotpic/user/hotpic/{application}/{infoId}",
            get(list_by_application_and_info_id),
        )
        .layer(Extension(pool))
}
