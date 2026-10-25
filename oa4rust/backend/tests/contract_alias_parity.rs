// o2server 契约路径别名 ↔ 旧路径 的全栈 parity 实测（2026-10-24 R1）
//
// 别名路由把 /api/organization/assemble/personal/* 原样委托旧路径
// （/api/person/*、/api/reset/*）的同一 handler。本测试穿过**真实中间件栈**
// （auth 豁免 / 限流 / CSRF / RBAC / admin 写门）对两条路径各打一次匿名
// 请求，要求状态码逐对相等：
//   - 404（别名）≠ 非 404（旧径）→ 别名没注册上（路由层回归）
//   - 401 差异 → 中间件四张表有表忘了补镜像（豁免/权限）
//   - 403 差异 → admin 写门不一致（安全弱化或过严）
// handler 之后的业务码（无 PG 时 500/400/415）两侧同一 handler 天然一致。
//
// 中间件判定层的单元级 parity 见 shared crate 的
// contract_alias_parity_tests（security.rs 尾部）；本文件是**全栈**证据。

use axum::body::Body;
use axum::http::{Method, Request};
use axum::Router;
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use tower::util::ServiceExt;

fn lazy_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr)
        .max_size(1)
        .build()
        .expect("lazy pool build (no connect)")
}

const PAIRS: &[(&str, &str, &str, &str)] = shared::middleware::contract_alias::CONTRACT_ALIAS_PAIRS;

/// 把路径里的 {param} 占位符替换成具体值（两侧同值，handler 解析失败也同败）。
fn concrete(path: &str) -> String {
    let mut out = String::new();
    let mut rest = path;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let close = rest[open..].find('}').expect("占位符闭合") + open;
        out.push('1');
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}

async fn app() -> Router {
    let pool = lazy_pool();
    let session_manager = shared::session::SessionManager::with_pool(pool.clone());
    let rate_limiter = shared::rate_limit::RateLimiter::new();
    oa4rust::create_app(pool, session_manager, rate_limiter)
        .await
        .expect("create_app 必须能在无 PG 的测试环境构建（惰性建连）")
}

async fn status(app: &Router, method: &Method, path: &str) -> axum::http::StatusCode {
    let req = Request::builder()
        .method(method.clone())
        .uri(concrete(path))
        .body(Body::empty())
        .expect("请求构造");
    app.clone().oneshot(req).await.expect("oneshot").status()
}

#[tokio::test]
async fn alias_and_old_path_behave_identically_through_full_stack() {
    let app = app().await;
    let mut checked = 0usize;
    for (am, alias, om, old) in PAIRS {
        let a_method: Method = am.parse().expect("合法方法");
        let o_method: Method = om.parse().expect("合法方法");
        let a_status = status(&app, &a_method, alias).await;
        let o_status = status(&app, &o_method, old).await;
        assert_ne!(
            a_status,
            axum::http::StatusCode::NOT_FOUND,
            "别名 {am} {alias} 未注册（404）——委托路由缺失"
        );
        assert_eq!(
            a_status, o_status,
            "{am} {alias} -> {a_status}，但旧径 {om} {old} -> {o_status}：中间件判定或路由形态不一致"
        );
        checked += 1;
    }
    assert!(checked >= 70, "测试表意外缩小：仅 {checked} 对");
}
