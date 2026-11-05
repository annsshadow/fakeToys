// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 R4-6）：`GET /api/processplatform/work/terminate/{id}`
//! 此前既不取会话也不校归属——按 ID 即可把任意 work 置为 terminated，且注册在 GET 上，
//! 不受 csrf_middleware（只覆盖 POST/PUT/PATCH/DELETE）与 requires_admin（只认三个方法）保护。
//! 现在：无会话 401；有会话但非属主且非 admin 403（`require_owner`，admin 旁路）。
//! 注：`work_retract` 名字像写，实为只读 SELECT，故不在本门禁面内。
//!
//! DB 依赖的属主臂需真实 PostgreSQL：
//! `cargo test --lib -p processplatform_core_express -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::body::Body;
    use axum::extract::Extension;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::Pool;
    use shared::error::AppError;
    use tower::ServiceExt;

    #[tokio::test]
    async fn work_terminate_route_rejects_without_session() {
        // 路由级：router 只 layer 了 Pool，无会话即 401，且这条路径完全不碰 DB。
        let pool: Pool = shared::testing::test_pool();
        let app = crate::processplatform_core_express_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/work/terminate/work-r4-6")
                    .method(Method::GET)
                    .body(Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("oneshot dispatch");
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "无会话终止 work 必须 401"
        );
    }

    #[tokio::test]
    async fn work_terminate_handler_rejects_without_session() {
        let res = crate::work_terminate(
            Extension(shared::testing::test_pool()),
            None,
            axum::extract::Path("work-r4-6".to_string()),
        )
        .await;
        let got_unauthorized = matches!(res, Err(AppError::Unauthorized));
        assert!(got_unauthorized, "无会话调用终止写必须 401");
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_terminate_forbidden_for_non_owner_non_admin() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let work_id = format!("r46-{}", uuid::Uuid::new_v4());
        let creator = "u-r46-owner";
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_work (id, title, process, application, work_status, creator) \
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &work_id,
                    &"r4-6 gate".to_string(),
                    &"r46".to_string(),
                    &"r46".to_string(),
                    &"processing".to_string(),
                    &creator.to_string(),
                ],
            )
            .await
            .unwrap();

        let res = crate::work_terminate(
            Extension(pool.clone()),
            Some(Extension(shared::testing::test_session("u-r46-intruder"))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        let got_forbidden = matches!(res, Err(AppError::Forbidden));
        assert!(got_forbidden, "非属主非 admin 终止他人 work 必须 403");

        // 属主本人可终止，且状态确实落地
        let res = crate::work_terminate(
            Extension(pool.clone()),
            Some(Extension(shared::testing::test_session(creator))),
            axum::extract::Path(work_id.clone()),
        )
        .await;
        assert!(res.is_ok(), "属主终止自己的 work 应放行");
        let status: Option<String> = client
            .query_opt("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .map(|row| row.get("work_status"));
        assert_eq!(status.as_deref(), Some("terminated"));

        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }

    /// P0-b（审计第六轮）：`creator=NULL` 的 work，`work_terminate` 的**内联副本**
    /// 曾用 `filter(!empty)` 把空归属过滤成「无需校验」，普通会话按 ID 就能
    /// `pending → terminated`。修后：空归属只允许 admin。
    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn work_terminate_empty_creator_is_admin_only() {
        if !shared::testing::is_db_available().await {
            eprintln!("skip: PG unavailable");
            return;
        }
        let pool = shared::testing::test_pool();
        let tag = format!("r6-{}", uuid::Uuid::new_v4().simple());
        let work_id = format!("{tag}-work");
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_work (id, title, process, application, work_status, creator) \
                 VALUES ($1, $2, $3, $4, $5, NULL)",
                &[
                    &work_id,
                    &"r6 empty creator".to_string(),
                    &"r6".to_string(),
                    &"r6".to_string(),
                    &"pending".to_string(),
                ],
            )
            .await
            .unwrap();

        // ① 普通会话 → 403，且状态不变。
        let res = crate::work_terminate(
            Extension(pool.clone()),
            Some(Extension(shared::testing::test_session(&format!(
                "{tag}-intruder"
            )))),
            axum::extract::Path(work_id.clone()),
        )
        .await
        .map(|_| ());
        assert!(
            matches!(res, Err(AppError::Forbidden)),
            "creator=NULL 不得对普通会话开放终止（实际 {res:?}）"
        );
        let status: Option<String> = client
            .query_opt("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .map(|row| row.get("work_status"));
        assert_eq!(status.as_deref(), Some("pending"), "403 时状态不得落库");

        // ② admin → 200 且状态 terminated。
        let (admin, role_id) = seed_admin(&pool, &tag).await;
        let res = crate::work_terminate(
            Extension(pool.clone()),
            Some(Extension(shared::testing::test_session(&admin))),
            axum::extract::Path(work_id.clone()),
        )
        .await
        .map(|_| ());
        assert!(
            res.is_ok(),
            "admin 代终止 creator=NULL 应放行（实际 {res:?}）"
        );
        let status: Option<String> = client
            .query_opt("SELECT work_status FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap()
            .map(|row| row.get("work_status"));
        assert_eq!(status.as_deref(), Some("terminated"));

        unseed_admin(&pool, &admin, &role_id).await;
        client
            .execute("DELETE FROM x_work WHERE id = $1", &[&work_id])
            .await
            .unwrap();
    }

    /// 按 `is_admin` 的真实口径（`auth_role.name='admin'` + 未删未停用）幂等造一个 admin。
    async fn seed_admin(pool: &Pool, tag: &str) -> (String, String) {
        let client = pool.get().await.unwrap();
        let person = format!("u-{tag}-admin");
        let role_id = format!("{tag}-role");
        client
            .execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false) \
                 ON CONFLICT (id) DO NOTHING",
                &[&role_id],
            )
            .await
            .unwrap();
        shared::testing::upsert_fixture_person(&client, &person, &person, &person).await;
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1, $2, $3) \
                 ON CONFLICT DO NOTHING",
                &[&person, &role_id, &format!("{tag}-unit")],
            )
            .await
            .unwrap();
        (person, role_id)
    }

    async fn unseed_admin(pool: &Pool, person: &str, role_id: &str) {
        let client = pool.get().await.unwrap();
        client
            .execute(
                "DELETE FROM auth_person_role WHERE person_id = $1",
                &[&person.to_string()],
            )
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM auth_person WHERE id = $1",
                &[&person.to_string()],
            )
            .await
            .unwrap();
        client
            .execute(
                "DELETE FROM auth_role WHERE id = $1",
                &[&role_id.to_string()],
            )
            .await
            .unwrap();
    }
}
