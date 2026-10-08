// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    express_group_list, express_identity_list, express_person_list, express_person_with_identity,
    express_person_with_unit, express_role_list, express_unit_list, get_express_info,
    list_express_companies, subscribe_express,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/express/query", get(get_express_info))
        .route("/api/express/companies", get(list_express_companies))
        .route("/api/express/subscribe", post(subscribe_express))
        // Batch query endpoints (R17-R24, no authentication required)
        .route("/api/express/person/list", post(express_person_list))
        .route("/api/express/unit/list", post(express_unit_list))
        .route("/api/express/identity/list", post(express_identity_list))
        .route("/api/express/group/list", post(express_group_list))
        .route("/api/express/role/list", post(express_role_list))
        .route(
            "/api/express/person/with/unit",
            post(express_person_with_unit),
        )
        .route(
            "/api/express/person/with/identity",
            post(express_person_with_identity),
        )
        .layer(Extension(pool))
}
