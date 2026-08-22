//! Probe: build the FULL merged application (all ~50 crate routers) to surface
//! any axum route-conflict panic at Router construction time (plan002 U2).

#[tokio::test]
async fn u2_probe_full_app_build() {
    let pg_pool = shared::testing::test_pool();
    let session_manager = shared::session::SessionManager::with_pool(pg_pool.clone());
    let rate_limiter = shared::rate_limit::RateLimiter::new();
    // create_app merges every crate router; axum panics on overlapping paths.
    let app = oa4rust::create_app(pg_pool.clone(), session_manager.clone(), rate_limiter)
        .await;
    let _ = app;
}
