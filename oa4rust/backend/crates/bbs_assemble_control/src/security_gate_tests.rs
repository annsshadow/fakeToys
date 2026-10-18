// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第五十一批）：bbs 的写面 `update_control_config` 1 把。
//!
//! `update_control_config` 原本 `UPDATE x_bbs_assemble_control_config`（BBS 全局开关：是否允许匿名、
//! 最大版块数等），函数体不读会话、无门禁——尺子归成 R7「已认证无会话无门禁」：任何登录用户都能改写
//! 全站 BBS 配置。本批按本 crate 既定「无归属系统数据写 = 管理员档」口径收门（无会话 401 / 非管理员 403，
//! `is_admin` 对不可用库 fail-closed）。门在 `pool.get()` 之前，离线即证。
//!
//! bbs 其余 21 条 R7 均为「只读挂写方法」false-positive（list/filter/page/role 列表 SELECT）或鉴权原语
//! （login/logout 只动内存 TOKEN_STORE，不应加会话门），按口径不动。

#[cfg(test)]
mod security_gate_tests {
    use axum::extract::{Extension, Json};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn session(person: &str) -> Session {
        shared::testing::test_session(person)
    }

    // 无会话臂：连不上库的 pool + None 会话，1 把必须全 401——出现 Internal 即门插在 pool.get() 之后。
    #[tokio::test]
    async fn b51_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let r = crate::update_control_config(Extension(pool.clone()), None, Json(json!({}))).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "update_control_config 无会话必须 401（门须在取连接之前）：{:?}",
            r.err()
        );
    }

    // 非管理员臂：真库连通时，非管理员会话必须 403（is_admin 对不可用库 fail-closed，故库不可达时
    // 同返回 403，本臂仍成立），且不应落笔。
    #[tokio::test]
    async fn b51_update_control_config_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let r = crate::update_control_config(
            Extension(pool.clone()),
            Some(Extension(session("b51-nonadmin"))),
            Json(json!({"enabled": false})),
        )
        .await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "update_control_config 非管理员必须 403（含库不可达 fail-closed）：{:?}",
            r.err()
        );
    }
}
