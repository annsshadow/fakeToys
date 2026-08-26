use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use serde_json::{json, Value};
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────────────────────
// bind — 扫码登录（绑定）
//
// 流程：GET /jaxrs/authentication/bind 生成二维码内容（meta）→ 已登录用户
// 扫码确认（POST /jaxrs/authentication/bind/meta/{meta}，需携带会话令牌）→
// 客户端轮询 GET /jaxrs/authentication/bind/meta/{meta}，确认后返回会话 token。
//
// 安全：仅在已确认扫码授权后签发会话（meta 一次性，5 分钟过期）。
// 此路径豁免认证中间件（见 shared AUTH_EXEMPT_PATHS），确认端点
// 自行校验请求中的会话令牌。
// ──────────────────────────────────────────────────────────────────────────────

const TTL_MINUTES: i64 = 5;

#[derive(Debug, Clone)]
struct BindEntry {
    confirmed: bool,
    person_unique: Option<String>,
    expires_at: DateTime<Utc>,
}

/// 扫码绑定存储（可独立构造用于测试；运行时使用全局单例）
pub struct BindStore {
    entries: Mutex<HashMap<String, BindEntry>>,
}

impl Default for BindStore {
    fn default() -> Self {
        Self::new()
    }
}

impl BindStore {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    fn cleanup(&self) {
        let now = Utc::now();
        if let Ok(mut map) = self.entries.lock() {
            map.retain(|_, e| e.expires_at > now);
        }
    }

    /// 生成一次性 meta
    pub fn create(&self) -> String {
        self.cleanup();
        let meta = Uuid::new_v4().to_string();
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                meta.clone(),
                BindEntry {
                    confirmed: false,
                    person_unique: None,
                    expires_at: Utc::now() + Duration::minutes(TTL_MINUTES),
                },
            );
        }
        meta
    }

    /// 已登录用户确认扫码绑定；meta 不存在或已确认/过期返回 false
    pub fn confirm(&self, meta: &str, person_unique: &str) -> bool {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return false;
        };
        match map.get_mut(meta) {
            Some(e) if !e.confirmed && e.expires_at > Utc::now() => {
                e.confirmed = true;
                e.person_unique = Some(person_unique.to_string());
                true
            }
            _ => false,
        }
    }

    /// 轮询：确认且未过期时返回绑定用户，并删除（一次性）
    pub fn poll(&self, meta: &str) -> Option<String> {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return None;
        };
        match map.get(meta) {
            Some(e) if e.confirmed => {
                let person = e.person_unique.clone();
                map.remove(meta);
                person
            }
            _ => None,
        }
    }
}

fn bind_store() -> &'static BindStore {
    static STORE: OnceLock<BindStore> = OnceLock::new();
    STORE.get_or_init(BindStore::new)
}

/// GET /jaxrs/authentication/bind —— 生成扫码登录二维码内容
pub async fn bind() -> Result<Json<ActionResult<Value>>, AppError> {
    let meta = bind_store().create();
    Ok(Json(ActionResult::success(json!({
        "meta": meta,
        "status": "pending",
        "image": format!("https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}", meta),
        "id": meta,
    }))))
}

/// POST /jaxrs/authentication/bind/meta/{meta} —— 已登录用户确认扫码绑定
pub async fn bind_confirm(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(meta): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    if bind_store().confirm(&meta, &session.person_unique) {
        Ok(Json(ActionResult::success(json!({ "confirmed": true }))))
    } else {
        Ok(Json(ActionResult::error("meta invalid or expired")))
    }
}

/// GET /jaxrs/authentication/bind/meta/{meta} —— 轮询绑定结果
pub async fn bind_poll(
    session_manager: Extension<SessionManager>,
    Path(meta): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match bind_store().poll(&meta) {
        Some(person_unique) => {
            let token = Uuid::new_v4().to_string();
            let session = session_manager
                .create_session(person_unique.clone(), token.clone())
                .await?;
            Ok(Json(ActionResult::success(json!({
                "status": "confirmed",
                "token": session.token,
                "name": person_unique,
                "id": session.token,
                "tokenType": "Bearer",
            }))))
        }
        None => Ok(Json(ActionResult::success(json!({ "status": "pending" })))),
    }
}

pub fn bind_router() -> Router {
    Router::new()
        .route("/jaxrs/authentication/bind", get(bind))
        .route(
            "/jaxrs/authentication/bind/meta/{meta}",
            get(bind_poll).post(bind_confirm),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_confirm_then_poll() {
        let store = BindStore::new();
        let meta = store.create();
        assert_eq!(store.poll(&meta), None);
        assert!(store.confirm(&meta, "person-1"));
        assert_eq!(store.poll(&meta), Some("person-1".to_string()));
        assert_eq!(store.poll(&meta), None);
    }

    #[test]
    fn test_bind_confirm_unknown_meta() {
        let store = BindStore::new();
        assert!(!store.confirm("nope", "person-1"));
    }

    #[test]
    fn test_bind_poll_without_confirm() {
        let store = BindStore::new();
        let meta = store.create();
        assert_eq!(store.poll(&meta), None);
        assert_eq!(store.poll(&meta), None);
    }
}
