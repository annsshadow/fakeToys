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
}
