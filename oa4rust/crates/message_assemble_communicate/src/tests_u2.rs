//! plan002 U2 新增端点的路由可达测试与单元测试。
//!
//! 路由测试使用 mock_pool（无法建连）：请求命中路由后 handler 返回 500
//! （带 Session 提取器的 handler 因未注入会话同样 500），断言 500/非 404
//! 即可证明路由已注册且动词正确；404=路径缺失，405=动词缺失。
//! Router 构建本身会校验路径唯一性——重复注册将直接 panic。
//!
//! IDOR 门禁（require_admin fail-closed）通过对 handler 的直接调用验证：
//! mock_pool 下 is_admin 必然返回 false，任何写操作必须先得到 Forbidden。

#[cfg(test)]
mod u2_tests {
    use crate::{
        instant_currentperson_consumed_put, mass_create, mass_id_mockdeletetoget,
        mass_target_list, router as message_router, ws_count_person,
    };
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Json;
    use serde_json::json;
    use shared::error::AppError;
    use shared::session::Session;
    use shared::testing::mock_pool;
    use tower::util::ServiceExt;

    async fn status_of(method: &str, uri: &str) -> StatusCode {
        let app = message_router(mock_pool());
        app.oneshot(
            Request::builder()
                .uri(uri)
                .method(method)
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    async fn status_of_json(method: &str, uri: &str, body: String) -> StatusCode {
        let app = message_router(mock_pool());
        app.oneshot(
            Request::builder()
                .uri(uri)
                .method(method)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    fn test_session() -> Session {
        Session {
            token: "u2-test-token".to_string(),
            person_unique: "person-u2".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(2),
        }
    }

    // ── 路由可达性：完全缺失的家族 ─────────────────────────────

    #[tokio::test]
    async fn u2_connector_route_reachable() {
        let status = status_of_json(
            "POST",
            "/jaxrs/message/assemble/communicate/connector",
            json!({"type": "taskCreate", "person": "u1", "title": "t", "body": {"k": 1}}).to_string(),
        )
        .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_ws_family_reachable() {
        assert_eq!(
            status_of_json("POST", "/jaxrs/message/assemble/communicate/ws", r#"{"person":"u1"}"#.to_string()).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(status_of("GET", "/jaxrs/message/assemble/communicate/ws/count/person").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/message/assemble/communicate/ws/list/person/current/node").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("GET", "/jaxrs/message/assemble/communicate/ws/list/person").await, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn u2_mass_family_routes_registered() {
        // POST /mass 与 DELETE /mass/{id}、GET enable/type、GET mockdeletetoget
        // 均带 Session 提取器：router 未注入会话 → 500（而非 404/405）
        assert_eq!(status_of("POST", "/jaxrs/message/assemble/communicate/mass").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status_of("DELETE", "/jaxrs/message/assemble/communicate/mass/m-1").await, StatusCode::INTERNAL_SERVER_ERROR);
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/mass/m-1/mockdeletetoget").await, StatusCode::NOT_FOUND);
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/mass/m-1/mockdeletetoget").await, StatusCode::METHOD_NOT_ALLOWED);
    }

    // ── 路由可达性：Java 动词链式补齐（405 = 动词仍缺失）───────

    #[tokio::test]
    async fn u2_java_verb_chains_accept_new_verbs() {
        // Java GET /consume/{id}/type/{type}
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/consume/c-1/type/ticket").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java PUT /consume/type/{type}
        assert_ne!(status_of("PUT", "/jaxrs/message/assemble/communicate/consume/type/ticket").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java POST /im/conversation/list/with/person
        assert_ne!(status_of("POST", "/jaxrs/message/assemble/communicate/im/conversation/list/with/person").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java GET /im/conversation/{id}/group/quit/self
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/im/conversation/c-1/group/quit/self").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java GET /im/msg/revoke/{id}
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/im/msg/revoke/m-1").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java POST /im/msg/list/{page}/size/{size} 与 /im/msg/list/object
        assert_ne!(status_of("POST", "/jaxrs/message/assemble/communicate/im/msg/list/1/size/20").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("POST", "/jaxrs/message/assemble/communicate/im/msg/list/object").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java PUT /instant/currentperson/consumed
        assert_ne!(status_of("PUT", "/jaxrs/message/assemble/communicate/instant/currentperson/consumed").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java POST /message/list/paging/{page}/size/{size}
        assert_ne!(status_of("POST", "/jaxrs/message/assemble/communicate/message/list/paging/1/size/20").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java GET /mass/enable/type
        assert_ne!(status_of("GET", "/jaxrs/message/assemble/communicate/mass/enable/type").await, StatusCode::METHOD_NOT_ALLOWED);
        // Java PUT 主动词：im read / top set
        assert_ne!(status_of("PUT", "/jaxrs/message/assemble/communicate/im/conversation/c-1/read").await, StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(status_of("PUT", "/jaxrs/message/assemble/communicate/im/conversation/c-1/top/set").await, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn u2_im_single_virtual_delete_routes_registered() {
        // Java DELETE /im/conversation/{id}/single + GET single/mockdeletetoget
        assert_ne!(status_of("DELETE", "/jaxrs/message/assemble/communicate/im/conversation/c-1/single").await, StatusCode::NOT_FOUND);
        let get_status = status_of("GET", "/jaxrs/message/assemble/communicate/im/conversation/c-1/single/mockdeletetoget").await;
        assert_ne!(get_status, StatusCode::NOT_FOUND);
        assert_ne!(get_status, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn u2_upload_path_normalized_reachable() {
        // 归一化前路径段是字面量 {\"conversationId\"}；现在为正常参数段，
        // 真实 URL /upload/<id>/type/<type> 必须命中 handler
        let status = status_of_json(
            "POST",
            "/jaxrs/message/assemble/communicate/im/msg/upload/conv-1/type/image",
            json!({"fileUrl": "http://x/f.png", "fileName": "f.png"}).to_string(),
        )
        .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── IDOR 门禁：fail-closed 直接调用验证 ────────────────────

    #[tokio::test]
    async fn u2_mass_create_requires_admin_before_any_db_write() {
        // is_admin 对不可用 DB fail-closed 返回 false → Forbidden（先于参数校验与 INSERT）
        let r = mass_create(
            axum::extract::Extension(mock_pool()),
            axum::extract::Extension(test_session()),
            Json(json!({"personList": ["u1"], "body": "hello"})),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_mass_delete_requires_admin_fail_closed() {
        let r = mass_id_mockdeletetoget(
            axum::extract::Extension(mock_pool()),
            axum::extract::Extension(test_session()),
            axum::extract::Path("m-1".to_string()),
        )
        .await;
        match r {
            Err(AppError::Forbidden) => {}
            other => panic!("expected Forbidden, got {:?}", other.map(|_| "ok")),
        }
    }

    #[tokio::test]
    async fn u2_instant_consumed_put_hits_db_with_session_person() {
        // PUT 语义是把当前人员的 instant 标记已消费：无 DB 时必须失败，
        // 而不是返回假成功
        let r = instant_currentperson_consumed_put(
            axum::extract::Extension(mock_pool()),
            axum::extract::Extension(test_session()),
        )
        .await;
        assert!(matches!(r, Err(AppError::Internal)));
    }

    #[tokio::test]
    async fn u2_ws_count_person_fails_internal_without_db() {
        let r = ws_count_person(axum::extract::Extension(mock_pool())).await;
        assert!(matches!(r, Err(AppError::Internal)));
    }

    // ── 业务规则纯单元测试：群发目标人群 ───────────────────────

    #[tokio::test]
    async fn u2_mass_target_list_merges_and_dedups() {
        // 业务规则：personList/identityList/groupList/unitList 合并去重，
        // 空串不算有效目标（对应 Java ExceptionEmptyTarget）
        let targets = mass_target_list(&json!({
            "personList": ["u1", "u2"],
            "identityList": ["i1"],
            "groupList": ["g1", ""],
            "unitList": ["u1"]
        }));
        assert_eq!(targets.len(), 4);
        assert!(targets.contains(&"u1".to_string()));
        assert!(targets.contains(&"u2".to_string()));
        assert!(targets.contains(&"i1".to_string()));
        assert!(targets.contains(&"g1".to_string()));
        assert_eq!(targets.iter().filter(|t| *t == "u1").count(), 1);
    }

    #[tokio::test]
    async fn u2_mass_target_list_empty_when_no_targets() {
        // 无任何目标人群时返回空列表 —— mass_create 据此拒绝群发
        assert!(mass_target_list(&json!({})).is_empty());
        assert!(mass_target_list(&json!({"personList": [], "unitList": [""]})).is_empty());
    }

    #[tokio::test]
    async fn u2_ws_create_reports_false_without_open_session_semantics() {
        // ws_create 在无在线连接时如实返回 value=false（Java 同义）；
        // 无 DB 时连接查询失败 → Internal，绝不假成功
        let pool = mock_pool();
        let client_err = crate::ws_create(
            axum::extract::Extension(pool),
            Json(json!({"person": "u1", "body": "hi"})),
        )
        .await;
        assert!(matches!(client_err, Err(AppError::Internal)));
    }
}
