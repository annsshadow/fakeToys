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

    /// 四轮 A3-N3：会议审批（accept / confirm allow·deny）、邀请追加、改单此前**完全不取
    /// session**——任一登录用户可批准/驳回任意会议、以他人身份打卡、往任意会议塞受邀人，
    /// `create_meeting` 的 creator 还整个取请求体（可冒名 / 落 "system"）；accept/confirm/checkin
    /// 另有 GET 注册（csrf 只覆盖 POST/PUT/PATCH/DELETE，`requires_admin` 也只认这三个方法）。
    /// 修复后：写族一律要求有效会话，状态/归属类写先读 `x_meeting.creator` 再走 `require_owner`。
    ///
    /// 本族门禁全部需要真实数据库——归属门的第一步就是查行，离线只能拿到 500，
    /// 证不到 403，故标 `#[ignore]` 而非伪装成绿读数。跑真门：
    ///   cargo test --lib -p meeting_assemble_control -- --ignored
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn meeting_state_writes_forbidden_for_non_owner_non_admin() {
        let pool = shared::testing::test_pool();
        // 自包含造数：一条 creator 非本会话、且本会话非 admin 的会议行。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, applied, invited, creator) \
                 VALUES ('meeting-owned-by-other','t',NOW(),NOW(),false,false,'meeting-owner-elsewhere') \
                 ON CONFLICT (id) DO UPDATE SET creator = 'meeting-owner-elsewhere'",
                &[],
            )
            .await
            .unwrap();
        }
        let sess = build_session("u-not-the-creator");
        let mid = || axum::extract::Path("meeting-owned-by-other".to_string());

        let cases: [(&str, Result<(), AppError>); 7] = [
            (
                "save_meeting",
                crate::save_meeting(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    mid(),
                    axum::Json(serde_json::json!({ "title": "t" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "confirm_allow",
                crate::meeting_id_confirm_allow(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    mid(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "confirm_deny",
                crate::meeting_id_confirm_deny(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    mid(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "add_invite",
                crate::meeting_id_add_invite(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    mid(),
                    axum::Json(serde_json::json!({ "invitee": "u-victim" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "accept",
                crate::meeting_id_accept(Extension(pool.clone()), Extension(sess.clone()), mid())
                    .await
                    .map(|_| ()),
            ),
            (
                "reject",
                crate::meeting_id_reject(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    mid(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "manual_completed",
                crate::meeting_id_manual_completed(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    mid(),
                )
                .await
                .map(|_| ()),
            ),
        ];
        for (name, res) in cases {
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "{name} 非属主且非 admin 必须 403，实得 {res:?}"
            );
        }
        // 收口：清造数。
        pool.get()
            .await
            .unwrap()
            .execute("DELETE FROM x_meeting WHERE id='meeting-owned-by-other'", &[])
            .await
            .unwrap();
    }
}
