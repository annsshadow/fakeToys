// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十三批）：attendance_assemble_control 写面末 2 把。
//!
//! - `attendanceappealInfo_workflow_appeal_id`（POST+PUT，`UPDATE x_attendance_appeal_info`
//!   SET workflow_status='appealed'）→ admin 档（同族 workflow_sync 已是 admin）
//! - `attendancedetail_analyse`（`UPDATE x_attendance_detail` SET analysed=true 按 person 区间批量）
//!   → admin 档（同族 analyse/id/{id}=require_admin_opt、analyse/redo=require_admin）
//!
//! 两把此前既无会话门也无 admin 门，任何登录人即可推进申述工作流 / 把他人考勤明细标「已核算」
//! 跳过重算。门插在 `pool.get()` 前。
//!
//! DB 依赖臂：`cargo test --lib -p attendance_assemble_control -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests_b63 {
    use axum::extract::{Extension, Json, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn non_admin() -> shared::session::Session {
        shared::testing::test_session("b63-nonadmin")
    }

    /// 无会话臂：dead pool + None 会话，2 把必须全 401（门在库前，绝不落 Internal）。
    #[tokio::test]
    async fn b63_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "attendanceappealInfo_workflow_appeal_id",
                crate::attendanceappealInfo_workflow_appeal_id(
                    Extension(pool.clone()),
                    None,
                    Path("b63-appeal-id".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "attendancedetail_analyse",
                crate::attendancedetail_analyse(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "personId": "b63-p", "startDate": "2026-01-01", "endDate": "2026-01-31" })),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 2, "本批 2 把必须逐把点名");
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

    /// 非 admin 臂：有效会话但非 admin，2 把必须 403（is_admin 对不可用库 fail-closed，离线稳健）。
    #[tokio::test]
    async fn b63_writes_non_admin_forbidden() {
        let pool = shared::testing::test_pool();
        for (name, res) in [
            (
                "attendanceappealInfo_workflow_appeal_id",
                crate::attendanceappealInfo_workflow_appeal_id(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Path("b63-appeal-id".to_string()),
                )
                .await
                .map(|_| ()),
            ),
            (
                "attendancedetail_analyse",
                crate::attendancedetail_analyse(
                    Extension(pool.clone()),
                    Some(Extension(non_admin())),
                    Json(json!({ "personId": "b63-p", "startDate": "2026-01-01", "endDate": "2026-01-31" })),
                )
                .await
                .map(|_| ()),
            ),
        ] {
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "{name} 非 admin 必须 403，实得 {res:?}"
            );
        }
    }
}
