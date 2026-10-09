// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde_json::json;
use tower::util::ServiceExt;

fn build_test_pool() -> deadpool_postgres::Pool {
    deadpool_postgres::Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap()
}
#[test]
fn test_send_message_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "sent": true,
        "from": "user1",
        "to": "user2"
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["sent"], true);
}

#[test]
fn test_receive_list_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "count": 1,
        "data": [{"id": "msg-1", "status": "unread"}]
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["count"], 1);
}

#[test]
fn test_mark_read_action_result_format() {
    let result: ActionResult<serde_json::Value> = ActionResult::success(json!({
        "id": "msg-1",
        "marked_read": true
    }));
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"]["marked_read"], true);
}

#[tokio::test]
async fn test_send_message_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let req = serde_json::to_string(&json!({
        "from": "sender",
        "to": "receiver",
        "content": "hello"
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/send")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(req))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_receive_list_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/receive/consumer1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_mark_read_route_exists() {
    let pool = build_test_pool();
    let app = crate::router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/mark_read/msg-1")
                .method(Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_delete_message_assemble_communicate_im_co() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/test-id/group/mockdeletetoget")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_message_assemble_communicate_mass_() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/test-id/mockdeletetoget")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_consu() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/list/test-id/count/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_im_co() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation/business/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_im_ma() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/manager/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_im_ms() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/collection/list/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_insta() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/currentperson/consumed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_mass_() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_messa() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/list/paging/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_message_assemble_communicate_recei() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/receive/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_consu() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/consume/type/test-id/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_im_co() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/conversation")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_im_ms() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_insta() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/instant/currentperson/consumed/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_mark_() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mark_read/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_mass_() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/mass/enable/type")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_messa() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/message/custom/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_message_assemble_communicate_send() {
        let pool = build_test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/send")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }
}

#[tokio::test]
async fn test_u2_put_im_conversation_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/im/conversation")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_delete_im_conversation_group_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/im/conversation/test-id/group")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_u2_post_im_conversation_route() {
    let pool = build_test_pool();
    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/im/conversation")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

/// 发消息二写原子性（优化二轮 35）：send 现把「INSERT x_message」+「UPDATE
/// x_message_conversation.last_message_time」包进单事务+commit。验证提交后终态
/// 一致——消息行入库 且 会话时间戳被刷新（二写同时可见，证非半提交）。
#[tokio::test]
async fn test_send_message_row_and_conversation_touch_are_atomic() {
    if !shared::testing::is_db_available().await {
        eprintln!(
            "skipping test_send_message_row_and_conversation_touch_are_atomic: DB not reachable"
        );
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_message (id TEXT, conversation_id TEXT, content TEXT, sender TEXT, type TEXT, create_time TEXT)",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_message_conversation (id TEXT, name TEXT, type TEXT, create_time TEXT)",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "ALTER TABLE x_message_conversation ADD COLUMN IF NOT EXISTS last_message_time TEXT",
        &[],
    )
    .await
    .unwrap();

    let conv_id = "u2-conv-send-atom-001";
    c.execute(
        "DELETE FROM x_message WHERE conversation_id = $1",
        &[&conv_id],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM x_message_conversation WHERE id = $1",
        &[&conv_id],
    )
    .await
    .unwrap();
    c.execute(
        "INSERT INTO x_message_conversation (id, name, type) VALUES ($1, 'conv', 'single')",
        &[&conv_id],
    )
    .await
    .unwrap();

    let app = crate::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/message/assemble/communicate/send")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "conversationId": conv_id,
                        "content": "u2-hello",
                        "sender": "u2-sender-dave",
                        "type": "text"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 65536)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["data"]["sent"], true);

    // 事务提交后：消息行入库 且 会话时间戳已刷新（二写同时落库）
    let msg_rows: i64 = c
        .query_one(
            "SELECT COUNT(*) AS c FROM x_message WHERE conversation_id = $1 AND content = 'u2-hello'",
            &[&conv_id],
        )
        .await
        .unwrap()
        .get("c");
    assert_eq!(msg_rows, 1, "消息行应入库");
    let touched: bool = c
        .query_one(
            "SELECT last_message_time IS NOT NULL AS t FROM x_message_conversation WHERE id = $1",
            &[&conv_id],
        )
        .await
        .unwrap()
        .get("t");
    assert!(touched, "会话 last_message_time 应与消息同事务刷新");

    c.execute(
        "DELETE FROM x_message WHERE conversation_id = $1",
        &[&conv_id],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM x_message_conversation WHERE id = $1",
        &[&conv_id],
    )
    .await
    .unwrap();
}

/// 群发消息游标分页回归（优化二轮 48）：mass_list next/prev 原把路径锚点 id 同时绑到
/// `mass_id = $1`（campaign id 列）AND `id > $2`，mass_id=<消息id> 恒假 → 永远返回空。
/// 修复后对齐同文件 instant_list 的 `WHERE id > $1 ... LIMIT $2` 游标语义。
#[tokio::test]
async fn test_mass_list_next_returns_messages_after_anchor() {
    if !shared::testing::is_db_available().await {
        eprintln!("skipping test_mass_list_next_returns_messages_after_anchor: DB not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    let c = pool.get().await.unwrap();
    c.execute(
        "CREATE TABLE IF NOT EXISTS x_message (id TEXT, conversation_id TEXT, content TEXT, sender TEXT, type TEXT, create_time TEXT, mass_id TEXT)",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "ALTER TABLE x_message ADD COLUMN IF NOT EXISTS mass_id TEXT",
        &[],
    )
    .await
    .unwrap();
    c.execute(
        "DELETE FROM x_message WHERE id IN ('u2-mass-a','u2-mass-b')",
        &[],
    )
    .await
    .unwrap();
    // 同一 mass campaign 的两条消息，id 'a' < 'b'
    c.execute(
        "INSERT INTO x_message (id, mass_id, content, create_time) VALUES \
         ('u2-mass-a','camp-1','first','2026-10-10 00:00:00'), \
         ('u2-mass-b','camp-1','second','2026-10-10 00:00:01')",
        &[],
    )
    .await
    .unwrap();

    // 以 'u2-mass-a' 为锚点取之后的消息 → 必含 'u2-mass-b'（修复前恒空）
    let resp = crate::mass_list_id_next_count(
        axum::Extension(pool.clone()),
        axum::extract::Path(("u2-mass-a".to_string(), 10)),
    )
    .await
    .unwrap();
    let j = serde_json::to_value(&resp.0).unwrap();
    let ids: Vec<String> = j["data"]
        .as_array()
        .expect("data array")
        .iter()
        .map(|r| r["id"].as_str().unwrap_or_default().to_string())
        .collect();
    assert!(
        ids.contains(&"u2-mass-b".to_string()),
        "锚点之后的群发消息必须可达（修复前因 mass_id=<消息id> 恒空）"
    );

    c.execute(
        "DELETE FROM x_message WHERE id IN ('u2-mass-a','u2-mass-b')",
        &[],
    )
    .await
    .unwrap();
}
