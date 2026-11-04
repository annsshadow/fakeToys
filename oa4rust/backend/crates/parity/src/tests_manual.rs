// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// O15：从 parity 生成物里分流出来的手工内容。
//
// 唯一一条为语义化命名的 parity 断言（2026-08-19 手工加入）。生成器只产
// `parity__<crate>__crate[_N]` 序号名，无法复现这个名字；断言体与
// `tests_generated.rs` 的 `parity__calendar_assemble_control__crate_31` 相同。
// 保留它是为了不丢人工命名痕迹，新增手写内容一律放这里，不要再改生成物。

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use axum::http::StatusCode;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn parity__calendar_assemble_control__get_calendar_detail() {
        let router = oa4rust::calendar_assemble_control::router(shared::testing::test_pool());
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/calendar/assemble/control/calendar/detail/test-id")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("oneshot dispatch");
        let (parts, body) = response.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("read parity body");
        assert!(
            parts.status != StatusCode::NOT_FOUND || !bytes.is_empty(),
            "parity: route missing on calendar_assemble_control: /api/calendar/assemble/control/calendar/detail/{{id}} (GET)"
        );
    }
}
