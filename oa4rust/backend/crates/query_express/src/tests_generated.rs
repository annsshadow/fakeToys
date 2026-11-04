// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_query_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "query_list route should be registered");
        }
    }

    // SKIPPED: create_query requires Session parameter
}
