//! Probe: build the FULL merged application (all ~50 crate routers) to surface
//! any axum route-conflict panic at Router construction time (plan002 U2).

#[tokio::test]
async fn u2_probe_full_app_build() {
    let pg_pool = shared::testing::test_pool();
    let session_manager = shared::session::SessionManager::with_pool(pg_pool.clone());
    let rate_limiter = shared::rate_limit::RateLimiter::new();
    // create_app merges every crate router; axum panics on overlapping paths.
    let app = oa4rust::create_app(pg_pool.clone(), session_manager.clone(), rate_limiter).await;
    let _ = app;
}

// P5：IM 实时协议 WebSocket 握手的全中间件栈可执行证据。
//
// /ws/realtime 不在 AUTH_EXEMPT（IM 属需登录功能）。未认证握手经全栈：
//   rate_limit（非豁免→内存/Redis 限流，单次不超限）
//   → csrf_middleware（GET 直接放行——仅拦 POST/PUT/PATCH/DELETE 写方法）
//   → auth_middleware（非豁免且无会话 Cookie/Bearer → 401 短路，不触 DB）
// 因此实测 401 同时证明：路由已注册（≠404）、auth 确实拦在握手前（≠101/200）、
// CSRF 未误伤 GET 升级请求。
#[tokio::test]
async fn p5_ws_realtime_handshake_is_auth_guarded() {
    use axum::http::{Method, StatusCode};

    let pg_pool = shared::testing::test_pool();
    let session_manager = shared::session::SessionManager::with_pool(pg_pool.clone());
    let rate_limiter = shared::rate_limit::RateLimiter::new();
    let app = oa4rust::create_app(pg_pool, session_manager, rate_limiter)
        .await
        .expect("create_app must build in non-production test env");

    let status = shared::testing::send(&app, Method::GET, "/ws/realtime", None, None).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "未认证 /ws/realtime 握手应被 auth_middleware 拦截为 401（404=未注册，101/200=未拦 auth）"
    );

    let room_status =
        shared::testing::send(&app, Method::GET, "/ws/realtime/room/default", None, None).await;
    assert_eq!(
        room_status,
        StatusCode::UNAUTHORIZED,
        "未认证 /ws/realtime/room/{{room_id}} 握手同样须被 auth 拦截"
    );
}
