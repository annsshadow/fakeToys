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
            ActionResult::success(serde_json::json!({"count": 0, "data": []}));
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
    async fn test_consume_list() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/consume/list/testuser/count/10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_custom_create() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"title": "test", "body": "test", "type": "info", "consumer": "test"}).to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/custom/create")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_mark_read() {
        let pool = build_test_pool();
        // round4-b73：mark_read 已收会话门——注入有效会话 Extension，
        // 保留原断言语义（路由可达 + handler 体直落死池 → 500）。
        let app = crate::router(pool).layer(axum::Extension(shared::testing::test_session(
            "message-mark-read-test",
        )));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/mark_read/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_unread_count() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/unread/count/testuser")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }


    // R317 M2: custom_create 的 INSERT 必须写 xconsumer（consume_list/unread_count 的读列），
    // 且与规范列 consumer 同值绑定；否则新系统消息对任何 consumer 都不可见。
    #[test]
    fn test_custom_create_insert_writes_xconsumer_column() {
        let src = include_str!("lib.rs");
        let start = src
            .find("pub async fn custom_create")
            .expect("custom_create must exist");
        let end = src[start..]
            .find("\npub async fn ")
            .map(|i| start + i)
            .unwrap_or(src.len());
        let func = &src[start..end];
        assert!(
            func.contains("INSERT INTO x_msg_message"),
            "custom_create must INSERT into x_msg_message"
        );
        let cols_start = func
            .find("(id, title, body, type, consumer")
            .expect("INSERT column list must start with canonical columns");
        let cols_end = func[cols_start..]
            .find(')')
            .map(|i| cols_start + i)
            .unwrap();
        let cols = &func[cols_start..cols_end];
        assert!(
            cols.contains("xconsumer"),
            "INSERT column list must include xconsumer so consume_list/unread_count (WHERE xconsumer=$1) can see new messages; got: {}",
            cols
        );
    }

    #[test]
    fn test_custom_create_xconsumer_reuses_consumer_placeholder() {
        let src = include_str!("lib.rs");
        let start = src
            .find("pub async fn custom_create")
            .expect("custom_create must exist");
        let end = src[start..]
            .find("\npub async fn ")
            .map(|i| start + i)
            .unwrap_or(src.len());
        let func = &src[start..end];
        let cols_start = func
            .find("(id, title, body, type, consumer")
            .expect("INSERT column list");
        let cols_end = func[cols_start..].find(')').map(|i| cols_start + i).unwrap();
        let cols = func[cols_start..cols_end].trim_matches(|c| c == '(' || c == ')');
        let values_start = func.find("VALUES (").expect("VALUES clause");
        let values_end = func[values_start..]
            .find(')')
            .map(|i| values_start + i)
            .unwrap();
        let values = func[values_start..values_end]
            .trim_start_matches("VALUES ")
            .trim_matches(|c| c == '(' || c == ')');
        let col_names: Vec<&str> = cols.split(',').map(|s| s.trim()).collect();
        let val_parts: Vec<&str> = values.split(',').map(|s| s.trim()).collect();
        let ci = col_names.iter().position(|&c| c == "consumer").expect("consumer column");
        let xi = col_names.iter().position(|&c| c == "xconsumer").expect("xconsumer column");
        assert_eq!(
            val_parts[xi], val_parts[ci],
            "xconsumer must bind the same placeholder as consumer (同值写入); consumer={} xconsumer={}",
            val_parts[ci], val_parts[xi]
        );
    }

    #[test]
    fn test_consume_list_xcreatetime_null_tolerant() {
        let src = include_str!("lib.rs");
        let start = src
            .find("pub async fn consume_list")
            .expect("consume_list must exist");
        let end = src[start..]
            .find("\npub async fn ")
            .map(|i| start + i)
            .unwrap_or(src.len());
        let func = &src[start..end];
        assert!(
            func.contains("get::<_, Option<String>>(\"xcreateTime\")"),
            "consume_list must read nullable xcreateTime as Option<String> to avoid deserialization panic; got prefix: {}",
            &func[..func.len().min(240)]
        );
    }
}

