// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! jpush 设备注册越权修复——`device_create` 的 userId 原取请求体，任一登录用户
//! 可替他人注册推送设备（推送劫持面）。修复后 user_id 恒取会话登录人。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use serde_json::json;

    use crate::device_create;

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    /// DB 门控：种真实行验证 user_id 取会话登录人（客户端 userId 被忽略）。
    #[tokio::test]
    async fn device_create_user_id_from_session() {
        if !shared::testing::is_db_available().await {
            eprintln!("SKIP (no PG): device_create_user_id_from_session");
            return;
        }
        let pool = shared::testing::test_pool();
        let client = pool.get().await.unwrap();

        // 客户端伪造 userId="u-impersonate"，会话为 u-owner → 落库必须 u-owner。
        let res = device_create(
            Extension(pool.clone()),
            Extension(build_session("u-owner")),
            axum::Json(json!({ "userId": "u-impersonate", "platform": "android", "token": "t" })),
        )
        .await;
        assert!(res.is_ok(), "device_create 属主注册应成功");

        let row = client
            .query_opt(
                "SELECT user_id FROM x_jpush_device WHERE user_id = 'u-owner' ORDER BY create_time DESC LIMIT 1",
                &[],
            )
            .await
            .unwrap();
        assert!(row.is_some(), "user_id 必须落库为会话登录人 u-owner");

        // 清理
        client
            .execute(
                "DELETE FROM x_jpush_device WHERE user_id = 'u-owner' AND platform = 'android' AND token = 't'",
                &[],
            )
            .await
            .unwrap();
    }
}
