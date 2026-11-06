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
            matches!(r, Err(AppError::Unavailable(_))),
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
            matches!(r, Err(AppError::Unavailable(_))),
            "add_participant 有效会话应过门并落到 DB 失败"
        );
    }
}

/// 分片 A 读越权（IDOR）回归：`get_meeting`（GET `/api/meeting/{id}`）原无会话、
/// 无归属过滤，任一登录用户凭 id 可读他人会议。修复后归属谓词写在 SQL WHERE 内。
#[cfg(test)]
mod sec_read_a_tests {
    use axum::extract::Extension;
    use shared::{error::AppError, testing::test_pool};

    use crate::get_meeting;

    fn session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn meeting_get_idor_is_blocked_both_ways() {
        let pool = test_pool();
        {
            let c = pool.get().await.unwrap();
            for (id, title, creator) in [
                ("sec-read-a-meeting-own", "own-title", "u-sec-read-owner"),
                (
                    "sec-read-a-meeting-other",
                    "other-title",
                    "u-sec-read-other",
                ),
            ] {
                c.execute(
                    "INSERT INTO x_meeting (id, title, start_time, end_time, applied, invited, creator) \
                     VALUES ($1, $2, NOW(), NOW(), false, false, $3) \
                     ON CONFLICT (id) DO UPDATE SET creator = EXCLUDED.creator, title = EXCLUDED.title",
                    &[&id, &title, &creator],
                )
                .await
                .unwrap();
            }
        }
        let owner = session("u-sec-read-owner");
        let other = session("u-sec-read-other");
        let path = |id: &str| axum::extract::Path(id.to_string());

        // 正控制：本人读自己的会议 200 且真返回落库内容。
        let body = get_meeting(
            Extension(pool.clone()),
            Some(Extension(owner.clone())),
            path("sec-read-a-meeting-own"),
        )
        .await
        .expect("本人读自己的会议应成功");
        let data = body.0.data.expect("成功响应必须带 data");
        assert_eq!(data.get("id").and_then(|v| v.as_str()), Some("sec-read-a-meeting-own"));
        assert_eq!(data.get("title").and_then(|v| v.as_str()), Some("own-title"));

        // 双向：A→B 与 B→A 都必须 403。
        for (sess, id) in [
            (&owner, "sec-read-a-meeting-other"),
            (&other, "sec-read-a-meeting-own"),
        ] {
            let res = get_meeting(
                Extension(pool.clone()),
                Some(Extension(sess.clone())),
                path(id),
            )
            .await;
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "越权读 {id} 必须 403"
            );
        }

        // 无会话：401。
        let res = get_meeting(
            Extension(pool.clone()),
            None,
            path("sec-read-a-meeting-own"),
        )
        .await;
        assert!(matches!(res, Err(AppError::Unauthorized)));
    }
}
