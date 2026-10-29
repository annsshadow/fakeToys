// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! `config_system_config_manage`（POST /config/system/config/manage）原为匿名可写——
//! 路由前缀 /config/system 在 o2server 对齐豁免清单中（未登录读），但写 handler
//! 无会话/无 admin 门禁，匿名网络调用即可 UPDATE x_meeting_config（会议模块全局
//! 配置：enableOnline / meetingViewer / 视图名等）。修复后：未认证 401、非 admin 403。
#[cfg(test)]
#[allow(clippy::module_inception)]
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
        let res = config_system_config_manage(
            Extension(pool),
            None,
            Json(json!({"configKey": "k", "configValue": "v"})),
        )
        .await;
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
            .execute(
                "DELETE FROM x_meeting WHERE id='meeting-owned-by-other'",
                &[],
            )
            .await
            .unwrap();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 第五十四批（2026-10 四轮）：meeting_assemble_control 剩余写面 4 把。
//
// - 控制配置新增 `create_meeting_control`（INSERT x_meeting_assemble_control）：
//   与既有 `delete_meeting_control` 同档，收 admin（无会话 401 / 非 admin 403）；
// - 会议改单三把 `meeting_id_delete_invite`（DELETE x_meeting_invite）、
//   `meeting_id_modify_completedtime` / `meeting_id_modify_starttime`（UPDATE x_meeting）：
//   收会议属主或 admin（`u2_meeting_owner_gate`，按 `x_meeting.creator`；无会话 401）。
// 余下 `meeting_id_checkin_code`（SELECT）与 5 把 `meeting_list_*`（SELECT 分页）为只读
// false-positive，按口径不动。
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod b54_gate_tests {
    use axum::extract::{Extension, Path};
    use axum::Json;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> shared::session::Session {
        shared::testing::test_session("b54-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，4 把必须全 401（门在 pool.get() 之前）。
    #[tokio::test]
    async fn b54_r7_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "create_meeting_control",
                crate::create_meeting_control(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "meetingId": "m1", "controlType": "c" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_delete_invite",
                crate::meeting_id_delete_invite(
                    Extension(pool.clone()),
                    None,
                    Path("m1".to_string()),
                    Json(json!({ "invitee": "u" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_modify_completedtime",
                crate::meeting_id_modify_completedtime(
                    Extension(pool.clone()),
                    None,
                    Path("m1".to_string()),
                    Json(json!({ "completedTime": "2026-01-01 00:00:00" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_modify_starttime",
                crate::meeting_id_modify_starttime(
                    Extension(pool.clone()),
                    None,
                    Path("m1".to_string()),
                    Json(json!({ "startTime": "2026-01-01 00:00:00" })),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 4, "本批 4 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// admin 臂：控制配置新增非管理员 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b54_create_control_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        let res = crate::create_meeting_control(
            Extension(pool),
            Some(Extension(non_admin())),
            Json(json!({ "meetingId": "m1", "controlType": "c" })),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "非 admin 新增控制配置必须 403，实得 {res:?}"
        );
    }

    /// 归属臂（需真实 PostgreSQL）：会议属主非本会话且本会话非 admin → 改单三把 403。
    #[tokio::test]
    #[ignore = "需真实 PostgreSQL（DATABASE_URL）"]
    async fn b54_owner_writes_non_owner_forbidden() {
        let pool = shared::testing::test_pool();
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO x_meeting (id, title, start_time, end_time, applied, invited, creator) \
                 VALUES ('b54-meeting-owned-by-other','t',NOW(),NOW(),false,false,'b54-owner-elsewhere') \
                 ON CONFLICT (id) DO UPDATE SET creator = 'b54-owner-elsewhere'",
                &[],
            )
            .await
            .unwrap();
        }
        let sess = non_admin();
        let mid = || Path("b54-meeting-owned-by-other".to_string());
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "meeting_id_delete_invite",
                crate::meeting_id_delete_invite(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    mid(),
                    Json(json!({ "invitee": "u" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_modify_completedtime",
                crate::meeting_id_modify_completedtime(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    mid(),
                    Json(json!({ "completedTime": "2026-01-01 00:00:00" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_modify_starttime",
                crate::meeting_id_modify_starttime(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    mid(),
                    Json(json!({ "startTime": "2026-01-01 00:00:00" })),
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
        pool.get()
            .await
            .unwrap()
            .execute(
                "DELETE FROM x_meeting WHERE id='b54-meeting-owned-by-other'",
                &[],
            )
            .await
            .unwrap();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// A2-R6 批：meeting_assemble_control 已认证+读会话无门禁的写路由补 require_session 门。
//
// 覆盖 6 把 handler（TSV 行 63-69）：
//   u2_attachment_create_from_processplatform / u2_attachment_upload /
//   u2_attachment_upload_callback / create_meeting / meeting_id_checkin /
//   u2_room_set_photo
// 判据：无会话（路由层不注入 Session 扩展）→ 401，门在首个 DB 访问之前，
// 死池离线可证（旧行为：门缺失时 pool.get() 失败 → 500）。
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
#[allow(clippy::module_inception)]
mod r6_gate_tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;
    use tower::ServiceExt;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn multipart_body() -> Body {
        Body::from(
            "--xboundary\r\n\
             Content-Disposition: form-data; name=\"file\"; filename=\"a.txt\"\r\n\
             Content-Type: text/plain\r\n\r\nhello\r\n--xboundary--\r\n",
        )
    }

    /// 直调 handler（Json 族）：无会话必须 401，且发生在 pool.get() 之前。
    #[tokio::test]
    async fn r6_json_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "create_meeting",
                crate::create_meeting(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Json(json!({
                        "title": "t", "content": "c",
                        "startTime": "2026-01-01 00:00:00",
                        "endTime": "2026-01-01 01:00:00",
                    })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "meeting_id_checkin",
                crate::meeting_id_checkin(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Path("m1".to_string()),
                    axum::extract::Json(json!({})),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2_attachment_create_from_processplatform",
                crate::u2_attachment_create_from_processplatform(
                    Extension(pool.clone()),
                    None,
                    axum::extract::Json(json!({ "meetingId": "m1", "title": "doc" })),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 3, "本批 Json 族 3 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// 路由层（multipart 族 + Json 族）：匿名 POST 必须 401，而非 500/404。
    #[tokio::test]
    async fn r6_routes_reject_anonymous_writes() {
        let app = crate::router(dead_pool());
        let json = |uri: &str, body: &str| {
            Request::builder()
                .uri(uri)
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap()
        };
        let cases: Vec<(&str, Request<Body>)> = vec![
            (
                "create_meeting",
                json(
                    "/api/meeting/assemble/control/meeting/create",
                    r#"{"title":"t","startTime":"2026-01-01 00:00:00","endTime":"2026-01-01 01:00:00"}"#,
                ),
            ),
            (
                "meeting_id_checkin",
                json(
                    "/api/meeting/assemble/control/meeting/m1/checkin",
                    "{}",
                ),
            ),
            (
                "u2_attachment_create_from_processplatform",
                json(
                    "/api/meeting/assemble/control/attachment/create/from/processplatform",
                    r#"{"meetingId":"m1"}"#,
                ),
            ),
            (
                "u2_attachment_upload",
                Request::builder()
                    .uri("/api/meeting/assemble/control/attachment/meeting/m1/upload/true")
                    .method("POST")
                    .header("content-type", "multipart/form-data; boundary=xboundary")
                    .body(multipart_body())
                    .unwrap(),
            ),
            (
                "u2_attachment_upload_callback",
                Request::builder()
                    .uri("/api/meeting/assemble/control/attachment/meeting/m1/upload/true/callback/cb1")
                    .method("POST")
                    .header("content-type", "multipart/form-data; boundary=xboundary")
                    .body(multipart_body())
                    .unwrap(),
            ),
            (
                "u2_room_set_photo",
                Request::builder()
                    .uri("/api/meeting/assemble/control/room/r1/photo")
                    .method("POST")
                    .header("content-type", "multipart/form-data; boundary=xboundary")
                    .body(multipart_body())
                    .unwrap(),
            ),
        ];
        assert_eq!(cases.len(), 6, "本批 6 把写路由必须逐条点名");
        let mut leaked: Vec<String> = Vec::new();
        for (name, req) in cases {
            let status = app
                .clone()
                .oneshot(req)
                .await
                .unwrap()
                .status();
            if status != StatusCode::UNAUTHORIZED {
                leaked.push(format!("{name} => 期望 401，实际 {status}"));
            }
        }
        assert!(
            leaked.is_empty(),
            "匿名写路由未回 401（出现 500 就是门插在读库之后，出现 400/415 是 body 未过提取器）：{leaked:?}"
        );
    }
}
