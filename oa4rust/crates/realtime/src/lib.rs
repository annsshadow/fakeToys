use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RealtimeError {
    #[error("room not found: {0}")]
    RoomNotFound(String),
    #[error("client envelope is not a broadcastable event: {0}")]
    InvalidEnvelope(String),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

pub type RealtimeResult<T> = Result<T, RealtimeError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMessage {
    pub room: String,
    pub sender: String,
    pub content: String,
    /// 线上格式为 `type`：前端 SDK（websocket.ts）按 `{type, data, timestamp}` 分发事件
    #[serde(rename = "type")]
    pub msg_type: String,
    pub timestamp: i64,
    #[serde(default)]
    pub data: Value,
}

impl RealtimeMessage {
    pub fn new(
        room: impl Into<String>,
        sender: impl Into<String>,
        content: impl Into<String>,
        msg_type: impl Into<String>,
    ) -> Self {
        Self {
            room: room.into(),
            sender: sender.into(),
            content: content.into(),
            msg_type: msg_type.into(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            data: Value::Null,
        }
    }

    pub fn to_json(&self) -> RealtimeResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

/// W10：客户端可上行的 IM 事件类型 —— 与前端 SDK websocket.ts 的 EventMap 一致。
const CLIENT_EVENT_TYPES: [&str; 5] = [
    "im_create",
    "im_revoke",
    "im_conversation",
    "notification",
    "process_task",
];

/// W10：规范化客户端经 WebSocket 上行的信封。
///
/// 仅接受 SDK EventMap 中的 IM 事件（`{type, data}` JSON 对象），归一为
/// 可广播的 RealtimeMessage；ping 心跳、裸文本等非事件上行一律拒绝，
/// 绝不转发到房间（否则心跳会广播给所有成员）。
pub fn client_message(room: &str, sender: &str, text: &str) -> RealtimeResult<RealtimeMessage> {
    let envelope: Value = serde_json::from_str(text)?;
    let msg_type = envelope
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !CLIENT_EVENT_TYPES.contains(&msg_type) {
        return Err(RealtimeError::InvalidEnvelope(msg_type.to_string()));
    }
    let data = envelope.get("data").cloned().unwrap_or_default();
    if !data.is_object() {
        return Err(RealtimeError::InvalidEnvelope(msg_type.to_string()));
    }
    Ok(RealtimeMessage {
        room: room.to_string(),
        sender: sender.to_string(),
        content: data.to_string(),
        msg_type: msg_type.to_string(),
        timestamp: chrono::Utc::now().timestamp_millis(),
        data,
    })
}

/// P5：客户端上行意图（IM 完整协议）。
///
/// 在 W10 的 5 类 IM 事件之上，补齐 SDK 客户端（websocket.ts）实际使用的
/// 三类控制上行：`message`（channel 消息）、`join_room`/`leave_room`（房间切换）、
/// `presence`（在线身份声明）。心跳 `ping` 与未知类型归 `Ignore`（不再拒绝，
/// 避免误伤客户端自定义事件）。
#[derive(Debug)]
pub enum Uplink {
    /// 可广播的 IM 事件（im_* / notification / process_task / channel message）。
    Im(RealtimeMessage),
    /// 切换目标房间。
    JoinRoom(String),
    /// 离开当前房间。
    LeaveRoom,
    /// 声明在线身份（sender），用于 presence/roster。
    Presence(String),
    /// 不处理（心跳 / 未知 / 非对象）。
    Ignore,
}

/// 解析客户端上行为 Uplink。`room`/`sender` 为当前连接上下文。
pub fn parse_uplink(room: &str, sender: &str, text: &str) -> Uplink {
    let envelope: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(_) => return Uplink::Ignore,
    };
    let msg_type = envelope
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let data = envelope.get("data").cloned().unwrap_or_else(|| Value::Null);
    match msg_type {
        "im_create" | "im_revoke" | "im_conversation" | "notification" | "process_task" => {
            Uplink::Im(RealtimeMessage {
                room: room.to_string(),
                sender: sender.to_string(),
                content: data.to_string(),
                msg_type: msg_type.to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
                data,
            })
        }
        // SDK send(channel, data)：{type:'message', channel, data}
        "message" => {
            let channel = envelope
                .get("channel")
                .and_then(Value::as_str)
                .unwrap_or(room)
                .to_string();
            Uplink::Im(RealtimeMessage {
                room: channel,
                sender: sender.to_string(),
                content: data.to_string(),
                msg_type: "im_create".to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
                data,
            })
        }
        "join_room" => Uplink::JoinRoom(
            envelope
                .get("data")
                .and_then(|d| d.get("room_id"))
                .and_then(Value::as_str)
                .unwrap_or(room)
                .to_string(),
        ),
        "leave_room" => Uplink::LeaveRoom,
        "presence" => Uplink::Presence(
            envelope
                .get("data")
                .and_then(|d| d.get("sender"))
                .and_then(Value::as_str)
                .unwrap_or(sender)
                .to_string(),
        ),
        // 心跳 / 未知类型：忽略（不广播，避免污染房间）
        _ => Uplink::Ignore,
    }
}

#[allow(dead_code)]
struct RoomHandle {
    room_id: String,
    tx: broadcast::Sender<RealtimeMessage>,
    connections: Arc<Mutex<HashSet<Uuid>>>,
    /// P5：在线身份（roster），由 presence/首条消息注册。
    senders: Arc<Mutex<HashSet<String>>>,
}

#[derive(Clone)]
pub struct RealtimeManager {
    rooms: Arc<Mutex<HashMap<String, RoomHandle>>>,
}

impl Default for RealtimeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RealtimeManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn join(&self, room_id: &str, conn_id: Uuid) -> broadcast::Receiver<RealtimeMessage> {
        let mut rooms = self.rooms.lock().await;
        let room = rooms.entry(room_id.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            RoomHandle {
                room_id: room_id.to_string(),
                tx,
                connections: Arc::new(Mutex::new(HashSet::new())),
                senders: Arc::new(Mutex::new(HashSet::new())),
            }
        });
        room.connections.lock().await.insert(conn_id);
        info!(conn_id = %conn_id, room = %room_id, "ws joined");
        room.tx.subscribe()
    }

    pub async fn leave(&self, room_id: &str, conn_id: Uuid) {
        if let Some(room) = self.rooms.lock().await.get(room_id) {
            room.connections.lock().await.remove(&conn_id);
            info!(conn_id = %conn_id, room = %room_id, "ws left");
        }
    }

    pub async fn broadcast(&self, room_id: &str, msg: RealtimeMessage) {
        if let Some(room) = self.rooms.lock().await.get(room_id) {
            let _ = room.tx.send(msg);
        }
    }

    /// P5：广播 IM 消息并返回投递到的在线连接数（用于回执 deliveryCount）。
    pub async fn broadcast_im(&self, room_id: &str, msg: &RealtimeMessage) -> usize {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            let count = room.connections.lock().await.len();
            let _ = room.tx.send(msg.clone());
            count
        } else {
            0
        }
    }

    /// P5：登记房间在线身份（roster）。
    pub async fn register_sender(&self, room_id: &str, sender: &str) {
        if sender.is_empty() {
            return;
        }
        if let Some(room) = self.rooms.lock().await.get(room_id) {
            room.senders.lock().await.insert(sender.to_string());
        }
    }

    /// P5：在线状态快照（roster 排序 + 在线连接数）。
    pub async fn presence(&self, room_id: &str) -> (Vec<String>, usize) {
        let rooms = self.rooms.lock().await;
        match rooms.get(room_id) {
            Some(room) => {
                let mut roster: Vec<String> = room.senders.lock().await.iter().cloned().collect();
                roster.sort();
                let count = room.connections.lock().await.len();
                (roster, count)
            }
            None => (Vec::new(), 0),
        }
    }

    pub async fn room_connections(&self, room_id: &str) -> usize {
        let rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get(room_id) {
            room.connections.lock().await.len()
        } else {
            0
        }
    }

    pub async fn list_rooms(&self) -> Vec<String> {
        self.rooms.lock().await.keys().cloned().collect()
    }
}

async fn handle_connection(
    socket: WebSocket,
    manager: Arc<RealtimeManager>,
    conn_id: Uuid,
    room_id: String,
) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut current_room = room_id.clone();
    let mut rx = manager.join(&current_room, conn_id).await;

    // 欢迎帧 + 初始在线快照（roster/presence）。
    let welcome = RealtimeMessage::new(
        current_room.clone(),
        "system",
        format!("connected: {}", conn_id),
        "system",
    );
    if let Ok(json) = welcome.to_json() {
        if ws_sender.send(Message::Text(json.into())).await.is_err() {
            manager.leave(&current_room, conn_id).await;
            return;
        }
    }
    if let Err(e) = send_presence(&mut ws_sender, &manager, &current_room).await {
        warn!(conn_id = %conn_id, error = %e, "ws presence send failed");
        manager.leave(&current_room, conn_id).await;
        return;
    }

    let mut heartbeat = tokio::time::interval(Duration::from_secs(30));
    let mut last_pong = tokio::time::Instant::now();
    // 发送者标签：初始为连接 UUID；客户端 presence 声明身份后改用声明值，
    // 使 roster 与消息归属一致（消息 sender = 已声明身份）。
    let mut sender_label = conn_id.to_string();

    loop {
        tokio::select! {
            _ = heartbeat.tick() => {
                if last_pong.elapsed() > Duration::from_secs(90) {
                    warn!(conn_id = %conn_id, "ws heartbeat timeout");
                    break;
                }
                if ws_sender.send(Message::Ping(axum::body::Bytes::new())).await.is_err() {
                    break;
                }
            }
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        if let Ok(json) = msg.to_json() {
                            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
            ws_msg = ws_receiver.next() => {
                match ws_msg {
                    Some(Ok(Message::Text(text))) => {
                        last_pong = tokio::time::Instant::now();
                        handle_uplink(&mut ws_sender, &manager, conn_id, &mut current_room, &mut rx, &mut sender_label, &text).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        break;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_pong = tokio::time::Instant::now();
                    }
                    Some(Ok(Message::Ping(_))) => {}
                    Some(Ok(Message::Binary(_))) => {}
                    Some(Err(e)) => {
                        warn!(conn_id = %conn_id, error = %e, "ws error");
                        break;
                    }
                    None => break,
                }
            }
        }
    }

    let _ = ws_sender
        .send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::NORMAL,
            reason: "bye".into(),
        })))
        .await;
    manager.leave(&current_room, conn_id).await;
    info!(conn_id = %conn_id, room = %current_room, "ws closed");
}

/// 广播一条 presence 下行（room 在线快照：roster + 连接数）。
async fn send_presence(
    ws_sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    manager: &Arc<RealtimeManager>,
    room_id: &str,
) -> Result<(), String> {
    let (roster, count) = manager.presence(room_id).await;
    let msg = RealtimeMessage {
        room: room_id.to_string(),
        sender: "system".to_string(),
        content: format!(
            "{}",
            serde_json::json!({ "room": room_id, "online": roster, "count": count })
        ),
        msg_type: "presence".to_string(),
        timestamp: chrono::Utc::now().timestamp_millis(),
        data: serde_json::json!({ "room": room_id, "online": roster, "count": count }),
    };
    let json = msg.to_json().map_err(|e| e.to_string())?;
    ws_sender
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 处理一条客户端上行文本（Uplink 意图分发）。
#[allow(clippy::too_many_arguments)]
async fn handle_uplink(
    ws_sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    manager: &Arc<RealtimeManager>,
    conn_id: Uuid,
    current_room: &mut String,
    rx: &mut broadcast::Receiver<RealtimeMessage>,
    sender_label: &mut String,
    text: &str,
) {
    let uplink = parse_uplink(current_room, sender_label, text);
    match uplink {
        Uplink::Im(msg) => {
            // 目标房间若与当前不同则切换订阅（channel 消息 / im 事件跨房间）。
            if msg.room != *current_room {
                manager.leave(current_room, conn_id).await;
                *current_room = msg.room.clone();
                *rx = manager.join(current_room, conn_id).await;
                let _ = send_presence(ws_sender, manager, current_room).await;
            }
            manager.register_sender(current_room, sender_label).await;
            let delivered = manager.broadcast_im(current_room, &msg).await;
            // 回执：向发送者单播投递结果（delivered 为当前在线连接数）。
            let receipt = RealtimeMessage {
                room: current_room.clone(),
                sender: "system".to_string(),
                content: serde_json::json!({ "delivered": delivered }).to_string(),
                msg_type: "receipt".to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
                data: serde_json::json!({ "delivered": delivered }),
            };
            if let Ok(json) = receipt.to_json() {
                if ws_sender.send(Message::Text(json.into())).await.is_err() {
                    warn!(conn_id = %conn_id, "ws receipt send failed");
                }
            }
        }
        Uplink::JoinRoom(target) => {
            if target == *current_room {
                return;
            }
            manager.leave(current_room, conn_id).await;
            *current_room = target.clone();
            *rx = manager.join(current_room, conn_id).await;
            let _ = send_presence(ws_sender, manager, current_room).await;
        }
        Uplink::LeaveRoom => {
            manager.leave(current_room, conn_id).await;
            // 回落到 default 房间，保持连接可用。
            *current_room = "default".to_string();
            *rx = manager.join(current_room, conn_id).await;
            let _ = send_presence(ws_sender, manager, current_room).await;
        }
        Uplink::Presence(sender) => {
            // 声明身份：登记 roster，并作为该连接后续 IM 消息的 sender 标签。
            if !sender.is_empty() {
                *sender_label = sender.clone();
                manager.register_sender(current_room, &sender).await;
            }
            let _ = send_presence(ws_sender, manager, current_room).await;
        }
        Uplink::Ignore => {
            // 心跳 / 未知：无需动作。
        }
    }
}

#[allow(non_snake_case)]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(manager): State<Arc<RealtimeManager>>,
) -> Response {
    ws.on_upgrade(|socket| async move {
        let conn_id = Uuid::new_v4();
        handle_connection(socket, manager, conn_id, "default".to_string()).await;
    })
}

#[allow(non_snake_case)]
pub async fn ws_room_handler(
    ws: WebSocketUpgrade,
    axum::extract::Path(room_id): axum::extract::Path<String>,
    State(manager): State<Arc<RealtimeManager>>,
) -> Response {
    ws.on_upgrade(|socket| async move {
        let conn_id = Uuid::new_v4();
        handle_connection(socket, manager, conn_id, room_id).await;
    })
}

#[allow(non_snake_case)]
pub async fn ws_stats(
    State(manager): State<Arc<RealtimeManager>>,
    axum::extract::Path(room_id): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let count = manager.room_connections(&room_id).await;
    axum::Json(serde_json::json!({
        "room": room_id,
        "connections": count,
        "type": "success"
    }))
}

pub fn ws_route() -> Router {
    Router::new()
        .route("/ws/realtime", get(ws_handler))
        .route("/ws/realtime/room/{room_id}", get(ws_room_handler))
        .route("/ws/realtime/room/{room_id}/stats", get(ws_stats))
        .with_state(Arc::new(RealtimeManager::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_ws_route_exists() {
        use axum::http::Version;
        let app = ws_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws/realtime")
                    .method("GET")
                    .version(Version::HTTP_11)
                    .header("Upgrade", "websocket")
                    .header("Connection", "upgrade")
                    .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
                    .header("Sec-WebSocket-Version", "13")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(
            response.status() == StatusCode::SWITCHING_PROTOCOLS
                || response.status() == StatusCode::UPGRADE_REQUIRED,
            "expected 101 or 426, got {}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_realtime_message_serialization() {
        let msg = RealtimeMessage::new("room1", "user1", "hello", "text");
        let json = msg.to_json().unwrap();
        let parsed: RealtimeMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.room, "room1");
        assert_eq!(parsed.sender, "user1");
        assert_eq!(parsed.content, "hello");
        assert_eq!(parsed.msg_type, "text");
        assert!(parsed.timestamp > 0);
    }

    #[test]
    fn w10_client_envelope_is_normalized_to_im_event() {
        let text = serde_json::json!({
            "type": "im_create",
            "data": {
                "id": "m-1",
                "conversationId": "c-1",
                "body": "{\"type\":\"text\",\"body\":\"hello\"}"
            }
        })
        .to_string();
        let msg = client_message("c-1", "person-1", &text).unwrap();
        assert_eq!(msg.room, "c-1");
        assert_eq!(msg.sender, "person-1");
        assert_eq!(msg.msg_type, "im_create");
        assert_eq!(msg.data["conversationId"], "c-1");
    }

    #[test]
    fn w10_invalid_client_envelope_is_rejected() {
        assert!(client_message("c-1", "person-1", "plain text").is_err());
        assert!(client_message("c-1", "person-1", r#"{"type":"ping"}"#).is_err());
    }

    #[tokio::test]
    async fn test_manager_join_leave() {
        let manager = Arc::new(RealtimeManager::new());
        let conn_id = Uuid::new_v4();
        let _rx = manager.join("room1", conn_id).await;
        assert_eq!(manager.room_connections("room1").await, 1);
        manager.leave("room1", conn_id).await;
        assert_eq!(manager.room_connections("room1").await, 0);
    }

    #[tokio::test]
    async fn test_manager_broadcast() {
        let manager = Arc::new(RealtimeManager::new());
        let conn_id = Uuid::new_v4();
        let mut rx = manager.join("room1", conn_id).await;

        let msg = RealtimeMessage::new("room1", "user1", "hello", "text");
        manager.broadcast("room1", msg.clone()).await;

        let received = rx.try_recv().unwrap();
        assert_eq!(received.content, "hello");
        assert_eq!(received.sender, "user1");
    }

    #[tokio::test]
    async fn test_ws_room_stats_route() {
        let app = ws_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws/realtime/room/test/stats")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // ── P5：IM 完整协议（parse_uplink / presence / receipt / 房间切换）──────

    #[test]
    fn p5_parse_uplink_maps_im_events_verbatim() {
        let text = serde_json::json!({ "type": "im_revoke", "data": { "id": "m-1" } }).to_string();
        match parse_uplink("c-1", "person-1", &text) {
            Uplink::Im(msg) => {
                assert_eq!(msg.room, "c-1");
                assert_eq!(msg.sender, "person-1");
                assert_eq!(msg.msg_type, "im_revoke");
                assert_eq!(msg.data["id"], "m-1");
            }
            other => panic!("expected Im, got {other:?}"),
        }
    }

    #[test]
    fn p5_message_uplink_routes_to_channel_room() {
        // SDK send(channel, data) 产生 {type:'message', channel, data}：
        // 归一为 im_create 并路由到 channel 房间（而非当前房间）。
        let text =
            serde_json::json!({ "type": "message", "channel": "conv-9", "data": { "body": "hi" } })
                .to_string();
        match parse_uplink("default", "person-1", &text) {
            Uplink::Im(msg) => {
                assert_eq!(msg.room, "conv-9");
                assert_eq!(msg.msg_type, "im_create");
                assert_eq!(msg.data["body"], "hi");
            }
            other => panic!("expected Im, got {other:?}"),
        }
    }

    #[test]
    fn p5_control_uplinks_parse() {
        assert!(matches!(
            parse_uplink("default", "p", &serde_json::json!({ "type": "join_room", "data": { "room_id": "room-x" } }).to_string()),
            Uplink::JoinRoom(r) if r == "room-x"
        ));
        assert!(matches!(
            parse_uplink(
                "room-x",
                "p",
                &serde_json::json!({ "type": "leave_room" }).to_string()
            ),
            Uplink::LeaveRoom
        ));
        // presence 上行声明身份（缺省回落当前连接 sender）
        assert!(matches!(
            parse_uplink("room-x", "p", &serde_json::json!({ "type": "presence", "data": { "sender": "alice" } }).to_string()),
            Uplink::Presence(s) if s == "alice"
        ));
        assert!(matches!(
            parse_uplink("room-x", "p", &serde_json::json!({ "type": "presence" }).to_string()),
            Uplink::Presence(s) if s == "p"
        ));
        // 心跳与未知 / 非 JSON 一律忽略（不广播，不拒绝连接）
        assert!(matches!(
            parse_uplink(
                "room-x",
                "p",
                &serde_json::json!({ "type": "ping" }).to_string()
            ),
            Uplink::Ignore
        ));
        assert!(matches!(
            parse_uplink("room-x", "p", "not json"),
            Uplink::Ignore
        ));
        assert!(matches!(
            parse_uplink(
                "room-x",
                "p",
                &serde_json::json!({ "type": "unknown_xyz" }).to_string()
            ),
            Uplink::Ignore
        ));
    }

    #[tokio::test]
    async fn p5_broadcast_im_returns_delivered_connection_count() {
        let manager = Arc::new(RealtimeManager::new());
        let c1 = Uuid::new_v4();
        let c2 = Uuid::new_v4();
        let mut rx1 = manager.join("conv-1", c1).await;
        manager.join("conv-1", c2).await;

        let msg = RealtimeMessage::new("conv-1", "alice", "hello", "im_create");
        let delivered = manager.broadcast_im("conv-1", &msg).await;
        assert_eq!(delivered, 2, "回执 delivered 必须等于房间在线连接数");
        assert_eq!(rx1.recv().await.unwrap().content, "hello");

        // 未知房间没有在线连接，回执为 0（不虚构投递数）
        assert_eq!(manager.broadcast_im("no-such-room", &msg).await, 0);
    }

    #[tokio::test]
    async fn p5_presence_roster_sorted_and_counts_connections() {
        let manager = Arc::new(RealtimeManager::new());
        let c1 = Uuid::new_v4();
        let c2 = Uuid::new_v4();
        let _ = manager.join("conv-1", c1).await;
        let _ = manager.join("conv-1", c2).await;

        manager.register_sender("conv-1", "bob").await;
        manager.register_sender("conv-1", "alice").await;
        manager.register_sender("conv-1", "").await; // 空身份不登记

        let (roster, count) = manager.presence("conv-1").await;
        assert_eq!(
            roster,
            vec!["alice".to_string(), "bob".to_string()],
            "roster 排序稳定"
        );
        assert_eq!(count, 2);
        assert_eq!(manager.presence("ghost").await, (Vec::new(), 0));
    }

    #[tokio::test]
    async fn p5_cross_room_message_switches_subscription() {
        // 复刻 handle_uplink 的房间切换语义：连接在 A，收到 channel=B 的消息
        // → 离开 A、订阅 B；此后 B 的消息只投递给该连接。
        let manager = Arc::new(RealtimeManager::new());
        let conn = Uuid::new_v4();
        let mut current_room = "room-a".to_string();
        let mut rx = manager.join(&current_room, conn).await;
        let other = Uuid::new_v4();
        let mut rx_other = manager.join("room-b", other).await;

        let msg = RealtimeMessage::new("room-b", "alice", "cross", "im_create");
        if msg.room != current_room {
            manager.leave(&current_room, conn).await;
            current_room = msg.room.clone();
            rx = manager.join(&current_room, conn).await;
        }
        manager.register_sender(&current_room, "alice").await;
        let delivered = manager.broadcast_im(&current_room, &msg).await;

        assert_eq!(delivered, 2, "A 房间已无人，B 房间 2 连接");
        assert_eq!(manager.room_connections("room-a").await, 0);
        assert_eq!(rx.recv().await.unwrap().content, "cross");
        assert_eq!(rx_other.recv().await.unwrap().content, "cross");
        assert_eq!(
            manager.presence("room-b").await.0,
            vec!["alice".to_string()]
        );
    }
}

#[cfg(test)]
mod tests_generated;
