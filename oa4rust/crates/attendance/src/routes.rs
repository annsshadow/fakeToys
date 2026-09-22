// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn attendance_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/attendance/admin/list/all", get(crate::list_admins))
        .route(
            "/api/attendance/employee/config/list/all",
            get(crate::list_employee_configs),
        )
        .route(
            "/api/attendance/statistical/cycle/list/all",
            get(crate::list_statistical_cycles),
        )
        .route(
            "/api/attendance/record/list",
            get(crate::list_check_in_records),
        )
        .route("/api/attendance/rule/list", get(crate::list_schedule_rules))
        .route(
            "/api/attendance/appeal/list",
            get(crate::list_appeal_records),
        )
        .route("/api/attendance/appeal/submit", post(crate::submit_appeal))
        .route("/api/attendance/appeal/audit", post(crate::audit_appeal))
        .route(
            "/api/attendance/appeal/archive/{id}",
            post(crate::archive_appeal),
        )
        .layer(axum::Extension(pool))
}
