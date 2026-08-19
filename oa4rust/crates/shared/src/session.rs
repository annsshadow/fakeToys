use chrono::{Duration, NaiveDateTime, Utc};
use deadpool_postgres::Pool;
use hmac::{Hmac, Mac};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;
use base64::Engine;

use anyhow::Context;
use crate::redis::RedisPool;

// ──────────────────────────────────────────────────────────────────────────────
// session
//
// 会话存储（内存 + PostgreSQL 持久化 + Redis 分布式缓存）。
// SessionManager 由 main.rs 构造单一实例注入各 router 与认证中间件。
// ──────────────────────────────────────────────────────────────────────────────

type HmacSha256 = Hmac<Sha256>;

const SESSION_KEY_PREFIX: &str = "oa4rust:session:";
const SESSION_TTL_SECONDS: u64 = 7200;

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub person_unique: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct SessionManager {
    pub sessions: Arc<RwLock<std::collections::HashMap<String, Session>>>,
    pub pool: Option<Pool>,
    pub hmac_secret: Option<String>,
    pub redis_pool: Arc<std::sync::Mutex<Option<RedisPool>>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: None,
            hmac_secret: std::env::var("SESSION_HMAC_SECRET").ok(),
            redis_pool: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn with_pool(pool: Pool) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: Some(pool),
            hmac_secret: std::env::var("SESSION_HMAC_SECRET").ok(),
            redis_pool: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn init_redis(&self) -> bool {
        let url = match std::env::var("REDIS_URL") {
            Ok(u) if !u.trim().is_empty() => u,
            _ => return false,
        };
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return false,
        };
        match rt.block_on(RedisPool::from_url(&url)) {
            Ok(pool) => {
                let mut g = self.redis_pool.lock().unwrap();
                *g = Some(pool);
                true
            }
            Err(e) => {
                warn!(error = %e, "failed to connect to Redis; session store falling back to in-memory");
                false
            }
        }
    }

    pub async fn init_redis_async(&self) -> bool {
        let url = match std::env::var("REDIS_URL") {
            Ok(u) if !u.trim().is_empty() => u,
            _ => return false,
        };
        let mut guard = self.redis_pool.lock().unwrap();
        if guard.is_some() {
            return true;
        }
        match RedisPool::from_url(&url).await {
            Ok(pool) => {
                *guard = Some(pool);
                true
            }
            Err(e) => {
                warn!(error = %e, "failed to connect to Redis; session store falling back to in-memory");
                false
            }
        }
    }

    fn get_redis_pool(&self) -> Option<RedisPool> {
        self.redis_pool.lock().unwrap().clone()
    }

    fn sign_token(&self, token: &str) -> String {
        if let Some(secret) = &self.hmac_secret {
            let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(token.as_bytes());
            let signature = mac.finalize().into_bytes();
            format!("{}.{}", token, base64::engine::general_purpose::URL_SAFE.encode(signature))
        } else {
            token.to_string()
        }
    }

    fn verify_and_extract(&self, signed_token: &str) -> Option<String> {
        if let Some(secret) = &self.hmac_secret {
            let parts: Vec<&str> = signed_token.split('.').collect();
            if parts.len() != 2 { return None; }
            let token = parts[0];
            let sig = parts[1];
            let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(token.as_bytes());
            let expected = base64::engine::general_purpose::URL_SAFE.decode(sig).ok()?;
            if mac.verify_slice(&expected).is_ok() { Some(token.to_string()) } else { None }
        } else {
            Some(signed_token.to_string())
        }
    }

    pub async fn create_session(&self, person_unique: String, token: String) -> Session {
        let now = Utc::now().naive_utc();
        let expires_at = now + Duration::hours(2).to_std().unwrap_or_default();
        let session = Session {
            token: token.clone(),
            person_unique,
            created_at: now,
            expires_at,
        };

        self.sessions.write().await.insert(token.clone(), session.clone());

        if let Some(ref pool) = self.get_redis_pool() {
            let signed_token = self.sign_token(&token);
            let key = format!("{}{}", SESSION_KEY_PREFIX, signed_token);
            let session_json = match serde_json::to_string(&session) {
                Ok(j) => j,
                Err(_) => {
                    Self::persist_to_db(&self.pool, &self.hmac_secret, &token, &session, &now, &expires_at).await;
                    return session;
                }
            };
            let mut guard = pool.0.manager.lock().await;
            if let Some(conn) = guard.as_mut() {
                let _ = conn.set_ex::<_, _, ()>(key, session_json, SESSION_TTL_SECONDS).await;
            }
        }

        Self::persist_to_db(&self.pool, &self.hmac_secret, &token, &session, &now, &expires_at).await;
        session
    }

    async fn persist_to_db(
        pool: &Option<Pool>,
        hmac_secret: &Option<String>,
        token: &str,
        session: &Session,
        now: &NaiveDateTime,
        expires_at: &NaiveDateTime,
    ) {
        if let Some(pool) = pool {
            if let Ok(client) = pool.get().await {
                let signed_token = if let Some(secret) = hmac_secret {
                    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
                    mac.update(token.as_bytes());
                    let signature = mac.finalize().into_bytes();
                    format!("{}.{}", token, base64::engine::general_purpose::URL_SAFE.encode(signature))
                } else { token.to_string() };
                let created_at_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                let expires_at_str = expires_at.format("%Y-%m-%d %H:%M:%S").to_string();
                let _ = client.execute(
                    "INSERT INTO auth_session (token, person_id, expires_at, created_at) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT (token) DO UPDATE SET expires_at = $3",
                    &[&signed_token, &session.person_unique, &expires_at_str, &created_at_str],
                ).await;
            }
        }
    }

    pub async fn check_token_threshold(&self, token_created_at: NaiveDateTime, person_unique: &str) -> bool {
        if let Some(pool) = &self.pool {
            if let Ok(client) = pool.get().await {
                if let Ok(Some(row)) = client
                    .query_opt(
                        "SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1",
                        &[&person_unique],
                    )
                    .await
                {
                    let threshold_str: String = row.get("threshold_time");
                    if let Ok(threshold) = NaiveDateTime::parse_from_str(&threshold_str, "%Y-%m-%d %H:%M:%S") {
                        return token_created_at >= threshold;
                    }
                }
            }
        }
        true
    }

    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let session = {
            let sessions = self.sessions.read().await;
            sessions.get(token).cloned()
        };

        match session {
            Some(s) if s.expires_at > Utc::now().naive_utc() => {
                if !self.check_token_threshold(s.created_at, &s.person_unique).await {
                    self.remove_session(token).await;
                    return None;
                }
                Some(s)
            }
            Some(_) => { self.remove_session(token).await; None }
                None => {
                    if let Some(ref pool) = self.get_redis_pool() {
                        let signed_token = self.sign_token(token);
                        let key = format!("{}{}", SESSION_KEY_PREFIX, signed_token);
                        let result: Option<String> = {
                            let mut guard = pool.0.manager.lock().await;
                            if let Some(conn) = guard.as_mut() {
                                conn.get::<_, Option<String>>(key.clone()).await.ok().flatten()
                            } else {
                                None
                            }
                        };

                        if let Some(session_json) = result {
                            if let Ok(session) = serde_json::from_str::<Session>(&session_json) {
                                if session.expires_at > Utc::now().naive_utc() {
                                    if !self.check_token_threshold(session.created_at, &session.person_unique).await {
                                        let _ = {
                                            let mut guard = pool.0.manager.lock().await;
                                            if let Some(conn) = guard.as_mut() {
                                                conn.del::<_, ()>(key).await.ok()
                                            } else {
                                                None
                                            }
                                        };
                                        return None;
                                    }
                                    self.sessions.write().await.insert(token.to_string(), session.clone());
                                    return Some(session);
                                } else {
                                    let _ = {
                                        let mut guard = pool.0.manager.lock().await;
                                        if let Some(conn) = guard.as_mut() {
                                            conn.del::<_, ()>(key).await.ok()
                                        } else {
                                            None
                                        }
                                    };
                                }
                            }
                        }
                    }

                if let Some(pg_pool) = &self.pool {
                    if let Ok(client) = pg_pool.get().await {
                        let signed_token = self.sign_token(token);
                        if let Ok(Some(row)) = client
                            .query_opt(
                                "SELECT token, person_id, created_at, expires_at \
                                 FROM auth_session \
                                 WHERE token = $1 AND expires_at > NOW()",
                                &[&signed_token],
                            )
                            .await
                        {
                            let raw_token = self.verify_and_extract(&row.get::<_, String>("token"))?;
                            let created_at_str: String = row.get("created_at");
                            let expires_at_str: String = row.get("expires_at");
                            let created_at = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S").ok()?;
                            let expires_at = NaiveDateTime::parse_from_str(&expires_at_str, "%Y-%m-%d %H:%M:%S").ok()?;
                            let session = Session {
                                token: raw_token,
                                person_unique: row.get("person_id"),
                                created_at,
                                expires_at,
                            };
                            if !self.check_token_threshold(session.created_at, &session.person_unique).await {
                                return None;
                            }
                            self.sessions.write().await.insert(token.to_string(), session.clone());
                            return Some(session);
                        }
                    }
                }
                None
            }
        }
    }

    pub async fn remove_session(&self, token: &str) {
        self.sessions.write().await.remove(token);

        if let Some(ref pool) = self.get_redis_pool() {
            let signed_token = self.sign_token(token);
            let key = format!("{}{}", SESSION_KEY_PREFIX, signed_token);
            let mut guard = pool.0.manager.lock().await;
            if let Some(conn) = guard.as_mut() {
                let _ = conn.del::<_, ()>(key).await;
            }
        }

        if let Some(pg_pool) = &self.pool {
            if let Ok(client) = pg_pool.get().await {
                let signed_token = self.sign_token(token);
                let _ = client.execute("DELETE FROM auth_session WHERE token = $1", &[&signed_token]).await;
            }
        }
    }

    pub async fn remove_sessions_by_person(&self, person_unique: &str) {
        let tokens_to_remove: Vec<String> = {
            let sessions = self.sessions.write().await;
            sessions.iter().filter(|(_, s)| s.person_unique == person_unique).map(|(t, _)| t.clone()).collect()
        };
        for token in &tokens_to_remove {
            self.sessions.write().await.remove(token);
        }

        if let Some(ref pool) = self.get_redis_pool() {
            let pattern = format!("{}*", SESSION_KEY_PREFIX);
            let keys: Vec<String> = {
                let mut guard = pool.0.manager.lock().await;
                if let Some(conn) = guard.as_mut() {
                    conn.keys::<_, Vec<String>>(pattern.clone()).await.ok().unwrap_or_default()
                } else {
                    Vec::new()
                }
            };

            for key in keys {
                let result: Option<String> = {
                    let mut guard = pool.0.manager.lock().await;
                    if let Some(conn) = guard.as_mut() {
                        conn.get::<_, Option<String>>(key.clone()).await.ok().flatten()
                    } else {
                        None
                    }
                };
                if let Some(session_json) = result {
                    if let Ok(session) = serde_json::from_str::<Session>(&session_json) {
                        if session.person_unique == person_unique {
                            let _ = {
                                let mut guard = pool.0.manager.lock().await;
                                if let Some(conn) = guard.as_mut() {
                                    conn.del::<_, ()>(key).await.ok()
                                } else {
                                    None
                                }
                            };
                        }
                    }
                }
            }
        }

        if let Some(pg_pool) = &self.pool {
            if let Ok(client) = pg_pool.get().await {
                if let Err(e) = client.execute(
                    "DELETE FROM auth_session WHERE person_id = $1",
                    &[&person_unique],
                ).await {
                    tracing::warn!(person = %person_unique, error = %e, "failed to batch-delete auth_session rows");
                }
            }
        }
    }

    pub async fn broadcast_logout(&self, person_unique: &str) {
        let pool = match &self.pool {
            Some(p) => p,
            None => { tracing::debug!(person = %person_unique, "single-instance mode: skipping broadcast"); return; }
        };

        let client = match pool.get().await { Ok(c) => c, Err(_) => return };

        let threshold: NaiveDateTime = match client
            .query_opt("SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1", &[&person_unique])
            .await
        {
            Ok(Some(row)) => {
                let threshold_str: String = row.get("threshold_time");
                let threshold = match NaiveDateTime::parse_from_str(&threshold_str, "%Y-%m-%d %H:%M:%S") {
                    Ok(t) => t,
                    Err(_) => return,
                };
                threshold
            }
            _ => return,
        };

        let expired_tokens: Vec<String> = {
            let sessions = self.sessions.read().await;
            sessions.iter().filter(|(_, s)| s.person_unique == person_unique && s.created_at < threshold).map(|(t, _)| t.clone()).collect()
        };
        for token in &expired_tokens {
            self.sessions.write().await.remove(token);
        }
        tracing::debug!(person = %person_unique, expired_count = expired_tokens.len(), "multi-instance broadcast: invalidated sessions before threshold");
    }
}
