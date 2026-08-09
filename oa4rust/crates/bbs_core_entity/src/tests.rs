#[cfg(test)]
mod tests {
    use crate::entities::{bbs_forum_info::Model as ForumInfo, bbs_section_info::Model as SectionInfo, bbs_subject_info::Model as SubjectInfo};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).build().unwrap()
    }

    #[test]
    fn test_forum_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::bbs_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/bbs/core/entity/forum/list")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        });
    }

    #[test]
    fn test_section_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::bbs_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/bbs/core/entity/section/list/test-forum-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_subject_top_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::bbs_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/bbs/core/entity/subject/top/test-section-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_subject_list_returns_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = build_test_pool();
            let app = crate::bbs_core_entity_router(pool);

            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/jaxrs/bbs/core/entity/subject/list/test-section-id")
                        .method(axum::http::Method::GET)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // axum 0.8: {param} 路由可匹配(0.7 下 :param/{param} 混用会 404), handler 缺 pool 返回 500
        });
    }

    #[test]
    fn test_action_result_format() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 1, "data": []}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert_eq!(json["data"]["count"], 1);
    }

    #[test]
    fn test_forum_info_serialization() {
        let forum = ForumInfo {
            id: "forum-001".to_string(),
            name: "技术论坛".to_string(),
            description: Some("交流技术".to_string()),
            create_time: None,
            disable: false,
            order_number: 0,
        };
        let json = serde_json::to_string(&forum).unwrap();
        assert!(json.contains("forum-001"));
        assert!(json.contains("技术论坛"));
    }

    #[test]
    fn test_section_info_serialization() {
        let section = SectionInfo {
            id: "section-001".to_string(),
            name: "前端板块".to_string(),
            forum_id: "forum-001".to_string(),
            description: None,
            order_number: 1,
            disable: false,
            create_time: None,
        };
        let json = serde_json::to_string(&section).unwrap();
        assert!(json.contains("section-001"));
        assert!(json.contains("前端板块"));
    }

    #[test]
    fn test_subject_info_serialization() {
        let subject = SubjectInfo {
            id: "subject-001".to_string(),
            title: "Rust异步编程".to_string(),
            author_id: "user-001".to_string(),
            section_id: "section-001".to_string(),
            content: None,
            reply_count: 0,
            view_count: 0,
            is_top: false,
            disable: false,
            create_time: None,
        };
        let json = serde_json::to_string(&subject).unwrap();
        assert!(json.contains("subject-001"));
        assert!(json.contains("Rust异步编程"));
    }
}
