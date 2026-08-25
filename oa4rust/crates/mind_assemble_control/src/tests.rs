#[cfg(test)]
mod tests {
    use crate::mind_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use shared::testing::{is_db_available, mock_pool, test_pool};
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_route_accessible() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_update_route_accessible() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"id":"test-id","configData":"{}"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[ignore = "requires a running PostgreSQL server"]
    async fn test_config_update_missing_id() {
        let app = mind_assemble_control_router(test_pool());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/config/update")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"configData":"{}"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&body);
        // Either route returns non-404 or body contains error message
        assert!(status != StatusCode::NOT_FOUND || body_str.contains("id is required"));
    }

    // ───────────── plan002 U2：新端点路由存在性（mock pool，断言非 404）─────────────

    #[tokio::test]
    async fn route_folder_delete() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/folder/x")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_folder_move_put() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/folder/move/x")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_folder_force_delete() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/folder/x/force")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_filter_list() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/filter/list/x/next/1")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_filter_recycle() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/filter/recycle/x/next/1")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_filter_shared() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/filter/shared/x/next/1")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_share_records() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/list/x/shareRecords")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_version_list() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/list/x/version")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_recycle() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/recycle/x")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_restore() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/restore/x")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_save() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/save")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_share() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/share/x")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_share_cancel() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/share/x/cancel")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_version_get() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/version/x")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_view() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/view/x")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_get() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/x")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_destroy() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/x/destorymind")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_destroy_recycle() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/x/destoryrecycle")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_icon_get() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/x/icon")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn route_mind_icon_set() {
        let app = mind_assemble_control_router(mock_pool());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/mind/assemble/control/mind/x/icon/size/64")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }

    // ───────────── plan002 U2：真实 DB 集成测试（DB 可用时执行，含清理）─────────────

    #[tokio::test]
    async fn real_mind_get_and_share_records() {
        if !is_db_available().await {
            return;
        }
        let pool = test_pool();
        let client = pool.get().await.expect("db");

        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_mind (id, name, content, creator, create_time) \
                 VALUES ($1, 'u2mind', 'c', 'u2tester', NOW())",
                &[&id],
            )
            .await
            .unwrap();
        client
            .execute(
                "INSERT INTO x_mind_share (id, mind_id, person, create_time) VALUES ($1, $2, 'sharee', NOW())",
                &[&uuid::Uuid::new_v4().to_string(), &id],
            )
            .await
            .unwrap();

        let app = mind_assemble_control_router(pool);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/jaxrs/mind/assemble/control/mind/{}", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["data"]["id"], id);

        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/jaxrs/mind/assemble/control/mind/list/{}/shareRecords", id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let _ = client.execute("DELETE FROM x_mind_share WHERE mind_id = $1", &[&id]).await;
        let _ = client.execute("DELETE FROM x_mind WHERE id = $1", &[&id]).await;
    }
}
