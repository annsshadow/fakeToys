#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_organization_assemble_control_role_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_role_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_role_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_role_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_unit_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_unit_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_unit_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_unit_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_unit_list_flag_sub_nested() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sub/nested")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_unit_list_flag_sub_nested route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_unit_list_flag_sup_nested() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sup/nested")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_unit_list_flag_sup_nested route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_unit_list_flag_sup_nested_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sup/nested/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_unit_list_flag_sup_nested_type_type route should be registered");
        }
    }

    #[tokio::test]
    async fn test_organization_assemble_control_person_list_like() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/person/list/like")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "organization_assemble_control_person_list_like route should be registered");
        }
    }

    #[tokio::test]
    async fn test_export_export_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/export/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "export_export_all route should be registered");
        }
    }

    #[tokio::test]
    async fn test_export_result_flag_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/result/flag/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "export_result_flag_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_export_zhengwudingding_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/zhengwudingding/person")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "export_zhengwudingding_person route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_like_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_like_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_like_pinyin() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/pinyin")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_like_pinyin route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_like_pinyin_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_like_pinyin_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_pinyininitial() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/pinyininitial")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_pinyininitial route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_pinyininitial_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_pinyininitial_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_sub_direct() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sub/direct")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_sub_direct route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_sub_nested() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sub/nested")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_sub_nested route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_sup_direct() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sup/direct")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_sup_direct route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_sup_nested() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sup/nested")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_sup_nested route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_add_member() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/add/member")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_add_member route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_add_member_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/add/member/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_add_member_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_delete_member() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/delete/member")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_delete_member route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_delete_member_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/delete/member/mockputtopost")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_delete_member_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/identity/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_id route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_like_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_like_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_like_pinyin() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/pinyin")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_like_pinyin route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_like_pinyin_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_like_pinyin_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_pinyininitial() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/pinyininitial")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_pinyininitial route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_pinyininitial_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_pinyininitial_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_inputperson_template() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/template")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "inputperson_template route should be registered");
        }
    }

    #[tokio::test]
    async fn test_inputperson_wipe() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/wipe")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "inputperson_wipe route should be registered");
        }
    }

    #[tokio::test]
    async fn test_permissionsetting_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "permissionsetting_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_permissionsetting_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "permissionsetting_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_permissionsetting_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "permissionsetting_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_permissionsetting_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "permissionsetting_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listgrouptypes() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listgrouptypes")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listgrouptypes route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listpaging_page_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpaging/page/test-id/size/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listpaging_page_page_size_size_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listpagingwithgroup_page_page_size_size_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/test-id/size/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listpagingwithgroup_page_page_size_size_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_mylist() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/mylist")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_mylist route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_like_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_like_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_like_pinyin() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/pinyin")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_like_pinyin route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_like_pinyin_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_like_pinyin_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_pinyininitial() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/pinyininitial")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_pinyininitial route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_pinyininitial_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_pinyininitial_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_distinct_name() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/distinct/name")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_distinct_name route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_flag_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_flag_mockputtopost route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_flag_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id/mockdeletetoget")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_flag_mockdeletetoget route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_update_member() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/update/member")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_update_member route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_unit_unitFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/unit/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_unit_unitFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_name_name() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/name/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_name_name route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_like() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/like")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_like route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_list_identity_identityFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/identity/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_list_identity_identityFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitduty_distinct_name_like_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/distinct/name/like/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitduty_distinct_name_like_key route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_unitattribute_list_unit_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/unit/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "unitattribute_list_unit_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_person_personFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_person_personFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_like() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_like route should be registered");
        }
    }

    #[tokio::test]
    async fn test_role_list_group_groupFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/group/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "role_list_group_groupFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listpagingwithgroup_page_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listpagingwithgroup_page_page_size_size route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listpaging_page_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpaging/page/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listpaging_page_page_size_size route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listVCf_idList() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listVCf/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listVCf_idList route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_listPersonalVCf_idList() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listPersonalVCf/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_listPersonalVCf_idList route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_createQR_cardId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/createQR/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_createQR_cardId route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personcard_createCode_cardId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/createCode/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personcard_createCode_cardId route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_personattribute_list_person_personFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "personattribute_list_person_personFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_loginrecord_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/loginrecord/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "loginrecord_stream route should be registered");
        }
    }

    #[tokio::test]
    async fn test_inputperson_result_flag_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/result/flag/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "inputperson_result_flag_flag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_flag_order_before_followFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/order/before/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_flag_order_before_followFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_flag_unitduty_name_unitDutyName() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/unitduty/name/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_flag_unitduty_name_unitDutyName route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_unitduty_name_unitDutyName() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/unitduty/name/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_unitduty_name_unitDutyName route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_unit_unitFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/unit/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_unit_unitFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_person_personFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_person_personFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_identity_list_like() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "identity_list_like route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_prev_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_flag_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_flag_next_count route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_role_roleFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/role/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_role_roleFlag route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_person_personFlag_sup_nested() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/person/test-id/sup/nested")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_person_personFlag_sup_nested route should be registered");
        }
    }

    #[tokio::test]
    async fn test_group_list_person_personFlag_sup_direct() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/person/test-id/sup/direct")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(),
                "group_list_person_personFlag_sup_direct route should be registered");
        }
    }

    // SKIPPED: duty_create requires Session parameter
    // SKIPPED: duty_edit requires Session parameter
    // SKIPPED: duty_mock_put_to_post requires Session parameter
    // SKIPPED: duty_delete requires Session parameter
    // SKIPPED: duty_update_member requires Session parameter
    // SKIPPED: duty_list_like not accessible
    // SKIPPED: resolve_person_id not accessible
    // SKIPPED: person_get not accessible
    // SKIPPED: person_create requires Session parameter
    // SKIPPED: person_edit requires Session parameter
    // SKIPPED: person_mock_put_to_post requires Session parameter
    // SKIPPED: person_delete requires Session parameter
    // SKIPPED: person_mock_delete_to_get requires Session parameter
    // SKIPPED: person_reserve_delete requires Session parameter
    // SKIPPED: person_reserve_mock_delete_to_get requires Session parameter
    // SKIPPED: person_list_next not accessible
    // SKIPPED: person_list_prev not accessible
    // SKIPPED: person_list_group_sub_direct not accessible
    // SKIPPED: person_list_group_sub_nested not accessible
    // SKIPPED: person_list_with_role not accessible
    // SKIPPED: person_list_pinyininitial not accessible
    // SKIPPED: person_list_like not accessible
    // SKIPPED: person_list_like_pinyin not accessible
    // SKIPPED: person_set_password requires Session parameter
    // SKIPPED: person_set_password_mock_put_to_post requires Session parameter
    // SKIPPED: person_reset_password requires Session parameter
    // SKIPPED: person_check_password not accessible
    // SKIPPED: person_get_icon not accessible
    // SKIPPED: person_set_icon requires Session parameter
    // SKIPPED: person_set_icon_mock_put_to_post requires Session parameter
    // SKIPPED: person_lock requires Session parameter
    // SKIPPED: person_unlock requires Session parameter
    // SKIPPED: person_ban requires Session parameter
    // SKIPPED: person_unban requires Session parameter
    // SKIPPED: person_set_password_expired_time requires Session parameter
    // SKIPPED: person_list_filter_paging not accessible
    // SKIPPED: person_list_delete_paging not accessible
}