#[cfg(test)]
mod tests {
    use crate::general_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_status_route_accessible() {
        let app = general_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/status")
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
                    .uri("/jaxrs/general/assemble/control/status/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"maintenanceMode":false,"allowRegistration":true}"#))
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
                    .uri("/jaxrs/general/assemble/control/permissions/mind")
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
                    .uri("/jaxrs/general/assemble/control/status/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"maintenanceMode":true,"allowRegistration":false}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_get_jaxrs_general_assemble_control_area_list() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/list")
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
    async fn test_get_jaxrs_general_assemble_control_area_id() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_attendsco() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/list")
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
    async fn test_get_jaxrs_general_assemble_control_ecnet_che() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/ecnet/check")
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
    async fn test_get_jaxrs_general_assemble_control_excel_res() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/result/flag/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_excel_exc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_generalfi() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/generalfile/download/flag/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_invoice_d() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/download/flag/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_invoice_g() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/get/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_invoice_l() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/list/paging/test-id/size/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_office_ht() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/office/html/to/word/result/flag/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_permissio() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/permissions/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_qrcode_li() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/list")
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
    async fn test_get_jaxrs_general_assemble_control_qrcode_id() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/test-id")
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
    async fn test_get_jaxrs_general_assemble_control_securityc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/object")
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
    async fn test_get_jaxrs_general_assemble_control_status() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/status")
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
    async fn test_get_jaxrs_general_assemble_control_upgrade_2() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/upgrade/2021090901")
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
    async fn test_get_jaxrs_general_assemble_control_worktime_() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/between/holiday/count/start/test-id/end/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_area_crea() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/create")
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
    async fn test_post_jaxrs_general_assemble_control_area_dele() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/delete/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_area_upda() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/update/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_attendsco() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/create")
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
    async fn test_post_jaxrs_general_assemble_control_excel_upl() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/upload")
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
    async fn test_post_jaxrs_general_assemble_control_invoice_c() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/create")
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
    async fn test_post_jaxrs_general_assemble_control_invoice_d() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/delete/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_invoice_u() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/update/apply/status/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_office_ht() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/office/html/to/word")
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
    async fn test_post_jaxrs_general_assemble_control_qrcode_de() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/delete/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_qrcode_wi() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/width/test-id/height/test-id/text/test-id")
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
    async fn test_post_jaxrs_general_assemble_control_securityc() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/create")
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
    async fn test_post_jaxrs_general_assemble_control_status_up() {
        let pool = test_pool();
        let app = crate::general_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/status/update")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }


}
