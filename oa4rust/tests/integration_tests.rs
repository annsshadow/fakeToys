use shared::{db::create_pool, rate_limit::RateLimiter, session::SessionManager};

/// 验证 80 个 crate 路由能够合并构建而不 panic。
///
/// 完整 Router 的构造在默认约 1–2MB 的线程栈上会栈溢出（已确认是栈大小需求，
/// 非无限递归；64MB 即可满足）。为以确定性方式规避该问题，这里在显式分配
/// 256MB 栈的 OS 线程中构建 runtime 并以 `block_on` 执行 `create_app`——
/// `block_on` 的 future 直接运行在该 256MB 栈的 std 线程上，完全不依赖 tokio
/// 的 `RUST_MIN_STACK` 环境变量（其在部分 tokio 版本中读取时机不确定）。
///
/// 该测试仅验证路由合并不 panic；`create_pool` 仅构建连接池管理器、
/// 不真正建立数据库连接，故无需可用的 PostgreSQL 即可运行。
#[test]
fn test_all_routes_merge_without_panic() {
    let handle = std::thread::Builder::new()
        .name("route-merge-check".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let pool = create_pool().await.expect("failed to create pool");
                let session_manager = SessionManager::new();
                let rate_limiter = RateLimiter::new();
                oa4rust::create_app(pool, session_manager, rate_limiter)
                    .await
                    .expect("router build should not panic");
            });
        })
        .expect("failed to spawn check thread");
    handle.join().expect("route merge check thread panicked");
}
