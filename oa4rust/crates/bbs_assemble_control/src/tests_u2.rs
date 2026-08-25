//! plan002 U2 — BBS 端点闭合测试。
//!
//! 覆盖三层意图：
//! 1. 全集注册：Java 106 条端点矩阵逐条打到路由器，断言无一 404（防漏挂）；
//! 2. 归一化红线：通配权限路由与 {page}/{page} 畸形路由必须已消失；
//! 3. 行为契约（live-gated）：IDOR 门禁（owner 放行 / 非所有者 403）、
//!    真实 SQL 往返（toggle/配置/角色绑定/投票），DB 不可达时显式跳过并输出。

use super::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde_json::json;
use tower::util::ServiceExt;

fn app() -> axum::Router {
    crate::router(shared::testing::mock_pool())
}

async fn status(method: Method, uri: &str) -> StatusCode {
    app()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
}

async fn status_json(method: Method, uri: &str, body: serde_json::Value) -> StatusCode {
    app()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
}

// ────────────────────────────────────────────────────────────────
// 1. 归一化 helper 单元测试
// ────────────────────────────────────────────────────────────────

#[test]
fn test_normalize_java_path_joins_class_and_method() {
    assert_eq!(u2::normalize_java_path("forum", "{id}"), "forum/{id}");
    assert_eq!(u2::normalize_java_path("user/forum", "all"), "user/forum/all");
}

#[test]
fn test_normalize_java_path_trims_trailing_slash() {
    // Java @Path("login") 类级 + 方法级无子路径，不得产出尾斜杠。
    assert_eq!(u2::normalize_java_path("login", ""), "login");
    assert_eq!(u2::normalize_java_path("logout", ""), "logout");
    assert_eq!(u2::normalize_java_path("permission", ""), "permission");
}

#[test]
fn test_normalize_java_path_collapses_adjacent_duplicate_segments() {
    // 历史事故形态：{page}/{page}/{count}/{count} 必须被折叠成单参数段。
    let got = u2::normalize_java_path("reply/filter/list", "{page}/{page}/{count}/{count}");
    assert_eq!(got, "reply/filter/list/{page}/{count}");
}

// ────────────────────────────────────────────────────────────────
// 2. flag 白名单 / base64 / LIKE 转义
// ────────────────────────────────────────────────────────────────

#[test]
fn test_topic_flag_column_whitelist_blocks_injection() {
    for flag in [
        "is_cream",
        "is_original",
        "is_recommend",
        "top_to_bbs",
        "top_to_forum",
        "top_to_main_section",
        "top_to_section",
        "locked",
        "completed",
    ] {
        assert_eq!(u2::topic_flag_column(flag), Some(flag), "flag {} 应在白名单", flag);
    }
    assert_eq!(u2::topic_flag_column("is_cream; DROP TABLE x_bbs_topic"), None);
    assert_eq!(u2::topic_flag_column("creator"), None);
    assert_eq!(u2::topic_flag_column(""), None);
}

#[test]
fn test_base64_encode_known_vectors() {
    assert_eq!(u2::base64_encode(b""), "");
    assert_eq!(u2::base64_encode(b"f"), "Zg==");
    assert_eq!(u2::base64_encode(b"fo"), "Zm8=");
    assert_eq!(u2::base64_encode(b"foo"), "Zm9v");
    assert_eq!(u2::base64_encode(b"foobar"), "Zm9vYmFy");
}

#[test]
fn test_like_escape_neutralizes_wildcards() {
    assert_eq!(u2::like_escape("100%"), "100\\%");
    assert_eq!(u2::like_escape("a_b"), "a\\_b");
    assert_eq!(u2::like_escape("back\\slash"), "back\\\\slash");
    assert_eq!(u2::like_escape("plain"), "plain");
}

// ────────────────────────────────────────────────────────────────
// 3. 全集注册矩阵（106 条）——防漏挂的权威断言
// ────────────────────────────────────────────────────────────────

const BASE: &str = "/jaxrs/bbs/assemble/control";

/// Java x_bbs_assemble_control jaxrs 全集（类级+方法级拼接、归一化后 106 条）。
const JAVA_ENDPOINTS: &[(&str, &str)] = &[
    ("DELETE", "attachment/{id}"),
    ("GET", "attachment/{id}"),
    ("GET", "attachment/download/{id}"),
    ("GET", "attachment/download/{id}/stream/{stream}"),
    ("GET", "attachment/list/subject/{subjectId}"),
    ("POST", "attachment/upload/subject/{subjectId}"),
    ("POST", "attachment/upload/subject/{subjectId}/callback/{callback}"),
    ("GET", "forum/{id}"),
    ("GET", "forum/view/all"),
    ("POST", "login"),
    ("POST", "logout"),
    ("GET", "mobile/view/all"),
    ("GET", "permission"),
    ("GET", "permission/replyPublishable/{subjectId}"),
    ("GET", "permission/section/{sectionId}"),
    ("GET", "permission/subject/{subjectId}"),
    ("GET", "permission/subjectPublishable/{sectionId}"),
    ("POST", "picture/encode/base64/size/{size}"),
    ("POST", "picture/section/{id}/icon"),
    ("GET", "reply/{id}"),
    ("GET", "reply/list/sub/{id}"),
    ("PUT", "reply/filter/list/page/{page}/count/{count}"),
    ("GET", "section/{id}"),
    ("GET", "section/syn"),
    ("GET", "section/viewforum/{forumId}"),
    ("GET", "section/viewsub/{sectionId}"),
    ("GET", "setting/bbsName"),
    ("DELETE", "shutup/{id}"),
    ("GET", "shutup/get/shutup"),
    ("POST", "shutup/list/paging/{page}/size/{size}"),
    ("POST", "shutup/save"),
    ("GET", "subject/recommended/index/{count}"),
    ("GET", "subject/statgrade/sectionName/{sectionName}/subjectType/{subjectType}"),
    ("GET", "subject/top/{sectionId}"),
    ("GET", "subject/view/{id}"),
    ("POST", "subject/filter/listsubjectinfo/page/{page}/count/{count}"),
    ("PUT", "subject/creamed/list/page/{page}/count/{count}"),
    ("PUT", "subject/filter/list/page/{page}/count/{count}"),
    ("PUT", "subject/index/list/page/{page}/count/{count}"),
    ("PUT", "subject/recommended/list/page/{page}/count/{count}"),
    ("PUT", "subject/search/list/page/{page}/count/{count}"),
    ("DELETE", "subjectattach/{id}"),
    ("GET", "subjectattach/{id}"),
    ("GET", "subjectattach/{id}/binary/base64/{size}"),
    ("GET", "subjectattach/list/subject/{id}"),
    ("DELETE", "user/forum/{id}"),
    ("GET", "user/forum/all"),
    ("POST", "user/forum"),
    ("GET", "user/permission/forum/{forumId}"),
    ("GET", "user/permission/role/{roleCode}"),
    ("GET", "user/permission/section/{sectionId}"),
    ("DELETE", "user/reply/{id}"),
    ("POST", "user/reply"),
    ("PUT", "user/reply/accept"),
    ("PUT", "user/reply/my/list/page/{page}/count/{count}"),
    ("DELETE", "user/role/{id}"),
    ("GET", "user/role/{id}"),
    ("GET", "user/role/all"),
    ("POST", "user/role"),
    ("PUT", "user/role/bind/object"),
    ("PUT", "user/role/bind/role"),
    ("PUT", "user/role/forum/{forumId}"),
    ("PUT", "user/role/rolecode/selected"),
    ("PUT", "user/role/section/{sectionId}"),
    ("PUT", "user/role/unit/selected"),
    ("PUT", "user/role/user/selected"),
    ("DELETE", "user/section/{id}"),
    ("DELETE", "user/section/force/{id}"),
    ("GET", "user/section/all"),
    ("GET", "user/section/forum/{forumId}"),
    ("GET", "user/section/sub/{sectionId}"),
    ("POST", "user/section"),
    ("GET", "user/setting/{id}"),
    ("GET", "user/setting/all"),
    ("PUT", "user/setting"),
    ("PUT", "user/setting/code"),
    ("DELETE", "user/subject/{id}"),
    ("GET", "user/subject/{id}"),
    ("GET", "user/subject/acceptreply/{id}/{replyId}"),
    ("GET", "user/subject/complete/{id}"),
    ("GET", "user/subject/lock/{id}"),
    ("GET", "user/subject/nonCream/{id}"),
    ("GET", "user/subject/nonOriginal/{id}"),
    ("GET", "user/subject/nonRecommendToBBSIndex/{id}"),
    ("GET", "user/subject/nonTopToBBS/{id}"),
    ("GET", "user/subject/nonTopToForum/{id}"),
    ("GET", "user/subject/nonTopToMainSection/{id}"),
    ("GET", "user/subject/nonTopToSection/{id}"),
    ("GET", "user/subject/setCream/{id}"),
    ("GET", "user/subject/setOriginal/{id}"),
    ("GET", "user/subject/setRecommendToBBSIndex/{id}"),
    ("GET", "user/subject/topToBBS/{id}"),
    ("GET", "user/subject/topToForum/{id}"),
    ("GET", "user/subject/topToMainSection/{id}"),
    ("GET", "user/subject/topToSection/{id}"),
    ("GET", "user/subject/unacceptreply/{id}"),
    ("GET", "user/subject/uncomplete/{id}"),
    ("GET", "user/subject/unlock/{id}"),
    ("POST", "user/subject"),
    ("PUT", "user/subject/change/section"),
    ("PUT", "user/subject/my/list/page/{page}/count/{count}"),
    ("PUT", "user/subject/vote/submit"),
    ("PUT", "user/subject/voterecord/list/page/{page}/count/{count}"),
    ("GET", "userinfo/update/nick/name/{person}"),
    ("PUT", "userinfo"),
    ("GET", "uuid/random"),
];

#[test]
fn test_java_endpoint_matrix_has_no_duplicates() {
    let mut seen = std::collections::HashSet::new();
    for (method, sub) in JAVA_ENDPOINTS {
        assert!(
            seen.insert(format!("{} {}", method, sub)),
            "矩阵内重复端点：{} {}",
            method,
            sub
        );
    }
    assert_eq!(JAVA_ENDPOINTS.len(), 106, "应与 Java HTTP 注解总数一致");
}

/// 权威对齐断言：106 条逐一请求，任何一条 404 即视为未闭合。
#[tokio::test]
async fn test_all_java_endpoints_registered() {
    let mut misses: Vec<String> = Vec::new();
    for (method, sub) in JAVA_ENDPOINTS {
        let path = format!("{}/{}", BASE, u2::normalize_java_path("", sub));
        let st = status(Method::from_bytes(method.as_bytes()).unwrap(), &path).await;
        if st == StatusCode::NOT_FOUND {
            misses.push(format!("{} {}", method, sub));
        }
    }
    assert!(misses.is_empty(), "以下 Java 端点未注册(404): {:?}", misses);
}

// ────────────────────────────────────────────────────────────────
// 4. 通配/畸形路由清除红线
// ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_wildcard_permission_route_removed() {
    // 归一化查重：{replyPublishable} 通配会吞并静态兄弟路由，必须 404。
    let st = status(Method::GET, &format!("{}/permission/foo/bar", BASE)).await;
    assert_eq!(st, StatusCode::NOT_FOUND, "通配权限路由应已移除");
}

#[tokio::test]
async fn test_malformed_dup_param_reply_filter_route_removed() {
    // 旧畸形路由 {page}/{page}/{count}/{count} 已被精确路径取代。
    let old_st = status(
        Method::GET,
        &format!("{}/reply/filter/list/1/1/10/10", BASE),
    )
    .await;
    assert_eq!(old_st, StatusCode::NOT_FOUND, "畸形重复参数路由应已移除");

    let new_st = status(
        Method::PUT,
        &format!("{}/reply/filter/list/page/2/count/5", BASE),
    )
    .await;
    assert_ne!(new_st, StatusCode::NOT_FOUND, "Java 精确路径应已注册");
}

// ────────────────────────────────────────────────────────────────
// 5. 显式 501 契约
// ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_unlandable_endpoints_return_explicit_501() {
    const UNLANDABLE: &[(&str, &str)] = &[
        ("GET", "attachment/download/att-1"),
        ("GET", "attachment/download/att-1/stream/0"),
        ("POST", "attachment/upload/subject/sub-1"),
        ("POST", "attachment/upload/subject/sub-1/callback/cb"),
        ("POST", "picture/encode/base64/size/100"),
        ("POST", "picture/section/sec-1/icon"),
        ("GET", "section/syn"),
    ];
    for (method, sub) in UNLANDABLE {
        let st = status(Method::from_bytes(method.as_bytes()).unwrap(), &format!("{}/{}", BASE, sub)).await;
        assert_eq!(
            st,
            StatusCode::NOT_IMPLEMENTED,
            "{} {} 应显式 501",
            method,
            sub
        );
    }
}

// ────────────────────────────────────────────────────────────────
// 6. 无 DB 依赖的行为契约
// ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_login_empty_credentials_contract() {
    let st = status_json(
        Method::POST,
        &format!("{}/login", BASE),
        json!({"credential": "", "password": ""}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "空凭据走 ActionResult.error 而非 HTTP 错误码");
}

#[tokio::test]
async fn test_uuid_random_returns_valid_uuid() {
    let response = app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("{}/uuid/random", BASE))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["type"], "success");
    let parsed = uuid::Uuid::parse_str(v["data"]["uuid"].as_str().unwrap());
    assert!(parsed.is_ok(), "uuid/random 应返回可解析 UUID");
}

#[tokio::test]
async fn test_legacy_extended_routes_survive() {
    // 扩展端点（非 Java 全集成员）不得因本轮改造丢失。
    for (method, uri) in [
        ("GET", format!("{}/config", BASE)),
        ("GET", format!("{}/forum/list", BASE)),
        ("GET", format!("{}/uuid", BASE)),
        ("DELETE", format!("{}/delete/forum", BASE)),
        ("POST", format!("{}/shutup/create", BASE)),
    ] {
        let st = status(Method::from_bytes(method.as_bytes()).unwrap(), &uri).await;
        assert_ne!(st, StatusCode::NOT_FOUND, "扩展路由丢失：{} {}", method, uri);
    }
}

// ────────────────────────────────────────────────────────────────
// 7. live-gated 行为契约（DB 可达时验证真实 SQL 与 IDOR 门禁）
// ────────────────────────────────────────────────────────────────

fn make_session(person: &str, tag: &str) -> shared::session::Session {
    let now = chrono::Utc::now().naive_utc();
    shared::session::Session {
        token: format!("u2-test-token-{}", tag),
        person_unique: person.to_string(),
        created_at: now,
        expires_at: now + chrono::Duration::hours(2),
    }
}

async fn send_with_session(
    app: axum::Router,
    method: Method,
    uri: &str,
    body: Option<serde_json::Value>,
    session: Option<shared::session::Session>,
) -> (StatusCode, serde_json::Value) {
    let mut builder = Request::builder().method(method.clone()).uri(uri);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    if session.is_some() {
        builder = builder.extension(session.unwrap());
    }
    let body_bytes = body
        .map(|b| serde_json::to_vec(&b).unwrap())
        .unwrap_or_default();
    let req = builder.body(Body::from(body_bytes)).unwrap();

    let response = app.oneshot(req).await.unwrap();
    let st = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 65536).await.unwrap();
    let json = serde_json::from_slice(&bytes).unwrap_or(json!({"error": "no json"}));
    (st, json)
}

/// IDOR 门禁核心意图：主题 flag toggle 只允许所有者或 admin 操作——
/// 非所有者必须 403；所有者操作后数据库真实落值。
#[tokio::test]
async fn test_toggle_cream_idor_gate_and_roundtrip() {
    use shared::testing::{is_db_available, test_pool};
    if !is_db_available().await {
        eprintln!("skipping test_toggle_cream_idor_gate_and_roundtrip: DB not reachable");
        return;
    }
    u2::ensure_u2_schema(&test_pool().get().await.unwrap()).await;

    let owner = "u2-owner-alice";
    let topic_id = "u2-topic-cream-001";
    {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_bbs_topic (id, title, author_id, creator, section_id) \
                 VALUES ($1,'t',$2,$3,'sec-u2') \
                 ON CONFLICT (id) DO UPDATE SET creator = $3, deleted_at = NULL",
                &[&topic_id, &owner, &owner],
            )
            .await
            .expect("seed topic");
    }

    // 非所有者 → 403
    let app = crate::router(test_pool());
    let (st_stranger, _) = send_with_session(
        app.clone(),
        Method::GET,
        &format!("{}/user/subject/setCream/{}", BASE, topic_id),
        None,
        Some(make_session("u2-stranger-bob", "bob")),
    )
    .await;
    assert_eq!(st_stranger, StatusCode::FORBIDDEN, "非所有者必须被 IDOR 门禁拒绝");

    // 所有者 → 落库为 true
    let (st_owner, body) = send_with_session(
        app,
        Method::GET,
        &format!("{}/user/subject/setCream/{}", BASE, topic_id),
        None,
        Some(make_session(owner, "alice")),
    )
    .await;
    assert_eq!(st_owner, StatusCode::OK);
    assert_eq!(body["type"], "success");

    let pool = test_pool();
    let client = pool.get().await.unwrap();
    let row = client
        .query_one("SELECT is_cream FROM x_bbs_topic WHERE id = $1", &[&topic_id])
        .await
        .unwrap();
    assert!(row.get::<_, bool>("is_cream"), "toggle 后 is_cream 应真实落库");

    // 清理
    let _ = client.execute("DELETE FROM x_bbs_topic WHERE id = $1", &[&topic_id]).await;
}

/// BBS_NAME 种子配置必须能被 setting/bbsName 读出（真实 SELECT 往返）。
#[tokio::test]
async fn test_setting_bbs_name_seeded_and_upsert_by_code() {
    use shared::testing::{is_db_available, test_pool};
    if !is_db_available().await {
        eprintln!("skipping test_setting_bbs_name_seeded_and_upsert_by_code: DB not reachable");
        return;
    }
    u2::ensure_u2_schema(&test_pool().get().await.unwrap()).await;

    let admin = make_session("u2-admin-carol", "carol");
    // 临时授予 admin 角色，验证管理写通道真实可用。
    {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO auth_role (id, name) VALUES ('u2-test-admin-role','admin') \
                 ON CONFLICT (id) DO NOTHING",
                &[],
            )
            .await
            .expect("seed admin role");
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ('u2-admin-person','u2-admin-carol','Carol','x') \
                 ON CONFLICT (id) DO NOTHING",
                &[],
            )
            .await
            .expect("seed admin person");
        client
            .execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('u2-admin-person','u2-test-admin-role','u2-unit') \
                 ON CONFLICT DO NOTHING",
                &[],
            )
            .await
            .expect("bind admin role");
    }

    let app = crate::router(test_pool());
    // GET bbsName 公开可读。
    let (st_read, body) = send_with_session(app.clone(), Method::GET, &format!("{}/setting/bbsName", BASE), None, None).await;
    assert_eq!(st_read, StatusCode::OK);
    assert!(body["data"]["bbsName"].as_str().is_some(), "bbsName 必须返回字符串");

    // 非 admin 更新配置 → 403（管理资源门禁）。
    let (st_non_admin, _) = send_with_session(
        app.clone(),
        Method::PUT,
        &format!("{}/user/setting", BASE),
        Some(json!({"code": "BBS_NAME", "value": "X"})),
        Some(make_session("nobody-u2", "nb")),
    )
    .await;
    assert_eq!(st_non_admin, StatusCode::FORBIDDEN);

    // admin upsert → 生效。
    let (st_admin, body_admin) = send_with_session(
        app,
        Method::PUT,
        &format!("{}/user/setting", BASE),
        Some(json!({"id": "bbs-setting-bbs-name", "code": "BBS_NAME", "value": "U2测试社区"})),
        Some(admin),
    )
    .await;
    assert_eq!(st_admin, StatusCode::OK);
    assert_eq!(body_admin["type"], "success");

    let (_, body_after) = send_with_session(crate::router(test_pool()), Method::GET, &format!("{}/setting/bbsName", BASE), None, None).await;
    assert_eq!(body_after["data"]["bbsName"], "U2测试社区", "upsert 后 bbsName 应变化");
}

/// 角色域往返：admin 建角色 → 列表可见 → 绑定人 → 按人查回。
#[tokio::test]
async fn test_role_save_bind_and_query_roundtrip() {
    use shared::testing::{is_db_available, test_pool};
    if !is_db_available().await {
        eprintln!("skipping test_role_save_bind_and_query_roundtrip: DB not reachable");
        return;
    }
    u2::ensure_u2_schema(&test_pool().get().await.unwrap()).await;

    // 授予 admin（与上一测试相同的种子行，幂等补齐）。
    {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client.execute("INSERT INTO auth_role (id, name) VALUES ('u2-test-admin-role','admin') ON CONFLICT (id) DO NOTHING", &[]).await.expect("seed admin role");
        client.execute("INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('u2-admin-person','u2-admin-carol','Carol','x') ON CONFLICT (id) DO NOTHING", &[]).await.expect("seed admin person");
        client.execute("INSERT INTO auth_person_role (person_id, role_id, unit_id) VALUES ('u2-admin-person','u2-test-admin-role','u2-unit') ON CONFLICT DO NOTHING", &[]).await.expect("bind admin role");
    }

    let admin = make_session("u2-admin-carol", "carol2");
    let code = format!("u2role{}", chrono::Utc::now().timestamp());
    let app = crate::router(test_pool());

    let (st_create, created) = send_with_session(
        app.clone(),
        Method::POST,
        &format!("{}/user/role", BASE),
        Some(json!({"name": "U2版主", "code": code})),
        Some(admin),
    )
    .await;
    assert_eq!(st_create, StatusCode::OK, "admin 创建角色应成功: {}", created);
    let role_id = created["data"]["id"].as_str().unwrap().to_string();

    // 绑定人到角色
    let (st_bind, bind_body) = send_with_session(
        app.clone(),
        Method::PUT,
        &format!("{}/user/role/bind/role", BASE),
        Some(json!({"personCode": "u2-member-dave", "roleIds": [role_id]})),
        Some(make_session("u2-admin-carol", "carol3")),
    )
    .await;
    assert_eq!(st_bind, StatusCode::OK);
    assert_eq!(bind_body["data"]["bound"], 1);

    // 按人查回绑定角色（响应契约：data 为数组）
    let (st_q, q_body) = send_with_session(
        app,
        Method::PUT,
        &format!("{}/user/role/user/selected", BASE),
        Some(json!({"personCode": "u2-member-dave"})),
        None,
    )
    .await;
    assert_eq!(st_q, StatusCode::OK);
    let found = q_body["data"]
        .as_array()
        .map(|a| a.iter().any(|r| r["id"] == created["data"]["id"]))
        .unwrap_or(false);
    assert!(found, "按人查询应返回刚绑定的角色，实际：{}", q_body);

    // 清理
    let pool = test_pool();
    let client = pool.get().await.unwrap();
    let _ = client.execute("DELETE FROM x_bbs_role_bind WHERE role_id = $1", &[&role_id]).await;
    let _ = client.execute("DELETE FROM x_bbs_role WHERE id = $1", &[&role_id]).await;
}

/// 投票提交持久化投票记录并累加 vote_count（真实 INSERT+UPDATE 往返）。
#[tokio::test]
async fn test_vote_submit_persists_record_and_count() {
    use shared::testing::{is_db_available, test_pool};
    if !is_db_available().await {
        eprintln!("skipping test_vote_submit_persists_record_and_count: DB not reachable");
        return;
    }
    u2::ensure_u2_schema(&test_pool().get().await.unwrap()).await;

    let voter = "u2-voter-erin";
    let topic_id = "u2-topic-vote-001";
    {
        let pool = test_pool();
        let client = pool.get().await.unwrap();
        client
            .execute(
                "INSERT INTO x_bbs_topic (id, title, author_id, creator, section_id) \
                 VALUES ($1,'vote t',$2,$3,'sec-u2') \
                 ON CONFLICT (id) DO UPDATE SET deleted_at = NULL",
                &[&topic_id, &voter, &voter],
            )
            .await
            .expect("seed vote topic");
        let _ = client.execute("UPDATE x_bbs_topic SET vote_count = 0 WHERE id = $1", &[&topic_id]).await;
        // 清理历史运行残留，保证幂等。
        let _ = client.execute("DELETE FROM x_bbs_vote_record WHERE topic_id = $1", &[&topic_id]).await;
    }

    let app = crate::router(test_pool());
    let (st, body) = send_with_session(
        app,
        Method::PUT,
        &format!("{}/user/subject/vote/submit", BASE),
        Some(json!({"subjectId": topic_id, "optionId": "opt-1", "optionName": "赞成"})),
        Some(make_session(voter, "erin")),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "{}", body);
    assert_eq!(body["type"], "success");

    let pool = test_pool();
    let client = pool.get().await.unwrap();
    let rec = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_vote_record WHERE topic_id = $1 AND person = $2",
            &[&topic_id, &voter],
        )
        .await
        .unwrap();
    assert_eq!(rec.get::<_, i64>(0), 1, "投票记录应真实入库");
    let cnt = client
        .query_one("SELECT vote_count FROM x_bbs_topic WHERE id = $1", &[&topic_id])
        .await
        .unwrap();
    assert_eq!(cnt.get::<_, i32>("vote_count"), 1, "vote_count 应累加到 1");

    let _ = client.execute("DELETE FROM x_bbs_vote_record WHERE topic_id = $1", &[&topic_id]).await;
    let _ = client.execute("DELETE FROM x_bbs_topic WHERE id = $1", &[&topic_id]).await;
}
