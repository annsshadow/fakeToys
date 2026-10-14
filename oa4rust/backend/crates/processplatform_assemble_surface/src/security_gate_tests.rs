// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 R4-7）：本 crate 的
//! `GET /api/processplatform/assemble/surface/application/list/complex/manage/{person}`
//! 与其字面量别名 `…/manage/person`。
//!
//! 这一格是「修好写门禁尺子才露出来」的真匿名写：路径落在
//! `shared::middleware::constants::AUTH_EXEMPT_PREFIXES` 的
//! `/api/processplatform/assemble/surface/application/list` 里（auth_middleware 直接放行），
//! 又注册在 **GET** 上（csrf_middleware 与 requires_admin 都只认 POST/PUT/PATCH/DELETE），
//! 所以中间件层两道门一道都够不着；而 handler 原本不取 Session，函数体第一条 SQL 就是
//! `UPDATE PP_E_APPLICATION SET "xupdateTime" = NOW() WHERE xid = $1`，随后按同一 ID 回读
//! createTime/updateTime 返回。即：**未认证**请求可按 ID 改任意应用的时间戳（审计字段伪造、
//! 可批量枚举），并拿到该应用的存在性与时间信息。
//!
//! 现在按应用管理面口径收口（桌面侧只有运维控制台弹窗手输人员会调它，不是终端用户流程）：
//! 无会话 401（先于任何 SQL，可离线证），非 admin 403（`shared::middleware::is_admin`
//! 无库 fail-closed）。
#[cfg(test)]
mod security_gate_tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;

    const MANAGE_URI: &str =
        "/api/processplatform/assemble/surface/application/list/complex/manage/r47-target";
    const MANAGE_ALIAS_URI: &str =
        "/api/processplatform/assemble/surface/application/list/complex/manage/person";

    fn app() -> axum::Router {
        crate::router(shared::testing::test_pool())
    }

    async fn get(uri: &str) -> StatusCode {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        resp.status()
    }

    async fn get_as(uri: &str, person: &str) -> StatusCode {
        let session = shared::testing::test_session(person);
        let resp = app()
            .layer(axum::Extension(session))
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        resp.status()
    }

    /// 未认证必须 401，而且是在拿到连接、发任何 SQL 之前——门在 `pool.get()` 之上。
    #[tokio::test]
    async fn r47_manage_touch_rejects_without_session() {
        assert_eq!(
            get(MANAGE_URI).await,
            StatusCode::UNAUTHORIZED,
            "参数位路由：匿名 GET 写必须 401"
        );
        assert_eq!(
            get(MANAGE_ALIAS_URI).await,
            StatusCode::UNAUTHORIZED,
            "字面量别名路由与参数位路由同 handler，必须同口径"
        );
    }

    /// 有效会话但非 admin 必须 403。用一个绝不会被授予角色的唯一身份，
    /// 因此该断言与测试库里是否恰好有 admin 数据无关。
    #[tokio::test]
    async fn r47_manage_touch_forbidden_for_non_admin() {
        assert_eq!(
            get_as(MANAGE_URI, "r47-plain-person-no-role").await,
            StatusCode::FORBIDDEN,
            "非 admin 按 ID touch 他人应用时间戳必须 403"
        );
        assert_eq!(
            get_as(MANAGE_ALIAS_URI, "r47-plain-person-no-role").await,
            StatusCode::FORBIDDEN,
            "别名路由同口径"
        );
    }

    /// 四轮 R4-2 追加：`service_work_id_touch` / `service_work_id_touch_mockputtopost`
    /// 四把路由（PUT/POST × 正式/别名）原先不取会话，任一有效会话按 work ID 就能
    /// `UPDATE PP_C_WORK."xupdateTime"` 并回读时间字段。无会话必须 401，且发生在
    /// `pool.get()` 之前——所以本支不碰库，离线可证。
    #[tokio::test]
    async fn surface_touch_writes_reject_without_session() {
        for (method, uri) in [
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/touch/r47-work",
            ),
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/touch/mockputtopost/r47-work",
            ),
            (
                Method::PUT,
                "/api/processplatform/assemble/surface/service/work/r47-work/touch",
            ),
            (
                Method::POST,
                "/api/processplatform/assemble/surface/service/work/r47-work/touch/mockputtopost",
            ),
        ] {
            let resp = app()
                .oneshot(
                    Request::builder()
                        .uri(uri)
                        .method(method.clone())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                resp.status(),
                StatusCode::UNAUTHORIZED,
                "无会话的 touch 写必须 401（门在取连接之前）: {method} {uri}"
            );
        }
    }
}
