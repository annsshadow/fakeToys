use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    Router,
    extract::{State, ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade}},
    response::Response,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{Mutex, broadcast};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RealtimeError {
    #[error("room not found: {0}")]
    RoomNotFound(String),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

pub type RealtimeResult<T> = Result<T, RealtimeError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMessage {
    pub room: String,
    pub sender: String,
    pub content: String,
    pub msg_type: String,
    pub timestamp: i64,
}

impl RealtimeMessage {
    pub fn new(room: impl Into<String>, sender: impl Into<String>, content: impl Into<String>, msg_type: impl Into<String>) -> Self {
        Self {
            room: room.into(),
            sender: sender.into(),
            content: content.into(),
            msg_type: msg_type.into(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn to_json(&self) -> RealtimeResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

struct RoomHandle {
    room_id: String,
    tx: broadcast::Sender<RealtimeMessage>,
    connections: Arc<Mutex<HashSet<Uuid>>>,
}

#[derive(Clone)]
pub struct RealtimeManager {
    rooms: Arc<Mutex<HashMap<String, RoomHandle>>>,
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

async fn handle_connection(socket: WebSocket, manager: Arc<RealtimeManager>, conn_id: Uuid, room_id: String) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let mut rx = manager.join(&room_id, conn_id).await;

    let welcome = RealtimeMessage::new(
        room_id.clone(),
        "system",
        format!("connected: {}", conn_id),
        "system",
    );
    if let Ok(json) = welcome.to_json() {
        if ws_sender.send(Message::Text(json.into())).await.is_err() {
            manager.leave(&room_id, conn_id).await;
            return;
        }
    }

    let mut heartbeat = tokio::time::interval(Duration::from_secs(30));
    let mut last_pong = tokio::time::Instant::now();

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
                        let msg = RealtimeMessage::new(
                            room_id.clone(),
                            conn_id.to_string(),
                            text.to_string(),
                            "text",
                        );
                        manager.broadcast(&room_id, msg).await;
                        last_pong = tokio::time::Instant::now();
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
        .send(Message::Close(Some(CloseFrame { code: axum::extract::ws::close_code::NORMAL, reason: "bye".into() })))
        .await;
    manager.leave(&room_id, conn_id).await;
    info!(conn_id = %conn_id, room = %room_id, "ws closed");
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(manager): State<Arc<RealtimeManager>>,
) -> Response {
    ws.on_upgrade(|socket| async move {
        let conn_id = Uuid::new_v4();
        handle_connection(socket, manager, conn_id, "default".to_string()).await;
    })
}

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
        use axum::http::{HeaderValue, Version};
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

    #[tokio::test]
    async fn test_manager_join_leave() {
        let manager = Arc::new(RealtimeManager::new());
        let conn_id = Uuid::new_v4();
        let mut rx = manager.join("room1", conn_id).await;
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
}

#[cfg(test)]
mod tests_generated;
