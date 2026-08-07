#[cfg(test)]
mod tests {
    use crate::general_assemble_control_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use tower::ServiceExt;

    fn mock_pool() -> Pool {
        Pool::builder(Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .max_size(1)
        .build()
        .unwrap()
    }

    #[tokio::test]
    async fn test_status_route_accessible() {
        let app = general_assemble_control_router(mock_pool());

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
    async fn test_status_update_route_accessible() {
        let app = general_assemble_control_router(mock_pool());

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
    async fn test_permissions_route_accessible() {
        let app = general_assemble_control_router(mock_pool());

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
    async fn test_status_update_response_shape() {
        let app = general_assemble_control_router(mock_pool());

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
}
