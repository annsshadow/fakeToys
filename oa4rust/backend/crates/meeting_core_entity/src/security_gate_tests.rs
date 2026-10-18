// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! meeting_core_entity 会议/会议室写端点越权修复——
//! - 会议室（x_meeting_room 无属主列）create/update/delete 仅 admin；
//! - 会议（x_meeting 有 creator 列）update/delete 仅属主/admin。
//!
//! 此前中间件兜底 Authenticated，任一登录用户可物理删除/篡改任意会议与会议室。
//!
//! 无 DB 时 is_admin fail-closed 返回 false → 会议室写 403（不依赖数据库即可断言）。
//! 会议属主门禁需先查库取 creator，故用 DB 门控：有库时种真实会议行断言
//! 非属主 403、属主放行；无库时跳过（与 meeting_assemble_control u2 先例一致）。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use serde_json::json;
    use shared::error::AppError;

    use crate::{create_room, delete_meeting, delete_room, update_meeting, update_room};

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    fn mock_db() -> sea_orm::DatabaseConnection {
        sea_orm::DatabaseConnection::Disconnected
    }

    #[tokio::test]
    async fn room_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let db = mock_db();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                create_room(
                    Extension(db.clone()),
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(json!({"name": "r"}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_room 非 admin 必须 403"
        );
        assert!(
            matches!(
                update_room(
                    Extension(db.clone()),
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("r1".to_string()),
                    axum::Json(json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "update_room 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_room(
                    Extension(db.clone()),
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("r1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_room 非 admin 必须 403"
        );
    }

    /// 会议属主门禁：非属主 403，属主放行（admin 旁路因 is_admin 需查库，此处以属主
    /// 会话验证 owner 分支）。无库则跳过。
    #[tokio::test]
    async fn meeting_writes_require_owner() {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP (no PG): meeting_writes_require_owner");
            return;
        }
        let pool = shared::testing::test_pool();
        let db = mock_db();
        let client = pool.get().await.unwrap();

        let meeting_id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, creator) \
                 VALUES ($1, 'sec-gate', NOW(), NOW() + INTERVAL '1 hour', 'u-owner') \
                 ON CONFLICT (id) DO NOTHING",
                &[&meeting_id],
            )
            .await
            .unwrap();

        // 非属主、非 admin → 403
        let res = update_meeting(
            Extension(db.clone()),
            Extension(pool.clone()),
            Extension(build_session("u-other")),
            Path(meeting_id.clone()),
            axum::Json(json!({"title": "hijack"})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "update_meeting 非属主必须 403"
        );
        let res = delete_meeting(
            Extension(db.clone()),
            Extension(pool.clone()),
            Extension(build_session("u-other")),
            Path(meeting_id.clone()),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "delete_meeting 非属主必须 403"
        );

        // 属主 → 放行（不落 403）
        let res = update_meeting(
            Extension(db.clone()),
            Extension(pool.clone()),
            Extension(build_session("u-owner")),
            Path(meeting_id.clone()),
            axum::Json(json!({"title": "own"})),
        )
        .await;
        assert!(
            !matches!(res, Err(AppError::Forbidden)),
            "update_meeting 属主不得 403"
        );

        client
            .execute("DELETE FROM x_meeting WHERE id = $1", &[&meeting_id])
            .await
            .unwrap();
    }
}
