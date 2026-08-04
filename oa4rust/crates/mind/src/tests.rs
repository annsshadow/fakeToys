#[cfg(test)]
mod tests {
    use crate::mind_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_folder_route_accessible() {
        let mut app = mind_router();

        let request = Request::builder()
            .uri("/jaxrs/mind/folder/tree/my")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_mind_route_accessible() {
        let app = mind_router();

        let request = Request::builder()
            .uri("/jaxrs/mind/mind/test-id")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_version_route_accessible() {
        let app = mind_router();

        let request = Request::builder()
            .uri("/jaxrs/mind/mind/list/test-id/version")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}

