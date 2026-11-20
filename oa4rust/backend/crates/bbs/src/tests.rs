// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use shared::response::ActionResult;
    use tower::ServiceExt;

    fn build_test_pool() -> deadpool_postgres::Pool {
        deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_router_builds() {
        let pool = build_test_pool();
        let _ = crate::router(pool);
    }

    #[tokio::test]
    async fn test_forum_view_all() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/forum/view/all")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_forum_view_one() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/forum/view/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_section_view_all() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/section/view/all")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_subject_list() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/subject/list/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_subject_create_writes_active_bbs_topic_table() {
        // R302 BBS 回显缺陷回归：PC 默认/我的/版块筛选列表均读 x_bbs_topic，
        // 发帖接口若继续写遗留表 bbs_subject_info，新建主题刷新后必然消失。
        let source = include_str!("subject.rs");
        let create = &source[source
            .find("pub async fn create(")
            .expect("subject::create must exist")..];
        assert!(
            create.contains("INSERT INTO x_bbs_topic"),
            "POST /api/bbs/subject/create must insert into active table x_bbs_topic"
        );
        assert!(
            !create.contains("INSERT INTO bbs_subject_info"),
            "subject::create must not write legacy bbs_subject_info"
        );
        assert!(
            create.contains("reject_impersonation"),
            "session-authoritative impersonation guard must stay in subject::create"
        );
        for column in [
            "author_id",
            "creator",
            "forum_id",
            "section_id",
            "section_name",
            "subject_type",
        ] {
            assert!(
                create.contains(column),
                "x_bbs_topic insert must map column {column}"
            );
        }
    }

    #[tokio::test]
    async fn test_subject_create() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"title": "test", "body": "test"}).to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/bbs/subject/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
