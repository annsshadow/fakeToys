// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 批次 66 安全门禁回归：meeting 的两把真写处理 `create_meeting`（POST `/api/meeting/create`，
//! INSERT `x_meeting`）与 `add_participant`（POST `/api/meeting/{meetingId}/participant/add`，
//! INSERT `x_meeting_invite`）。两者原本不取会话、无门禁，属 R7 真写面。本批加 crate 内
//! `require_session` 会话存在档：门置于 `pool.get()` 之前，无会话先 401，不到库。
//!
//! 证明方式（离线、不依赖真实 DB）：
//! 1. `b66_writes_reject_without_session`：两把处理各以 dead pool + `None` 会话调用，断言
//!    `Err(AppError::Unauthorized)`（门在 DB 访问前返回）。
//! 2. `b66_writes_accept_valid_session_past_gate`：两把处理各以 dead pool + 有效会话调用，断言
//!    `Err(AppError::Internal)`（会话放行后抵达 `pool.get()`，dead pool 取连接失败 → Internal），
//!    证门在「库访问」之前、且有效会话能过门。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Json, Path};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::Value;
    use shared::{error::AppError, session::Session, testing::test_session};

    use crate::{add_participant, create_meeting};

    /// 构造一个「取连接必败」的 dead pool（空配置），用于离线证明门在 `pool.get()` 之前。
    fn dead_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    fn empty_json() -> Json<Value> {
        Json(Value::Object(serde_json::Map::new()))
    }

    fn valid_session() -> Option<Extension<Session>> {
        Some(Extension(test_session("b66-meeting")))
    }

    #[tokio::test]
    async fn b66_writes_reject_without_session() {
        // create_meeting 无会话 → 401
        let r = create_meeting(Extension(dead_pool()), None, empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "create_meeting 无会话应 401"
        );
        // add_participant 无会话 → 401
        let r = add_participant(
            Extension(dead_pool()),
            None,
            Path("b66-fake-meeting".to_string()),
            empty_json(),
        )
        .await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "add_participant 无会话应 401"
        );
    }

    #[tokio::test]
    async fn b66_writes_accept_valid_session_past_gate() {
        // 有效会话过门后抵达 pool.get()，dead pool 取连接失败 → Internal（证门在库访问前）
        let r = create_meeting(Extension(dead_pool()), valid_session(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Internal)),
            "create_meeting 有效会话应过门并落到 DB 失败"
        );
        let r = add_participant(
            Extension(dead_pool()),
            valid_session(),
            Path("b66-fake-meeting".to_string()),
            empty_json(),
        )
        .await;
        assert!(
            matches!(r, Err(AppError::Internal)),
            "add_participant 有效会话应过门并落到 DB 失败"
        );
    }
}
