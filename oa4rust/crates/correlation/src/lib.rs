// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::Router;
use deadpool_postgres::Pool;

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn correlation_router(pool: Pool) -> Router {
    routes::correlation_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::correlation_router(pool)
}
