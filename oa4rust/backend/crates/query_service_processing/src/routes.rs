// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use deadpool_postgres::Pool;

use crate::query_service_processing_router;

/// 创建查询服务处理路由
pub fn router(pool: Pool) -> axum::Router {
    query_service_processing_router(pool)
}
