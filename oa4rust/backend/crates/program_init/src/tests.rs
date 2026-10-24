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
            ActionResult::success(serde_json::json!({"initialized": false}));
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
    async fn test_secret_check() {
        std::env::remove_var("SECRET_ENCRYPTION_KEY");
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/secret/check")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_secret_set() {
        std::env::remove_var("SECRET_ENCRYPTION_KEY");
        let pool = build_test_pool();
        let app = crate::router(pool);

        let body = serde_json::json!({"secret": "test-secret"}).to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/secret/set")
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
    async fn test_secret_set_cancel() {
        std::env::remove_var("SECRET_ENCRYPTION_KEY");
        let pool = build_test_pool();
        let app = crate::router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/secret/set/cancel")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}

#[cfg(test)]
mod datasource_batch_insert_tests {
    use super::super::datasource_row_placeholders;

    #[test]
    fn datasource_placeholders_are_globally_sequential() {
        // 多行 INSERT 的占位符必须全局连续（$1..$10, $11..$20 …）：
        // 按行重置编号会让第二行起参数整体错位——且不报错，
        // 只是静默写错值（2026-10-24 多行 VALUES 改造的回归钉）。
        const COLS: usize = 10; // 与 lib.rs 的 DATASOURCE_COLS 一致
        let mut all: Vec<String> = Vec::new();
        let mut base = 1;
        for _ in 0..3 {
            all.extend(datasource_row_placeholders(base));
            base += COLS;
        }
        let expected: Vec<String> = (1..=30).map(|n| format!("${n}")).collect();
        assert_eq!(all, expected);
    }

    // 参数上限不变量（DATASOURCE_MAX_ROWS × COLS ≤ 65535）是编译期性质，
    // 已改为 lib.rs 里的 const 断言——运行时断言恒真，clippy deny 门下报错。
}
