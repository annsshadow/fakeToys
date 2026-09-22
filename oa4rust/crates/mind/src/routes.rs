// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{routing::delete, routing::get, routing::post, Router};

pub fn mind_routes() -> Router {
    Router::new()
        .route("/api/mind/mind/{id}", get(crate::get_mind_with_id))
        .route("/api/mind/mind", post(crate::create_mind))
        .route("/api/mind/mind/{id}", post(crate::update_mind))
        .route("/api/mind/mind/{id}", delete(crate::delete_mind))
        .route("/api/mind/folder/tree/my", get(crate::list_my_folders))
        .route("/api/mind/folder", post(crate::create_folder))
        .route("/api/mind/folder/{id}", post(crate::update_folder))
        .route("/api/mind/folder/{id}", delete(crate::delete_folder))
        .route(
            "/api/mind/mind/list/{id}/version",
            get(crate::list_versions_with_mind_id),
        )
        .route("/api/mind/version", post(crate::create_version))
        .fallback(axum::routing::any(|| async { "not found" }))
}
