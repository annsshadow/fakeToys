// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 四轮审计 N-06 回归：身份关联（`x_correlation`）写族此前整族无门禁——路由只吃
//! `Extension<Pool>`，既不取会话也不校归属，且 `creator` 恒写 `"system"`（无属主的
//! 系统级数据）。任一有效会话可按 person_id 给任意人挂/改/删/解外部身份（SSO、
//! 第三方账号），等价于身份劫持与关联清除。修复后按「系统级数据 = admin」档收口：
//! 无会话 401、非 admin 403，`is_admin` 无库时 fail-closed，故离线臂即可证拒绝侧。
//!
//! 覆盖格：`create/save/delete_correlation` + `unlink_service`（lib.rs 4 把）
//! 与 `create/update/delete_{pp,cms}`（u2.rs 6 把，共享同一个 `require_admin`）。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::{Extension, Path};
    use shared::error::AppError;

    use crate::{create_correlation, delete_correlation, save_correlation, unlink_service};

    #[tokio::test]
    async fn correlation_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin-correlation");
        let ext = || Extension(sess.clone());
        let p = || Path("c1".to_string());
        let req = || {
            axum::Json(crate::CorrelationRequest {
                person_id: Some("victim-person".into()),
                target_id: Some("victim-target".into()),
                r#type: Some("sso".into()),
            })
        };
        let ureq = || {
            axum::Json(crate::CorrelationUpdateRequest {
                target_id: Some("other-target".into()),
                r#type: Some("sso".into()),
            })
        };
        let create_wi = || {
            axum::Json(
                serde_json::from_value::<crate::u2::CreateWi>(serde_json::json!({})).unwrap(),
            )
        };
        let update_wi = || {
            axum::Json(
                serde_json::from_value::<crate::u2::UpdateWi>(serde_json::json!({})).unwrap(),
            )
        };
        let delete_wi = || {
            axum::Json(
                serde_json::from_value::<crate::u2::DeleteWi>(serde_json::json!({})).unwrap(),
            )
        };

        let cases: [(&str, Result<(), AppError>); 10] = [
            (
                "create_correlation",
                create_correlation(Extension(pool.clone()), ext(), req())
                    .await
                    .map(|_| ()),
            ),
            (
                "save_correlation",
                save_correlation(Extension(pool.clone()), ext(), p(), ureq())
                    .await
                    .map(|_| ()),
            ),
            (
                "delete_correlation",
                delete_correlation(Extension(pool.clone()), ext(), p())
                    .await
                    .map(|_| ()),
            ),
            (
                "unlink_service",
                unlink_service(
                    Extension(pool.clone()),
                    ext(),
                    Path((
                        "sso".to_string(),
                        "s1".to_string(),
                        "person".to_string(),
                        "p1".to_string(),
                    )),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::create_pp",
                crate::u2::create_pp(
                    Extension(pool.clone()),
                    ext(),
                    Path("job1".to_string()),
                    create_wi(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::create_cms",
                crate::u2::create_cms(
                    Extension(pool.clone()),
                    ext(),
                    Path("doc1".to_string()),
                    create_wi(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::update_pp",
                crate::u2::update_pp(
                    Extension(pool.clone()),
                    ext(),
                    Path("job1".to_string()),
                    update_wi(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::update_cms",
                crate::u2::update_cms(
                    Extension(pool.clone()),
                    ext(),
                    Path("doc1".to_string()),
                    update_wi(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::delete_pp",
                crate::u2::delete_pp(
                    Extension(pool.clone()),
                    ext(),
                    Path("job1".to_string()),
                    delete_wi(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "u2::delete_cms",
                crate::u2::delete_cms(
                    Extension(pool.clone()),
                    ext(),
                    Path("doc1".to_string()),
                    delete_wi(),
                )
                .await
                .map(|_| ()),
            ),
        ];

        let bad: Vec<&str> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Forbidden)))
            .map(|(n, _)| *n)
            .collect();
        assert!(
            bad.is_empty(),
            "10 把身份关联写必须全部 403（少一把说明门只盖住了部分入口），异常格：{bad:?}"
        );
    }

    /// 门必须早于任何 SQL：本臂用「拿不到库」的池，若门禁判定排在 `pool.get()` 之后，
    /// 错误会变成 `Internal` 而不是 `Forbidden`。
    #[tokio::test]
    async fn correlation_gate_runs_before_any_sql() {
        let pool = shared::testing::test_pool();
        let res = delete_correlation(
            Extension(pool),
            Extension(shared::testing::test_session("u-nonadmin-correlation")),
            Path("no-such-row".to_string()),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "拒绝必须来自门禁而不是库错误：{res:?}"
        );
    }

    /// DB 臂（行为差）：闯入者给他人挂关联必须 403 **且一行都不落**；
    /// admin 挂同一 (person, target) 才真正建行，臂末清理。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn admin_can_link_and_nonadmin_lands_no_rows() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: admin_can_link_and_nonadmin_lands_no_rows (no DATABASE_URL)");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = uuid::Uuid::new_v4().simple().to_string();
        let victim = format!("r48-victim-{tag}");
        let target = format!("r48-target-{tag}");
        {
            // 测试自举：本 crate 的契约用例用最小 DDL 建表，其中没有 `creator` 列，
            // 而 create_correlation 会写入该列。缺列时 INSERT 直接报错，
            // 会让「admin 放行侧」看起来像门失效。
            let c = pool.get().await.unwrap();
            c.execute(
                "ALTER TABLE x_correlation ADD COLUMN IF NOT EXISTS creator VARCHAR(255)",
                &[],
            )
            .await
            .unwrap();
        }

        let intruder = create_correlation(
            Extension(pool.clone()),
            Extension(shared::testing::test_session("u-r48-nonadmin")),
            axum::Json(crate::CorrelationRequest {
                person_id: Some(victim.clone()),
                target_id: Some(target.clone()),
                r#type: Some("sso".into()),
            }),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(intruder, Err(AppError::Forbidden)),
            "非 admin 为他人建身份关联必须 403"
        );
        let client = pool.get().await.unwrap();
        let n: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_correlation WHERE person_id = $1",
                &[&victim],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(n, 0, "拒绝路径不得留下任何关联行");

        // 按 is_admin 真实口径造一个 admin（未删未禁的 admin 角色 + 人员关联）
        let admin = format!("u-r48-admin-{tag}");
        client
            .execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false)",
                &[&format!("r48-role-{tag}")],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ($1, $1, 'r48 admin', 'x')",
                &[&admin],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1, $2, $3)",
                &[
                    &admin,
                    &format!("r48-role-{tag}"),
                    &format!("r48-unit-{tag}"),
                ],
            )
            .await
            .unwrap();

        let ok = create_correlation(
            Extension(pool.clone()),
            Extension(shared::testing::test_session(&admin)),
            axum::Json(crate::CorrelationRequest {
                person_id: Some(victim.clone()),
                target_id: Some(target.clone()),
                r#type: Some("sso".into()),
            }),
        )
        .await
        .map(|_| ());
        assert!(ok.is_ok(), "admin 应放行，实际 {ok:?}");
        let landed: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_correlation WHERE person_id = $1 AND target_id = $2",
                &[&victim, &target],
            )
            .await
            .unwrap()
            .get("c");
        assert_eq!(landed, 1, "admin 臂应确实建出关联行");

        client
            .execute("DELETE FROM x_correlation WHERE person_id = $1", &[&victim])
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM auth_person_role WHERE person_id = $1",
                &[&admin],
            )
            .await
            .unwrap();
        client
            .execute("DELETE FROM auth_person WHERE id = $1", &[&admin])
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM auth_role WHERE id = $1",
                &[&format!("r48-role-{tag}")],
            )
            .await
            .unwrap();
    }
}
