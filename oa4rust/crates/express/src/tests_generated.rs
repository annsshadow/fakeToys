#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::{Extension, Path, Json};
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::{test_pool, test_sea_orm_pool};
    use tower::util::ServiceExt;

    // SKIPPED: express_person_list has unresolvable params
    // SKIPPED: express_unit_list has unresolvable params
    // SKIPPED: express_identity_list has unresolvable params
    // SKIPPED: express_group_list has unresolvable params
    // SKIPPED: express_role_list has unresolvable params
    // SKIPPED: express_person_with_unit has unresolvable params
    // SKIPPED: express_person_with_identity has unresolvable params
    #[tokio::test]
    async fn test_get_express_info() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/query")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "get_express_info route should be registered");
    }

    #[tokio::test]
    async fn test_list_express_companies() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/companies")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "list_express_companies route should be registered");
    }

    #[tokio::test]
    async fn test_subscribe_express() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/express/subscribe")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "subscribe_express route should be registered");
    }

}