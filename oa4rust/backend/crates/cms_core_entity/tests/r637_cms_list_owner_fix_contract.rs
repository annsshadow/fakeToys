// R637: CMS 文章列表/笔记列表归属门契约（implementation 入库，验证用）
// 真父 RED -> 修复 GREEN；通过 router oneshot 发请求，签名变化前后均可编译。
use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    Router,
};
use cms_core_entity::router;
use deadpool_postgres::Pool;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use shared::{
    migrate::run_migrations,
    testing::{test_pool, test_sea_orm_pool, test_session},
};
use tower::ServiceExt;

async fn fresh() -> (Pool, DatabaseConnection) {
    let pool = test_pool();
    run_migrations(&pool).await.expect("run_migrations");
    let db = test_sea_orm_pool().await.expect("test_sea_orm_pool");
    (pool, db)
}

async fn seed(pool: &Pool) {
    let c = pool.get().await.expect("seed conn");
    c.batch_execute(
        "DELETE FROM auth_person_role WHERE person_id LIKE 'r637-%';
         DELETE FROM auth_person WHERE id LIKE 'r637-%';
         DELETE FROM auth_role WHERE id = 'r637-admin';
         DELETE FROM x_cms_article WHERE id LIKE 'r637-%';
         DELETE FROM x_cms_note WHERE id LIKE 'r637-%';
         INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES
           ('r637-alice','r637-alice','alice','x'),
           ('r637-bob','r637-bob','bob','x'),
           ('r637-admin','r637-admin','admin','x');
         INSERT INTO auth_role (id, name) VALUES ('r637-admin','admin');
         INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('r637-admin','r637-admin','r637-unit');
         INSERT INTO x_cms_article (id, category_id, title, content, author_id, status) VALUES
           ('r637-alice-draft','r637-cat','alice draft','a','r637-alice','draft'),
           ('r637-bob-draft','r637-cat','bob draft','b','r637-bob','draft'),
           ('r637-public','r637-cat','public','c','r637-bob','published');
         INSERT INTO x_cms_note (id, title, content, person_id, status) VALUES
           ('r637-note-alice','alice note','a','r637-alice','active'),
           ('r637-note-bob','bob note','b','r637-bob','active');",
    )
    .await
    .expect("seed batch");
}

async fn call(app: &Router, path: &str, who: Option<&str>) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .expect("request");
    let resp = match who {
        Some(p) => app
            .clone()
            .layer(Extension(test_session(p)))
            .oneshot(req)
            .await
            .expect("oneshot"),
        None => app.clone().oneshot(req).await.expect("oneshot"),
    };
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("body");
    let text = String::from_utf8_lossy(&bytes).to_string();
    let json = serde_json::from_str(&text).unwrap_or(Value::Null);
    (status, json)
}

fn ids(json: &Value) -> Vec<String> {
    json.get("data")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.get("id").and_then(|s| s.as_str()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[tokio::test]
async fn r637_cms_list_owner_gate() {
    let (pool, db) = fresh().await;
    seed(&pool).await;
    let app = router(db, pool);

    // 文章列表：真父裸查全表 -> alice/匿名可见 bob 私有草稿；修复后仅本人+公开+admin。
    let (st, body) = call(&app, "/api/cms/article/list", Some("r637-alice")).await;
    assert_eq!(st, StatusCode::OK, "alice 文章列表 200");
    let got = ids(&body);
    assert!(
        !got.iter().any(|s| s == "r637-bob-draft"),
        "alice 不得见 bob 私有草稿, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-alice-draft"),
        "alice 应见本人私有草稿, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-public"),
        "alice 应见公开文章, got={got:?}"
    );

    // 匿名：published 可读，但 bob 私有草稿必须不可见。
    let (st, body) = call(&app, "/api/cms/article/list", None).await;
    assert_eq!(st, StatusCode::OK, "匿名文章列表 200");
    let got = ids(&body);
    assert!(
        !got.iter().any(|s| s == "r637-bob-draft"),
        "匿名不得见 bob 私有草稿, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-public"),
        "匿名应见公开文章, got={got:?}"
    );

    // admin 反臂：全量可见（行为不变）。
    let (st, body) = call(&app, "/api/cms/article/list", Some("r637-admin")).await;
    assert_eq!(st, StatusCode::OK, "admin 文章列表 200");
    let got = ids(&body);
    assert!(
        got.iter().any(|s| s == "r637-bob-draft"),
        "admin 应见 bob 草稿, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-alice-draft"),
        "admin 应见 alice 草稿, got={got:?}"
    );

    // 笔记列表：真父匿名 200 裸读全表；修复后匿名 401。
    let (st, _) = call(&app, "/api/cms/core/entity/note/list", None).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED, "笔记列表匿名必须 401");

    // alice 只见本人笔记。
    let (st, body) = call(&app, "/api/cms/core/entity/note/list", Some("r637-alice")).await;
    assert_eq!(st, StatusCode::OK, "alice 笔记列表 200");
    let got = ids(&body);
    assert!(
        !got.iter().any(|s| s == "r637-note-bob"),
        "alice 不得见 bob 笔记, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-note-alice"),
        "alice 应见本人笔记, got={got:?}"
    );

    // admin 反臂：全量笔记可见。
    let (st, body) = call(&app, "/api/cms/core/entity/note/list", Some("r637-admin")).await;
    assert_eq!(st, StatusCode::OK, "admin 笔记列表 200");
    let got = ids(&body);
    assert!(
        got.iter().any(|s| s == "r637-note-bob"),
        "admin 应见 bob 笔记, got={got:?}"
    );
    assert!(
        got.iter().any(|s| s == "r637-note-alice"),
        "admin 应见 alice 笔记, got={got:?}"
    );
}

#[test]
fn r637_cms_list_source_guard() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read src/lib.rs");
    let a = src
        .find("pub async fn article_list(")
        .expect("article_list not found");
    let a_end = src[a..]
        .find("pub async fn article_get(")
        .map(|i| a + i)
        .unwrap_or(src.len());
    let article = &src[a..a_end];
    assert!(
        article.contains("session: Option<Extension<shared::session::Session>>"),
        "article_list 必须提取会话"
    );
    assert!(
        article.contains("Condition") && article.contains("AuthorId"),
        "article_list 必须用可见范围条件谓词"
    );
    let n = src
        .find("pub async fn cms_entity_note_list(")
        .expect("cms_entity_note_list not found");
    let n_end = src[n..]
        .find("cms/core/entity/* 的 create/save/delete")
        .map(|i| n + i)
        .unwrap_or(src.len());
    let note = &src[n..n_end];
    assert!(
        note.contains("session: Option<Extension<shared::session::Session>>"),
        "cms_entity_note_list 必须提取会话"
    );
    assert!(
        note.contains("Unauthorized"),
        "cms_entity_note_list 匿名必须 401"
    );
    let t = src
        .find("async fn list_cms_entity_table(")
        .expect("list_cms_entity_table not found");
    let t_end = src[t..]
        .find("#[axum::debug_handler]")
        .map(|i| t + i)
        .unwrap_or(src.len());
    let table = &src[t..t_end];
    assert!(
        table.contains("person_id=$1") || table.contains("person_id = $1"),
        "note 列表 SQL 必须有 person_id 归属谓词"
    );
}
