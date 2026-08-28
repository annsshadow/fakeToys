use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use reqwest::Client;
use base64::Engine;
use uuid::Uuid;
use thiserror::Error;

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT COUNT(*) as cnt FROM x_jpush WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.map(|r| r.get("cnt")).unwrap_or(0);
    let enabled = count > 0;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(enabled)),
            ("defaultAppKey".to_string(), Value::String("default".to_string())),
            ("maxPushCount".to_string(), Value::Number(serde_json::Number::from(10000i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_apps(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT target as id, COUNT(*) as cnt FROM x_jpush WHERE deleted_at IS NULL GROUP BY target ORDER BY target ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("id"))),
                ("enabled".to_string(), Value::Bool(row.get::<_, i64>("cnt") > 0)),
            ]))
        })
        .collect();

    let total_data = data.len();
    Ok(Json(ActionResult::java_success(Value::Array(data), total_data as i64, 0)))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let config = body.0;
    tracing::info!("Updating jpush assemble control config: {:?}", config);

    let id = uuid::Uuid::new_v4().to_string();
    let title = config.get("name").and_then(|v| v.as_str()).unwrap_or("default").to_string();

    let result = client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &"", &"all", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
            ("config".to_string(), config),
        ]),
    ))))
}

pub fn jpush_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_assemble_control_router(pool)
}


#[derive(Debug, serde::Deserialize)]
pub struct JpushRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub target: Option<String>,
}

#[axum::debug_handler]
pub async fn list_jpushs(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn get_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("jpush not found"))),
    }
}

#[axum::debug_handler]
pub async fn create_jpush(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let target = req.target.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &content, &target, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("content".to_string(), Value::String(content)),
        ("target".to_string(), Value::String(target)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn save_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = req.title.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let target = req.target.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_jpush SET title = $1, content = $2, target = $3 WHERE id = $4 AND deleted_at IS NULL",
            &[&title, &content, &target, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("jpush not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("title".to_string(), Value::String(title)),
            ("content".to_string(), Value::String(content)),
            ("target".to_string(), Value::String(target)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("jpush not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn device_admin_unbind_all_person(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    if person_id.is_empty() {
        return Ok(Json(ActionResult::error("personId is required")));
    }

    let result = client
        .execute(
            "DELETE FROM x_jpush WHERE creator = $1 AND deleted_at IS NULL",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            ("unbound".to_string(), Value::Bool(result > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn device_bind(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let device_name = req.get("deviceName").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let device_type = req.get("deviceType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let push_type = req.get("pushType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = req.get("personId").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    let id = uuid::Uuid::new_v4().to_string();

    let result = client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &device_name, &device_type, &push_type, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::new()))))
}

pub async fn device_check_deviceName_deviceType_pushType(
    pool: Extension<Pool>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE title = $1 AND content = $2 AND target = $3 AND deleted_at IS NULL LIMIT 1",
            &[&device_name, &device_type, &push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let exists = row.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(exists)),
        ]),
    ))))
}

pub async fn device_config_push_type(
    pool: Extension<Pool>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_jpush WHERE target = $1 AND deleted_at IS NULL",
            &[&push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("pushType".to_string(), Value::String(push_type)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn device_list_pushType(
    pool: Extension<Pool>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE target = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn device_unbind_new_deviceName_deviceType_pushType(
    pool: Extension<Pool>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE title = $1 AND content = $2 AND target = $3 AND deleted_at IS NULL",
            &[&device_name, &device_type, &push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("device not found or already unbound")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("pushType".to_string(), Value::String(push_type)),
            ("unbound".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn device_unbind_deviceName_deviceType(
    pool: Extension<Pool>,
    Path((device_name, device_type)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE title = $1 AND content = $2 AND deleted_at IS NULL",
            &[&device_name, &device_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("device not found or already unbound")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("unbound".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn message_test_send(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, target, creator FROM x_jpush WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 10",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let messages: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
            ]))
        })
        .collect();

    let gateway: Arc<dyn PushGateway> = Arc::new(MockPushGateway::new());
    let mut push_sent = false;

    for msg in &messages {
        let title = msg.get("title").and_then(|v| v.as_str()).unwrap_or("test");
        let content = msg.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let target = msg.get("target").and_then(|v| v.as_str()).unwrap_or("all");

        match gateway.send_push(title, content, target).await {
            Ok(_) => push_sent = true,
            Err(_) => continue,
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("pushed".to_string(), Value::Bool(push_sent)),
            ("count".to_string(), Value::Number(serde_json::Number::from(messages.len() as i64))),
            ("messages".to_string(), Value::Array(messages)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// Push Gateway Abstraction
// ──────────────────────────────────────────────────────────────────────────────

#[async_trait]
pub trait PushGateway: Send + Sync {
    async fn send_push(
        &self,
        title: &str,
        content: &str,
        target: &str,
    ) -> Result<PushResult, PushError>;
}

#[derive(Debug, Clone)]
pub struct PushResult {
    pub message_id: String,
    pub sent_at: String,
    pub status: PushStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PushStatus {
    Sent,
    Failed,
}

impl PushStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PushStatus::Sent => "sent",
            PushStatus::Failed => "failed",
        }
    }
}

#[derive(Debug, Error)]
pub enum PushError {
    #[error("gateway error: {0}")]
    Gateway(String),
    #[error("network error")]
    Network,
    #[error("bad request: {0}")]
    BadRequest(String),
}

#[derive(Debug, Default)]
pub struct MockPushGateway {
    pub sent_messages: Mutex<Vec<(String, String, String)>>,
}

impl MockPushGateway {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sent_count(&self) -> usize {
        self.sent_messages.lock().unwrap().len()
    }

    pub fn reset(&self) {
        self.sent_messages.lock().unwrap().clear();
    }
}

#[async_trait]
impl PushGateway for MockPushGateway {
    async fn send_push(
        &self,
        title: &str,
        content: &str,
        target: &str,
    ) -> Result<PushResult, PushError> {
        let result = PushResult {
            message_id: Uuid::new_v4().to_string(),
            sent_at: Utc::now().to_rfc3339(),
            status: PushStatus::Sent,
        };
        self.sent_messages
            .lock()
            .unwrap()
            .push((title.to_string(), content.to_string(), target.to_string()));
        tracing::info!(
            "[Push:mock] title={} content={} target={} status=sent",
            title, content, target
        );
        Ok(result)
    }
}

#[derive(Debug, Default)]
pub struct ConsolePushGateway;

#[async_trait]
impl PushGateway for ConsolePushGateway {
    async fn send_push(
        &self,
        title: &str,
        content: &str,
        target: &str,
    ) -> Result<PushResult, PushError> {
        let result = PushResult {
            message_id: Uuid::new_v4().to_string(),
            sent_at: Utc::now().to_rfc3339(),
            status: PushStatus::Sent,
        };
        eprintln!(
            "[Push:console] title={} content={} target={} status=sent",
            title, content, target
        );
        Ok(result)
    }
}

pub struct JPushGateway {
    app_key: String,
    app_secret: String,
    client: Client,
}

impl JPushGateway {
    pub fn new(app_key: String, app_secret: String) -> Self {
        Self {
            app_key,
            app_secret,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl PushGateway for JPushGateway {
    async fn send_push(
        &self,
        title: &str,
        content: &str,
        target: &str,
    ) -> Result<PushResult, PushError> {
        let auth = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", self.app_key, self.app_secret));

        let payload = serde_json::json!({
            "platform": ["ios", "android"],
            "audience": {
                "alias": [target]
            },
            "notification": {
                "ios": {
                    "alert": content,
                    "sound": "default",
                    "title": title,
                },
                "android": {
                    "alert": content,
                    "title": title,
                    "sound": "default",
                },
            },
            "options": {
                "time_to_live": 86400,
                "apns_production": false,
            },
        });

        let resp = self
            .client
            .post("https://api.jpush.cn/v3/push")
            .header("Authorization", format!("Basic {}", auth))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|_| PushError::Network)?;

        let status = resp.status();
        let body: Value = resp
            .json()
            .await
            .map_err(|_| PushError::Network)?;

        if !status.is_success() {
            let err_msg = body["error"]["message"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            return Err(PushError::BadRequest(err_msg));
        }

        let message_id = body["msg_id"]
            .as_str()
            .or_else(|| body["data"][0]["msg_id"].as_str())
            .unwrap_or_default()
            .to_string();

        Ok(PushResult {
            message_id,
            sent_at: Utc::now().to_rfc3339(),
            status: PushStatus::Sent,
        })
    }
}



pub async fn message_send(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let target = req.get("target").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = req.get("creator").and_then(|v| v.as_str()).unwrap_or("system");
    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &content, &target, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::new()))))
}
