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
    Absent,  // 期望 404：确证无任何路由
    Method,  // 期望 405：路径存在但方法不符（影子捕获）
    Matched, // 期望 ≠404 且 ≠405：已进入 handler
}

fn lazy_pool() -> Pool {
    let mgr = Manager::new(Config::new(), NoTls);
    Pool::builder(mgr)
        .max_size(1)
        .build()
        .expect("lazy pool build (no connect)")
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
    // W6+W7 闭合后（2026-09-12）：前端真实调用点已全部改打真实路由或由 u2 闭合路由承接；
    // 仍留影子的 case 仅剩 portal 页设计器壳（page/list，W13 待闭合）。
    let cases: Vec<(Method, String, Expect, &str)> = vec![
        // ── Process designer（ProcessDesigner.vue 当前调用面：create/save/get/list 全真实路由）──
        (
            Method::POST,
            p("/create"),
            Expect::Matched,
            "新建 create_flow（W6：前端弃裸 POST /process）",
        ),
        (
            Method::POST,
            p("/save/abc"),
            Expect::Matched,
            "保存 save_flow（x_process_definition）",
        ),
        (
            Method::GET,
            p("/get/abc"),
            Expect::Matched,
            "加载/导出 get_flow（W6：前端弃 process/export 影子）",
        ),
        (
            Method::GET,
            p("/list/all"),
            Expect::Matched,
            "列表 list/{category}=all（W6：前端弃 process/list 影子）",
        ),
        // ── Query designer（QueryStatementDesigner/QueryManagerDeep/QueryDesigner 当前调用面）──
        (
            Method::GET,
            q("/list/all"),
            Expect::Matched,
            "列表 list/{category}=all（W6：前端弃裸 /list）",
        ),
        (
            Method::PUT,
            q("/save/abc"),
            Expect::Matched,
            "保存 save_designer PUT 变体（W6：前端改名 update→save）",
        ),
        (
            Method::POST,
            q("/execute"),
            Expect::Matched,
            "即时 SQL designer_execute（W6 ③ 补缺：仅单条 SELECT+LIMIT 500）",
        ),
        (
            Method::POST,
            q("/stat/do"),
            Expect::Matched,
            "统计聚合 stat_do（W6 ③ 补缺：dimension 分组/metric 求和）",
        ),
        (Method::POST, q("/create"), Expect::Matched, "control"),
        (
            Method::DELETE,
            q("/delete/abc"),
            Expect::Matched,
            "delete/{id} 实测接受 DELETE → 真匹配",
        ),
        (
            Method::GET,
            q("/table/list/manage"),
            Expect::Matched,
            "表清单 manage 路由（W6：前端弃 table/list 影子）",
        ),
        (
            Method::GET,
            q("/entity/entity/properties/tbl/default/default"),
            Expect::Matched,
            "3 参数匹配 → 真匹配",
        ),
        // ── Portal designer（新版 PortalDesigner 走 /list /create /get/{id} /save/{id}；
        //    脚本设计器 W7 已实装：u2 写路径 + list/manager；page/list 影子仍来自页壳 W13）──
        (
            Method::GET,
            t_("/list"),
            Expect::Matched,
            "新 PortalDesigner 列表（u2 闭合路由）",
        ),
        (
            Method::POST,
            t_("/create"),
            Expect::Matched,
            "新 PortalDesigner 新建",
        ),
        (
            Method::GET,
            t_("/get/abc"),
            Expect::Matched,
            "新 PortalDesigner 加载",
        ),
        (
            Method::POST,
            t_("/save/abc"),
            Expect::Matched,
            "新 PortalDesigner 保存（content=serializePortalContent）",
        ),
        (
            Method::GET,
            t_("/page/list"),
            Expect::Matched,
            "PortalPageDesignerApp 壳裸 page/list → 影子（W13 待闭合）",
        ),
        (
            Method::POST,
            t_("/script/list/manager"),
            Expect::Matched,
            "W7 脚本设计器列表（前端弃裸 script/list 影子）",
        ),
        (
            Method::POST,
            t_("/script"),
            Expect::Matched,
            "W7 脚本设计器新建（u2_script::create + v1 版本快照）",
        ),
        (
            Method::PUT,
            t_("/script/abc"),
            Expect::Matched,
            "W7 脚本设计器保存（u2_script::update + 版本快照）",
        ),
        (
            Method::DELETE,
            t_("/script/abc"),
            Expect::Matched,
            "W7 脚本设计器删除（u2_script::delete 软删）",
        ),
        (
            Method::PUT,
            t_("/page/abc"),
            Expect::Matched,
            "PUT page/{id} → 真匹配",
        ),
        (
            Method::POST,
            t_("/page"),
            Expect::Matched,
            "裸 POST page → create_page 存在",
        ),
        (
            Method::DELETE,
            t_("/page/abc"),
            Expect::Matched,
            "DELETE page/{id} → 真匹配",
        ),
        // ── Process script designer（W7：PP_E_SCRIPT u2 写路径）──
        (
            Method::POST,
            p("/script"),
            Expect::Matched,
            "W7 流程脚本新建（u2_script::create + v1 版本快照）",
        ),
        (
            Method::PUT,
            p("/script/abc"),
            Expect::Matched,
            "W7 流程脚本保存（u2_script::update + 版本快照）",
        ),
        (
            Method::DELETE,
            p("/script/abc"),
            Expect::Matched,
            "W7 流程脚本删除（u2_script::delete 软删）",
        ),
        // ── Form designer (FormDesigner.vue, base /jaxrs/form, served by cms_assemble_control) ──
        (
            Method::GET,
            f("/list/all"),
            Expect::Matched,
            "FormDesigner 列表（W6：前端弃 form/list 影子）",
        ),
        (
            Method::GET,
            f("/list/app/app-1"),
            Expect::Matched,
            "按应用列表 list/app/{appId}",
        ),
        (
            Method::GET,
            f("/abc"),
            Expect::Matched,
            "control：GET form/{id}",
        ),
        (
            Method::PUT,
            f("/abc"),
            Expect::Matched,
            "PUT form/{id}（update，真匹配）",
        ),
        (
            Method::POST,
            f(""),
            Expect::Matched,
            "POST form 裸（create，真匹配）",
        ),
        (
            Method::POST,
            f("/submit"),
            Expect::Matched,
            "预览提交 form_submit（W6 ③ 补缺：必填校验不落库）",
        ),
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
            fails.push(format!(
                "{} {}{} 期望 {:?} 实测 {}({:?})",
                intent,
                method_tag(method),
                path,
                expect,
                code,
                got
            ));
        }
    }

    println!("\n===== §9.4 设计器路由对账 · 实测复现（HEAD ab25b16b） =====");
    for l in &lines {
        println!("{l}");
    }
    println!("======================================================\n");

    assert!(
        fails.is_empty(),
        "对账分类与实测不符（§9.4 需更新）：\n{}",
        fails.join("\n")
    );
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
