// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_user_setting() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/organization/assemble/personal/test-id/setting")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(), "user_setting route should be registered (兜底空体 404)");
        }
    }

    #[tokio::test]
    async fn test_user_role_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/organization/assemble/personal/test-id/role/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
            assert!(!bytes.is_empty(), "user_role_list route should be registered (兜底空体 404)");
        }
    }
}
