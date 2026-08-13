use super::*;
use shared::response::ActionResult;
use serde_json::json;
use axum::body::Body;
use axum::http::{Request, Method, StatusCode};
use deadpool_postgres::{Manager, Pool};
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use tower::util::ServiceExt;



#[test]
fn test_action_result_success_serialization() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({"count": 2, "data": []}));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/get/control/config")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_list_control_sections_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/list/control/sections")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_update_control_config_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req_body = serde_json::to_string(&json!({"enabled": true, "maxCategoryCount": 300})).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jaxrs/cms_assemble_control/update/control/config")
                .method(Method::GET)
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
#[cfg(test)]
mod tests {

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            Config::new(),
            NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }


    #[tokio::test]
    async fn test_get_jaxrs_application_id() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/application/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_get_control_c() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/get/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_list_control_() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/list/control/sections")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_cms_assemble_control_update_contro() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/cms_assemble_control/update/control/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_commend_list_paging_docId() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/commend/list/paging/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_jaxrs_queryview_flag_view_definition_que() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/queryview/flag/test-id/definition/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_jaxrs_document_id_view_count() {
        let pool = build_test_pool();
        let app = crate::cms_assemble_control_router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/document/test-id/view/count")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}
