// 设计器前端调用点 ↔ 后端路由 逐条实测对账（docs/plans §9.4 的可复现硬证据）
//
// 用 axum Router::oneshot 命中各设计器前端真实调用路径，实测区分：
//   404 Absent   = 无任何路由匹配（前端调用是"哑弹"，最易发现）
//   405 Method   = 路径被宽 `{id}` 路由影子捕获，但 HTTP 动词不支持（报错，但仍非静默）
//   Matched(≠404/≠405, 通常 500/415) = 已进 handler。若前端"想要列表/统计/脚本"而 handler
//       是"按 id 取单个"，即 **静默错数据（silent shadow misroute）**——生产带 DB 时返回 200 错体，
//       这里因无 DB 表现为 500；GET 类命中即属最危险一档。
// 与仓库既有 idiom 一致（crates/query_assemble_designer/src/tests.rs: build_test_pool + oneshot，
// 断言 route exists 为 INTERNAL_SERVER_ERROR）。
//
// 本测试把 2026-09-11 HEAD ab25b16b 的实测分类固化为断言：任一设计器路由被修复或回归都会让
// 相应 case 失败，从而强制更新 §9.4。W6 完成某条后，把该 case 期望改为 Matched 并附真实语义。

use axum::body::Body;
use axum::http::{Method, Request};
use axum::Router;
use deadpool_postgres::tokio_postgres::{Config, NoTls};
use deadpool_postgres::{Manager, Pool};
use tower::util::ServiceExt;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Expect {
    Absent, // 期望 404：确证无任何路由
    Method, // 期望 405：路径存在但方法不符（影子捕获）
    Matched, // 期望 ≠404 且 ≠405：已进入 handler
}

fn lazy_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr).max_size(1).build().expect("lazy pool build (no connect)")
}

fn app() -> Router {
    Router::new()
        .merge(processplatform_assemble_designer::router(lazy_pool()))
        .merge(query_assemble_designer::router(lazy_pool()))
        .merge(portal_assemble_designer::router(lazy_pool()))
        .merge(cms_assemble_control::router(lazy_pool()))
}

fn p(t: &str) -> String {
    format!("/jaxrs/processplatform/assemble/designer{t}")
}
fn q(t: &str) -> String {
    format!("/jaxrs/query/assemble/designer{t}")
}
fn t_(t: &str) -> String {
    format!("/jaxrs/portal/assemble/designer{t}")
}
fn f(t: &str) -> String {
    format!("/jaxrs/form{t}")
}

fn classify(code: u16) -> Expect {
    match code {
        404 => Expect::Absent,
        405 => Expect::Method,
        _ => Expect::Matched,
    }
}

#[tokio::test]
async fn designer_route_reconciliation() {
    // (method, path, 实测期望分类, 前端意图)
    let cases: Vec<(Method, String, Expect, &str)> = vec![
        // ── Process designer ──
        (Method::GET, p("/process/abc"), Expect::Matched, "load 单流程(真匹配)"),
        (Method::PUT, p("/process/abc"), Expect::Matched, "save 更新(真匹配)"),
        (Method::POST, p("/process"), Expect::Absent, "新建→前端 POST 裸 /process，后端为 /create → 404"),
        (Method::GET, p("/process/list"), Expect::Matched, "列表→影子命中 process/{id} id=list（静默错数据）"),
        (Method::GET, p("/process/export"), Expect::Matched, "导出→影子命中 process/{id}（静默）"),
        (Method::POST, p("/create"), Expect::Matched, "control：真实新建路径存在"),
        // ── Query designer ──
        (Method::GET, q("/list"), Expect::Absent, "裸 list 无(仅 list/{category}) → 404"),
        (Method::PUT, q("/update/abc"), Expect::Method, "PUT update 被 /{id}/{count} 影子捕获但仅 GET → 405"),
        (Method::POST, q("/execute"), Expect::Absent, "裸 execute 无(仅 table/execute/{flag}) → 404"),
        (Method::POST, q("/create"), Expect::Matched, "control"),
        (Method::DELETE, q("/delete/abc"), Expect::Matched, "delete/{id} 实测接受 DELETE → 真匹配"),
        (Method::GET, q("/table/list"), Expect::Matched, "裸 table/list 命中 table/{flag}（静默）"),
        (Method::POST, q("/stat/do"), Expect::Method, "POST stat/do 命中 stat/{id} 但无 POST → 405"),
        (Method::GET, q("/entity/entity/properties/tbl/default/default"), Expect::Matched, "3 参数匹配 → 真匹配"),
        // ── Portal designer ──
        (Method::GET, t_("/page/list"), Expect::Matched, "裸 page/list 命中 page/{id}（静默）"),
        (Method::GET, t_("/script/list"), Expect::Matched, "裸 script/list 命中 script/{id}（静默）"),
        (Method::PUT, t_("/page/abc"), Expect::Matched, "PUT page/{id} → 真匹配"),
        (Method::POST, t_("/page"), Expect::Matched, "裸 POST page → create_page 存在"),
        (Method::DELETE, t_("/page/abc"), Expect::Matched, "DELETE page/{id} → 真匹配"),
        (Method::PUT, t_("/script/abc"), Expect::Method, "PUT script/{id} 后端仅 GET → 405"),
        (Method::POST, t_("/script"), Expect::Absent, "裸 POST script → 404"),
        (Method::POST, t_("/script/run"), Expect::Method, "POST script/run 命中 script/{id} 但无 POST → 405"),
        // ── Form designer (FormDesigner.vue, base /jaxrs/form, served by cms_assemble_control) ──
        (Method::GET, f("/list"), Expect::Matched, "GET form/list 裸→影子命中 form/{id} id=list（静默错数据）"),
        (Method::GET, f("/abc"), Expect::Matched, "control：GET form/{id}"),
        (Method::PUT, f("/abc"), Expect::Matched, "PUT form/{id}（update，真匹配）"),
        (Method::POST, f(""), Expect::Matched, "POST form 裸（create，真匹配）"),
        (Method::POST, f("/submit"), Expect::Method, "POST form/submit→命中 form/{id} 但无 POST 或无 submit → 校验"),
    ];

    let mut lines = Vec::new();
    let mut fails = Vec::new();

    for (method, path, expect, intent) in &cases {
        let req = Request::builder()
            .method(method.clone())
            .uri(path.as_str())
            .body(Body::empty())
            .unwrap();
        let code = app().oneshot(req).await.unwrap().status().as_u16();
        let got = classify(code);
        lines.push(format!(
            "{:<7} {:<4} {:<60} -> {:<4} 期望={:?} got={:?} {}",
            path.split('/').nth(4).unwrap_or("?"), // designer seg
            method_tag(method),
            path,
            code,
            expect,
            got,
            if expect == &got { "" } else { " <<< MISMATCH" }
        ));
        if expect != &got {
            fails.push(format!("{} {}{} 期望 {:?} 实测 {}({:?})", intent, method_tag(method), path, expect, code, got));
        }
    }

    println!("\n===== §9.4 设计器路由对账 · 实测复现（HEAD ab25b16b） =====");
    for l in &lines {
        println!("{l}");
    }
    println!("======================================================\n");

    assert!(fails.is_empty(), "对账分类与实测不符（§9.4 需更新）：\n{}", fails.join("\n"));
}

fn method_tag(m: &Method) -> &'static str {
    if *m == Method::GET {
        "GET"
    } else if *m == Method::POST {
        "POST"
    } else if *m == Method::PUT {
        "PUT"
    } else if *m == Method::DELETE {
        "DEL"
    } else {
        "OTH"
    }
}
