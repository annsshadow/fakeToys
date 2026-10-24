// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 2026-10-24：随 lib.rs 的空转 SeaORM 池删除一并改同步——本 crate 不再
// 需要 await 任何东西（5 个子 router 全为空，见 lib.rs 注释）。
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_core_entity_router(pool)
}
