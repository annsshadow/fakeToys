#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    async fn ensure_test_surface(pool: &deadpool_postgres::Pool) {
        let client = pool.get().await.unwrap();
        let existing: i64 = client
            .query_one("SELECT COUNT(*)::bigint FROM x_process_surface WHERE id = $1", &[&"test-id"])
            .await
            .unwrap()
            .get(0);
        if existing == 0 {
            let _ = client.execute(
                "INSERT INTO x_process_surface (id, name, category, content, version, creator, create_time, update_time)
                 VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
                 ON CONFLICT (id) DO NOTHING",
                &[&"test-id", &"Test Surface", &"processplatform", &r#"{"html":"<div>test</div>"}"#, &"1.0", &"test"],
            ).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_get_surface() {
        let pool = shared::testing::test_pool();
        ensure_test_surface(&pool).await;
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/get/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_surface route should be registered");
    }

    #[tokio::test]
    async fn test_create_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "create_surface route should be registered");
    }

    #[tokio::test]
    async fn test_list_surfaces() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_surfaces route should be registered");
    }

    #[tokio::test]
    async fn test_preview_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/preview/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "preview_surface route should be registered");
    }

    #[tokio::test]
    async fn test_publish_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/publish/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "publish_surface route should be registered");
    }

    #[tokio::test]
    async fn test_delete_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "delete_surface route should be registered");
    }

    #[tokio::test]
    async fn test_save_surface() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/save/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "save_surface route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_read_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/anonymous/read/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_read_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_anonymous_task_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/anonymous/task/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "anonymous_task_count_credential route should be registered");
    }

    // SKIPPED: application_list not accessible
    // SKIPPED: application_list_complex not accessible
    #[tokio::test]
    async fn test_application_list_complex_manage_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/list/complex/manage/person")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_list_complex_manage_person route should be registered");
    }

    #[tokio::test]
    async fn test_application_list_key_key() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/list/key/key")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_list_key_key route should be registered");
    }

    // SKIPPED: application_list_range not accessible
    #[tokio::test]
    async fn test_application_list_terminal_terminal() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/list/terminal/terminal")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_list_terminal_terminal route should be registered");
    }

    #[tokio::test]
    async fn test_application_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_flag route should be registered");
    }

    #[tokio::test]
    async fn test_application_flag_icon() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/icon/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_flag_icon route should be registered");
    }

    #[tokio::test]
    async fn test_application_flag_is_manager() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/is/manager/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_flag_is_manager route should be registered");
    }

    #[tokio::test]
    async fn test_application_flag_onlyRemoveNotCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/application/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "application_flag_onlyRemoveNotCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_list_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/list/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_list_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/applicationdict/application/path0/path1/path2/path3/path4/path5/path6/path7/data/mockputtopost/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_control_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/control/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "control_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/correlation/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_job_job_delete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/correlation/job/job/delete")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_job_job_delete route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/correlation/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_list_job_job_site_site() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/correlation/list/job/job/site/site")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_list_job_job_site_site route should be registered");
    }

    #[tokio::test]
    async fn test_correlation_update_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/correlation/update/job/job")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "correlation_update_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_data_fetch_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/fetch/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_fetch_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_array_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/array/data")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_array_data route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5_path6() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5_path6 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5_path6_path7() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5_path6_path7 route should be registered");
    }

    #[tokio::test]
    async fn test_data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/job/job/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6_path7() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6_path7 route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/work/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_from_data() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/from/data/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_from_data route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_from_item() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/from/item/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_from_item route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/path7/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7 route should be registered");
    }

    #[tokio::test]
    async fn test_data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/data/workcompleted/path0/path1/path2/path3/path4/path5/path6/path7/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_datarecord_get_job_job_path_path() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/datarecord/get/job/job/path/path")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "datarecord_get_job_job_path_path route should be registered");
    }

    #[tokio::test]
    async fn test_datarecord_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/datarecord/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "datarecord_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_list_job_job_category_category() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/list/job/job/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_list_job_job_category_category route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_list_workorworkcompleted_workOrWorkCompleted_category_category() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/list/workorworkcompleted/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_list_workorworkcompleted_workOrWorkCompleted_category_category route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/work/work")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_work_work route should be registered");
    }

    #[tokio::test]
    async fn test_documentversion_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/documentversion/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "documentversion_id route should be registered");
    }

    #[tokio::test]
    async fn test_draft_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_draft_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_draft_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_draft_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_draft_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_draft_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_id route should be registered");
    }

    #[tokio::test]
    async fn test_draft_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_draft_id_start() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/draft/start/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "draft_id_start route should be registered");
    }

    #[tokio::test]
    async fn test_file_list_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/file/list/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_list_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_application_applicationFlag_content() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/file/application/content/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_application_applicationFlag_content route should be registered");
    }

    #[tokio::test]
    async fn test_file_flag_application_applicationFlag_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/file/application/download/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "file_flag_application_applicationFlag_download route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_taskcompleted_taskcompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_taskcompleted_taskcompleted route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_taskcompleted_taskcompleted_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/lookup/taskcompleted/taskcompleted/mobile")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_taskcompleted_taskcompleted_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_lookup_workorworkcompleted_workOrWorkCompleted_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/lookup/workorworkcompleted/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_lookup_workorworkcompleted_workOrWorkCompleted_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_id route should be registered");
    }

    #[tokio::test]
    async fn test_form_v2_id_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/v2/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_v2_id_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_form_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_flag route should be registered");
    }

    #[tokio::test]
    async fn test_form_flag_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/application/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_flag_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_form_flag_application_applicationFlag_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/application/mobile/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_flag_application_applicationFlag_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_form_flag_mobile() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/form/mobile/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "form_flag_mobile route should be registered");
    }

    #[tokio::test]
    async fn test_handover_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/handover/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "handover_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_handover_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/handover/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "handover_id route should be registered");
    }

    #[tokio::test]
    async fn test_handover_id_cancel() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/handover/cancel/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "handover_id_cancel route should be registered");
    }

    #[tokio::test]
    async fn test_handover_id_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/handover/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "handover_id_process route should be registered");
    }

    #[tokio::test]
    async fn test_job_latest_work_workcompleted_serial_serial() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/job/latest/work/workcompleted/serial/serial")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "job_latest_work_workcompleted_serial_serial route should be registered");
    }

    #[tokio::test]
    async fn test_job_v2_job_projection() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/job/v2/job/projection")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "job_v2_job_projection route should be registered");
    }

    #[tokio::test]
    async fn test_job_job_allow_visit_person_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/job/job/allow/visit/person/person")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "job_job_allow_visit_person_person route should be registered");
    }

    #[tokio::test]
    async fn test_job_job_find_work_workcompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/job/job/find/work/workcompleted")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "job_job_find_work_workcompleted route should be registered");
    }

    #[tokio::test]
    async fn test_keylock_lock() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/keylock/lock")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "keylock_lock route should be registered");
    }

    #[tokio::test]
    async fn test_keylock_lock_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/keylock/lock/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "keylock_lock_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_mode_clear_person_person_manager() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/mode/clear/person/person/manager")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mode_clear_person_person_manager route should be registered");
    }

    // SKIPPED: mode_list not accessible
    #[tokio::test]
    async fn test_mode_save() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/mode/save")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mode_save route should be registered");
    }

    #[tokio::test]
    async fn test_mode_id_delete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/mode/delete/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mode_id_delete route should be registered");
    }

    #[tokio::test]
    async fn test_process_activity_activity_activityType_activityType() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/activity/activity/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_activity_activity_activityType_activityType route should be registered");
    }

    #[tokio::test]
    async fn test_process_list_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/list/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_list_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_process_list_application_applicationFlag_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/list/application/filter/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_list_application_applicationFlag_filter route should be registered");
    }

    #[tokio::test]
    async fn test_process_list_available_identity_process_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/list/available/identity/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_list_available_identity_process_flag route should be registered");
    }

    #[tokio::test]
    async fn test_process_list_controllable_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/list/controllable/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_list_controllable_application_applicationFlag route should be registered");
    }

    // SKIPPED: process_list_ids not accessible
    #[tokio::test]
    async fn test_process_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_flag route should be registered");
    }

    #[tokio::test]
    async fn test_process_flag_allowrerouteto() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/allowrerouteto/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_flag_allowrerouteto route should be registered");
    }

    #[tokio::test]
    async fn test_process_flag_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/application/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_flag_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_process_flag_complex() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/complex/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_flag_complex route should be registered");
    }

    #[tokio::test]
    async fn test_process_flag_onlyRemoveNotCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "process_flag_onlyRemoveNotCompleted route should be registered");
    }

    // SKIPPED: read_count_filter not accessible
    #[tokio::test]
    async fn test_read_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_read_filter_attribute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/filter/attribute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_filter_attribute route should be registered");
    }

    #[tokio::test]
    async fn test_read_filter_attribute_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/filter/attribute/filter")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_filter_attribute_filter route should be registered");
    }

    // SKIPPED: read_list_count_application not accessible
    #[tokio::test]
    async fn test_read_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_date_date_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/date/date/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_date_date_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_my_filter_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/my/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_my_filter_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_person_person_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/person/person/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_person_person_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/work/work")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_work_work route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_next_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/next/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_next_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_next_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/next/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_next_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_prev_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/prev/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_prev_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_read_list_id_prev_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/list/prev/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_list_id_prev_count_process_processFlag route should be registered");
    }

    // SKIPPED: read_v2_count not accessible
    // SKIPPED: read_v2_list not accessible
    #[tokio::test]
    async fn test_read_v2_list_create_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/create/paging/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_create_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_read_v2_list_create_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/create/next/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_create_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_v2_list_create_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/create/prev/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_create_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_read_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_read_work_workId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/work/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_work_workId route should be registered");
    }

    #[tokio::test]
    async fn test_read_workcompleted_workCompletedId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/workcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_workcompleted_workCompletedId route should be registered");
    }

    #[tokio::test]
    async fn test_read_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_opinion_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/opinion/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_opinion_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_opinion_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/opinion/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_opinion_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/processing/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_processing route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_processing_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/processing/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_processing_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_processing_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/processing/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_processing_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_reference() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/reference/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_reference route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_reset_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/reset/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_reset_manage route should be registered");
    }

    #[tokio::test]
    async fn test_read_id_reset_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/read/reset/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "read_id_reset_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_filter_attribute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_filter_attribute route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_filter_attribute_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/filter/attribute/filter")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_filter_attribute_filter route should be registered");
    }

    // SKIPPED: readcompleted_list_count_application not accessible
    #[tokio::test]
    async fn test_readcompleted_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_date_date_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/date/date/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_date_date_manage route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_my_filter_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/my/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_my_filter_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/work/work")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_work_work route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_next_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/next/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_next_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_next_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/next/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_next_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_prev_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_prev_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_list_id_prev_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/list/prev/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_list_id_prev_count_process_processFlag route should be registered");
    }

    // SKIPPED: readcompleted_v2_count not accessible
    // SKIPPED: readcompleted_v2_list not accessible
    #[tokio::test]
    async fn test_readcompleted_v2_list_create_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/paging/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_create_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_v2_list_create_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/next/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_create_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_v2_list_create_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/create/prev/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_create_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_id route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_id_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_id_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_id_opinion_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/opinion/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_id_opinion_manage route should be registered");
    }

    #[tokio::test]
    async fn test_readcompleted_id_reference() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readcompleted/reference/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readcompleted_id_reference route should be registered");
    }

    #[tokio::test]
    async fn test_readrecord_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readrecord/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readrecord_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_readrecord_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/readrecord/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "readrecord_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_record_job_job_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/job/job/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_job_job_manage route should be registered");
    }

    #[tokio::test]
    async fn test_record_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_record_list_job_job_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/list/job/job/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_list_job_job_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_record_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/list/workorworkcompleted/paging/test-id/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_record_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_record_id_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_id_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_record_id_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/record/manage/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "record_id_manage_mockputtopost route should be registered");
    }

    // SKIPPED: review_count_application not accessible
    #[tokio::test]
    async fn test_review_count_person_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/person/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_count_person_credential route should be registered");
    }

    #[tokio::test]
    async fn test_review_create_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/create/work")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_create_work route should be registered");
    }

    #[tokio::test]
    async fn test_review_create_workcompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/create/workcompleted")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_create_workcompleted route should be registered");
    }

    #[tokio::test]
    async fn test_review_filter_attribute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/filter/attribute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_filter_attribute route should be registered");
    }

    #[tokio::test]
    async fn test_review_filter_create_entry() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/filter/create/entry")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_filter_create_entry route should be registered");
    }

    #[tokio::test]
    async fn test_review_filter_entry() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/filter/entry")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_filter_entry route should be registered");
    }

    #[tokio::test]
    async fn test_review_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_list_job_job route should be registered");
    }

    // SKIPPED: review_v2_count not accessible
    // SKIPPED: review_v2_list not accessible
    #[tokio::test]
    async fn test_review_v2_list_create_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/create/paging/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_create_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_create_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/create/next/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_create_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_create_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/create/prev/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_create_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_paging_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/paging/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_paging_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_review_v2_search() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/v2/search")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_v2_search route should be registered");
    }

    #[tokio::test]
    async fn test_review_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_review_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_id route should be registered");
    }

    #[tokio::test]
    async fn test_review_id_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/application/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_id_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_review_id_application_applicationFlag_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/review/application/manage/mockdeletetoget/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "review_id_application_applicationFlag_manage_mockdeletetoget route should be registered");
    }

    // SKIPPED: route_list not accessible
    #[tokio::test]
    async fn test_route_list_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/route/list/mockputtopost")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "route_list_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_route_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/route/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "route_id route should be registered");
    }

    #[tokio::test]
    async fn test_route_id_selectconfig() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/route/selectconfig/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "route_id_selectconfig route should be registered");
    }

    #[tokio::test]
    async fn test_script_flag_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/script/application/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_flag_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_script_flag_application_applicationFlag_imported() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/script/application/imported/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "script_flag_application_applicationFlag_imported route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_generate_process_processId_name_name_serial() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/generate/process/name/name/serial/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_generate_process_processId_name_name_serial route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_list_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/list/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_list_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_id route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_serialnumber_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/serialnumber/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "serialnumber_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_service_work_id_touch() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/service/work/touch/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "service_work_id_touch route should be registered");
    }

    #[tokio::test]
    async fn test_service_work_id_touch_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/service/work/touch/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "service_work_id_touch_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_sign_download_scrawlId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/download/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_download_scrawlId route should be registered");
    }

    #[tokio::test]
    async fn test_sign_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_sign_save_task_taskId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/save/task/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_save_task_taskId route should be registered");
    }

    #[tokio::test]
    async fn test_sign_task_taskId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/task/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_task_taskId route should be registered");
    }

    #[tokio::test]
    async fn test_sign_task_taskId_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/task/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_task_taskId_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_sign_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_id route should be registered");
    }

    #[tokio::test]
    async fn test_sign_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/sign/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sign_id_mockdeletetoget route should be registered");
    }

    // SKIPPED: task_count_filter not accessible
    #[tokio::test]
    async fn test_task_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_task_filter_attribute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/filter/attribute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_filter_attribute route should be registered");
    }

    #[tokio::test]
    async fn test_task_filter_attribute_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/filter/attribute/filter")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_filter_attribute_filter route should be registered");
    }

    // SKIPPED: task_list_count_application not accessible
    #[tokio::test]
    async fn test_task_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_date_date_hour_hour_exclude_draft_isExcludeDraft_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/date/date/hour/hour/exclude/draft/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_date_date_hour_hour_exclude_draft_isExcludeDraft_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_my_filter_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/my/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_my_filter_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_person_person_exclude_draft_isExcludeDraft_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/person/person/exclude/draft/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_person_person_exclude_draft_isExcludeDraft_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/work/work")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_work_work route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/filter/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_next_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/next/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_next_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/filter/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_list_id_prev_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/list/prev/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_list_id_prev_count_process_processFlag route should be registered");
    }

    // SKIPPED: task_v2_count not accessible
    // SKIPPED: task_v2_list not accessible
    #[tokio::test]
    async fn test_task_v2_list_create_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/create/paging/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_create_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_list_create_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/create/next/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_create_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_list_create_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/create/prev/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_create_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_id_pause() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/pause/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_id_pause route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_id_reset() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/reset/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_id_reset route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_id_reset_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/reset/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_id_reset_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_id_resume() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/resume/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_id_resume route should be registered");
    }

    #[tokio::test]
    async fn test_task_v2_id_trigger_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v2/trigger/processing/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v2_id_trigger_processing route should be registered");
    }

    #[tokio::test]
    async fn test_task_v3_id_add() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v3/add/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v3_id_add route should be registered");
    }

    #[tokio::test]
    async fn test_task_v3_id_pin() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/v3/pin/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_v3_id_pin route should be registered");
    }

    #[tokio::test]
    async fn test_task_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_opinion_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/opinion/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_opinion_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_opinion_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/opinion/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_opinion_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_press_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/press/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_press_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/processing/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_processing route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_processing_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/processing/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_processing_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_processing_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/processing/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_processing_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_processing_neural() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/processing/neural/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_processing_neural route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_reference() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/reference/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_reference route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_reset_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/reset/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_reset_manage route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_reset_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/reset/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_reset_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_task_id_will() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/task/will/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "task_id_will route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_filter_attribute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_filter_attribute route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_filter_attribute_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/filter/attribute/filter")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_filter_attribute_filter route should be registered");
    }

    // SKIPPED: taskcompleted_list_count_application not accessible
    #[tokio::test]
    async fn test_taskcompleted_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_date_date_hour_hour_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/date/date/hour/hour/manage")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_date_date_hour_hour_manage route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_my_filter_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_my_filter_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_prev_manual_flag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/manual/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_prev_manual_flag route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/work/work")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_work_work route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_next_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_next_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_next_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/next/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_next_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_prev_count_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_prev_count_filter route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_list_id_prev_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/list/prev/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_list_id_prev_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_press_work_work() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/press/work/work")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_press_work_work route should be registered");
    }

    // SKIPPED: taskcompleted_v2_count not accessible
    // SKIPPED: taskcompleted_v2_list not accessible
    #[tokio::test]
    async fn test_taskcompleted_v2_list_create_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/paging/test-id/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_create_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_v2_list_create_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/next/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_create_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_v2_list_create_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/create/prev/test-id/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_create_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_opinion_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/opinion/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_opinion_manage route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_opinion_manage_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/opinion/manage/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_opinion_manage_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_reference() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/reference/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_reference route should be registered");
    }

    #[tokio::test]
    async fn test_taskcompleted_id_reference_control() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/taskcompleted/reference/control/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "taskcompleted_id_reference_control route should be registered");
    }

    // SKIPPED: touch_expire not accessible
    // SKIPPED: touch_passexpired not accessible
    // SKIPPED: touch_touchdetained not accessible
    #[tokio::test]
    async fn test_work_application_applicationFlag_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_application_applicationFlag_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_count_credential() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_count_credential route should be registered");
    }

    #[tokio::test]
    async fn test_work_count_credential_application_appId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_count_credential_application_appId route should be registered");
    }

    #[tokio::test]
    async fn test_work_filter_attribute_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/filter/attribute/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_filter_attribute_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_filter_attribute_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/filter/attribute/application/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_filter_attribute_application_applicationFlag_manage route should be registered");
    }

    // SKIPPED: work_list_count_application not accessible
    #[tokio::test]
    async fn test_work_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_count_application_applicationFlag_process_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/application/process/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_count_application_applicationFlag_process_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_my_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/my/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_my_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_paging_page_size_size_application_applicationFlag_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/paging/application/filter/manage/test-id/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_paging_page_size_size_application_applicationFlag_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_application_applicationFlag_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/application/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_application_applicationFlag_filter route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_application_applicationFlag_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/application/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_application_applicationFlag_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/application/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_creator_current() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/creator/current/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_creator_current route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_creator_current_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/creator/current/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_creator_current_filter route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_next_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/next/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_next_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_application_applicationFlag_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/application/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_application_applicationFlag_filter route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_application_applicationFlag_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/application/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_application_applicationFlag_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/application/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_creator_current() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/creator/current/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_creator_current route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_creator_current_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/creator/current/filter/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_creator_current_filter route should be registered");
    }

    #[tokio::test]
    async fn test_work_list_id_prev_count_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/list/prev/process/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_list_id_prev_count_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_work_process_processFlag_force() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/process/force/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_process_processFlag_force route should be registered");
    }

    // SKIPPED: work_v2_list not accessible
    #[tokio::test]
    async fn test_work_v2_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/list/paging/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_list_paging_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_list_id_activity_goback() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/list/activity/goback/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_list_id_activity_goback route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/list/next/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/list/prev/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_add_split() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/add/split/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_add_split route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_add_split_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/add/split/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_add_split_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_reroute() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/reroute/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_reroute route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_reroute_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/reroute/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_reroute_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/retract/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_retract route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_retract_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/retract/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_retract_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_rollback() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/rollback/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_rollback route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_rollback_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/rollback/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_rollback_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_terminate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/terminate/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_terminate route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_terminate_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/terminate/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_terminate_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_v2_id_trigger_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v2/trigger/processing/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v2_id_trigger_processing route should be registered");
    }

    #[tokio::test]
    async fn test_work_v3_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v3/retract")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v3_retract route should be registered");
    }

    #[tokio::test]
    async fn test_work_v3_retract_stage_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v3/retract/stage/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v3_retract_stage_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_work_v3_workorworkcompleted_workOrWorkCompleted_permission() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/v3/workorworkcompleted/permission/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_v3_workorworkcompleted_workOrWorkCompleted_permission route should be registered");
    }

    #[tokio::test]
    async fn test_work_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_work_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_assignment_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/assignment/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_assignment_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_close_check() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/close/check/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_close_check route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/processing/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_processing route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_processing_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/processing/mockputtopost/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_processing_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_projection() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/projection/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_projection route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_refer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/refer/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_refer route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_relative_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/relative/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_relative_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_relative_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/relative/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_relative_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_single_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/single/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_single_manage route should be registered");
    }

    #[tokio::test]
    async fn test_work_id_single_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/work/single/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "work_id_single_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_filter_attribute_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/filter/attribute/application/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_filter_attribute_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_filter_attribute_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/filter/attribute/application/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_filter_attribute_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_filter_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/filter/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_filter_list_id_prev_count_application_applicationFlag route should be registered");
    }

    // SKIPPED: workcompleted_list_count_application not accessible
    #[tokio::test]
    async fn test_workcompleted_list_count_application_applicationFlag_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/application/process/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_count_application_applicationFlag_process route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_count_application_applicationFlag_process_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/application/process/manage/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_count_application_applicationFlag_process_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_filter_page_size_size_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_filter_page_size_size_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_paging_page_size_size_application_applicationFlag_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/paging/application/filter/manage/test-id/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_paging_page_size_size_application_applicationFlag_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_next_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_next_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_next_count_application_applicationFlag_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_next_count_application_applicationFlag_filter route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_next_count_application_applicationFlag_filter_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/filter/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_next_count_application_applicationFlag_filter_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_next_count_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/next/application/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_next_count_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_prev_count_application_applicationFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_prev_count_application_applicationFlag route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_prev_count_application_applicationFlag_filter() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/filter/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_prev_count_application_applicationFlag_filter route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_list_id_prev_count_application_applicationFlag_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/list/prev/application/manage/test-id/test-id/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_list_id_prev_count_application_applicationFlag_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_process_processFlag() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/process/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_process_processFlag route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_shift_time() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/shift/time")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_shift_time route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_flag_rollback() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/rollback/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_flag_rollback route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_flag_rollback_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/rollback/mockputtopost/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_flag_rollback_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_id route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_id_assignment_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/assignment/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_id_assignment_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_id_delete_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/delete/manage/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_id_delete_manage route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_id_delete_manage_mockdeletetoget() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/delete/manage/mockdeletetoget/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_id_delete_manage_mockdeletetoget route should be registered");
    }

    #[tokio::test]
    async fn test_workcompleted_id_manage() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/workcompleted/manage/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "workcompleted_id_manage route should be registered");
    }

    #[tokio::test]
    async fn test_worklog_list_add_split_work_workId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/worklog/list/add/split/work/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "worklog_list_add_split_work_workId route should be registered");
    }

    #[tokio::test]
    async fn test_worklog_list_job_job() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/worklog/list/job/job")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "worklog_list_job_job route should be registered");
    }

    #[tokio::test]
    async fn test_worklog_list_rollback_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/worklog/list/rollback/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "worklog_list_rollback_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

    #[tokio::test]
    async fn test_worklog_list_workorworkcompleted_workOrWorkCompleted() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/processplatform/assemble/surface/worklog/list/workorworkcompleted/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "worklog_list_workorworkcompleted_workOrWorkCompleted route should be registered");
    }

}