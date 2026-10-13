// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! `config_system_config_manage`（POST /config/system/config/manage）原为匿名可写——
//! 路由前缀 /config/system 在 o2server 对齐豁免清单中（未登录读），但写 handler
//! 无会话/无 admin 门禁，匿名网络调用即可 UPDATE x_meeting_config（会议模块全局
//! 配置：enableOnline / meetingViewer / 视图名等）。修复后：未认证 401、非 admin 403。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::Json;
    use serde_json::json;
    use shared::error::AppError;

    use crate::config_system_config_manage;

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn config_manage_requires_session() {
        let pool = shared::testing::test_pool();
        let res =
            config_system_config_manage(Extension(pool), None, Json(json!({"configKey": "k", "configValue": "v"}))).await;
        assert!(
            matches!(res, Err(AppError::Unauthorized)),
            "无会话调用会议配置写必须 401"
        );
    }

    #[tokio::test]
    async fn config_manage_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let res = config_system_config_manage(
            Extension(pool),
            Some(Extension(build_session("u-nonadmin"))),
            Json(json!({"configKey": "k", "configValue": "v"})),
        )
        .await;
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 会话调用会议配置写必须 403（无 DB 时 is_admin fail-closed）"
        );
    }
}
