// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    use crate::general_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_status_route_accessible() {
        let app = general_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_status_update_route_accessible() {
        let app = general_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/status/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"maintenanceMode":false,"allowRegistration":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_permissions_route_accessible() {
        let app = general_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/permissions/mind")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_status_update_response_shape() {
        let app = general_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/status/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"maintenanceMode":true,"allowRegistration":false}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_area_list() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/area/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_area_id() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/area/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_attendsco() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/attendscope/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_ecnet_che() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/ecnet/check")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_excel_res() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/excel/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_excel_exc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/excel/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_generalfi() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/generalfile/download/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_invoice_d() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/download/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_invoice_g() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/get/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_invoice_l() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/list/paging/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_office_ht() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/office/html/to/word/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_permissio() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/permissions/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_qrcode_li() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/qrcode/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_qrcode_id() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/qrcode/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_securityc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/securityclearance/object")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_status() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/status")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_upgrade_2() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/upgrade/2021090901")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_general_assemble_control_worktime_() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/worktime/between/holiday/count/start/test-id/end/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_area_crea() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/area/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_area_dele() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/area/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_area_upda() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/area/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_attendsco() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/attendscope/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_excel_upl() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/excel/upload")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_invoice_c() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_invoice_d() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_invoice_u() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/invoice/update/apply/status/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_office_ht() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/office/html/to/word")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_qrcode_de() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/qrcode/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_qrcode_wi() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/qrcode/width/test-id/height/test-id/text/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_securityc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/securityclearance/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_post_general_assemble_control_status_up() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/status/update")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_post_ecnet_check_route() {
        let app = general_assemble_control_router(test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/ecnet/check")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_get_worktime_isholiday_route() {
        let app = general_assemble_control_router(test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/worktime/isholiday/2024-01-01")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn u2_test_post_office_route() {
        let app = general_assemble_control_router(test_pool());
        let body = serde_json::to_string(&serde_json::json!({"htmlContent": "<p>x</p>"})).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/assemble/control/office")
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    /// 批2：二维码写入的 creator 必须落会话身份，而不是常量 `'system'`。
    /// 写侧固定常量会让读侧按 creator 收归属的谓词永远对不上。
    #[tokio::test]
    async fn b2_qrcode_create_persists_session_creator() {
        let pool = test_pool();
        let person = "b2-qrcode-owner";
        let text = format!("b2-qrcode-{}", uuid::Uuid::new_v4());
        let res = crate::qrcode_width_width_height_height_text_text(
            axum::Extension(pool.clone()),
            Some(axum::Extension(shared::testing::test_session(person))),
            axum::extract::Path((120u32, 120u32, text.clone())),
        )
        .await;
        assert!(res.is_ok(), "有会话建二维码必须成功");

        let client = pool.get().await.expect("db");
        let row = client
            .query_opt(
                "SELECT creator FROM x_general_assemble_qrcode WHERE text = $1 \
                 ORDER BY create_time DESC LIMIT 1",
                &[&text],
            )
            .await
            .expect("query qrcode");
        let owner = row
            .as_ref()
            .and_then(|r| r.get::<_, Option<String>>("creator"))
            .unwrap_or_default();
        client
            .execute(
                "DELETE FROM x_general_assemble_qrcode WHERE text = $1",
                &[&text],
            )
            .await
            .expect("cleanup qrcode");
        assert_eq!(owner, person, "二维码 creator 必须是会话身份");
    }
}

// ── r7：securityclearance_enable 的 creator 不再硬写 'system'（建→改） ──
#[tokio::test]
async fn r7_securityclearance_enable_persists_session_creator() {
    let pool = shared::testing::test_pool();
    let client = pool.get().await.expect("db");
    const ADMIN: &str = "r7-general-admin";
    const ROLE: &str = "r7-general-role-admin";
    client
        .execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
             ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
            &[&ROLE.to_string()],
        )
        .await
        .expect("seed admin role");
    shared::testing::upsert_fixture_person(&client, ADMIN, ADMIN, "r7 general admin").await;
    client
        .execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ($1,$2,'r7-unit') \
             ON CONFLICT DO NOTHING",
            &[&ADMIN.to_string(), &ROLE.to_string()],
        )
        .await
        .expect("seed admin role map");

    let name = format!("r7-sc-{}", uuid::Uuid::new_v4().simple());
    let created = crate::securityclearance_enable(
        axum::extract::Extension(pool.clone()),
        axum::extract::Extension(shared::testing::test_session(ADMIN)),
        axum::extract::Json(serde_json::json!({"name": name, "enabled": true})),
    )
    .await;
    assert!(created.is_ok(), "admin 开关安全等级必须成功");
    let owner: Option<String> = client
        .query_one(
            "SELECT creator FROM x_general_assemble_security_clearance WHERE name = $1",
            &[&name],
        )
        .await
        .expect("created clearance row")
        .get("creator");
    assert_eq!(
        owner.as_deref(),
        Some(ADMIN),
        "建后 owner 必须是会话人，不得是 'system'"
    );

    let updated = crate::securityclearance_enable(
        axum::extract::Extension(pool.clone()),
        axum::extract::Extension(shared::testing::test_session(ADMIN)),
        axum::extract::Json(serde_json::json!({"name": name, "enabled": false})),
    )
    .await;
    assert!(updated.is_ok(), "admin 改写安全等级必须成功");
    let owner_after_update: Option<String> = client
        .query_one(
            "SELECT creator FROM x_general_assemble_security_clearance WHERE name = $1",
            &[&name],
        )
        .await
        .expect("updated clearance row")
        .get("creator");
    assert_eq!(
        owner_after_update.as_deref(),
        Some(ADMIN),
        "改写后 owner 不得变化"
    );
    client
        .execute(
            "DELETE FROM x_general_assemble_security_clearance WHERE name = $1",
            &[&name],
        )
        .await
        .expect("cleanup clearance");
}
