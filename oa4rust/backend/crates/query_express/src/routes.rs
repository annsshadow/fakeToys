// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::query_express_router;

pub fn router(db: sea_orm::DatabaseConnection) -> axum::Router {
    // O9：handler 取 Extension<DatabaseConnection>——此前这里 layer 的是
    // deadpool Pool（handler 不用），真正的 DatabaseConnection 从未注入，
    // /api/query/create|list 在生产恒定 500（§五 遗留 30 的修复）。
    query_express_router(db)
}
