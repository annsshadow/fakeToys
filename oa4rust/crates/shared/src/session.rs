use chrono::{Duration, NaiveDateTime, Utc};
use deadpool_postgres::Pool;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::RwLock;
use base64::Engine;

// ──────────────────────────────────────────────────────────────────────────────
// session
//
// 会话存储（内存 + PostgreSQL 持久化）。
// SessionManager 由 main.rs 构造单一实例注入各 router 与认证中间件，
// 避免认证与限流状态分裂。
// ──────────────────────────────────────────────────────────────────────────────

type HmacSha256 = Hmac<Sha256>;

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
        }
    }

    /// 创建带数据库持久化的 SessionManager
    pub fn with_pool(pool: Pool) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: Some(pool),
            hmac_secret: std::env::var("SESSION_HMAC_SECRET").ok(),
        }
    }

    /// 签名 token（HMAC-SHA256）
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

    /// 验证签名并提取原始 token
    fn verify_and_extract(&self, signed_token: &str) -> Option<String> {
        if let Some(secret) = &self.hmac_secret {
            let parts: Vec<&str> = signed_token.split('.').collect();
            if parts.len() != 2 {
                return None;
            }
            let token = parts[0];
            let sig = parts[1];

            let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(token.as_bytes());
            let expected = base64::engine::general_purpose::URL_SAFE.decode(sig).ok()?;
            if mac.verify_slice(&expected).is_ok() {
                Some(token.to_string())
            } else {
                None
            }
        } else {
            Some(signed_token.to_string())
        }
    }

    /// 创建会话
    ///
    /// 生成一个新的 Session，有效期 2 小时，同时保存到内存和数据库。
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

        if let Some(pool) = &self.pool {
            if let Ok(client) = pool.get().await {
                let signed_token = self.sign_token(&token);
                let created_at_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                let expires_at_str = expires_at.format("%Y-%m-%d %H:%M:%S").to_string();
                let _ = client
                    .execute(
                        "INSERT INTO auth_session (token, person_id, expires_at, created_at) \
                         VALUES ($1, $2, $3, $4) \
                         ON CONFLICT (token) DO UPDATE SET expires_at = $3",
                        &[&signed_token, &session.person_unique, &expires_at_str, &created_at_str],
                    )
                    .await;
            }
        }

        session
    }

    /// 检查 TokenThreshold：若会话创建时间早于用户最新注销时间戳，则拒绝该会话。
    /// 多实例场景下，safe_logout 写入 TokenThreshold 后，其他实例通过此方法
    /// 在验证会话时感知注销事件，使旧 token 失效。
    /// 返回 true 表示会话有效（未被注销），false 表示已被注销。
    pub async fn check_token_threshold(&self, token_created_at: NaiveDateTime, person_unique: &str) -> bool {
        if let Some(pool) = &self.pool {
            if let Ok(client) = pool.get().await {
                if let Ok(row) = client
                    .query_opt(
                        "SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1",
                        &[&person_unique],
                    )
                    .await
                {
                    if let Some(row) = row {
                        let threshold_str: String = row.get("threshold_time");
                        if let Ok(threshold) = NaiveDateTime::parse_from_str(&threshold_str, "%Y-%m-%d %H:%M:%S") {
                            return token_created_at >= threshold;
                        }
                    }
                }
            }
        }
        true
    }

    /// 验证会话令牌是否有效（未过期、存在、未被 TokenThreshold 注销）
    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let session = {
            let sessions = self.sessions.read().await;
            sessions.get(token).cloned()
        };

        match session {
            Some(s) if s.expires_at > Utc::now().naive_utc() => {
                // 检查 TokenThreshold
                if !self.check_token_threshold(s.created_at, &s.person_unique).await {
                    self.remove_session(token).await;
                    return None;
                }
                Some(s)
            }
            Some(_) => {
                self.remove_session(token).await;
                None
            }
            None => {
                if let Some(pool) = &self.pool {
                    if let Ok(client) = pool.get().await {
                        let signed_token = self.sign_token(token);
                        if let Ok(row) = client
                            .query_opt(
                                "SELECT token, person_id, created_at, expires_at \
                                 FROM auth_session \
                                 WHERE token = $1 AND expires_at > NOW()",
                                &[&signed_token],
                            )
                            .await
                        {
                            if let Some(row) = row {
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
                                // 检查 TokenThreshold
                                if !self.check_token_threshold(session.created_at, &session.person_unique).await {
                                    return None;
                                }
                                self.sessions
                                    .write()
                                    .await
                                    .insert(token.to_string(), session.clone());
                                return Some(session);
                            }
                        }
                    }
                }
                None
            }
        }
    }

    /// 删除会话（退出登录时使用）
    pub async fn remove_session(&self, token: &str) {
        self.sessions.write().await.remove(token);

        if let Some(pool) = &self.pool {
            if let Ok(client) = pool.get().await {
                let signed_token = self.sign_token(token);
                let _ = client
                    .execute("DELETE FROM auth_session WHERE token = $1", &[&signed_token])
                    .await;
            }
        }
    }

    /// 删除指定用户的所有会话（安全注销时使用）
    ///
    /// 遍历内存 sessions 和数据库 auth_session，批量移除所有属于 person_unique 的会话。
    /// 注意：此操作会强制该用户所有设备同时下线。
    pub async fn remove_sessions_by_person(&self, person_unique: &str) {
        // 先移除内存中的会话
        let tokens_to_remove: Vec<String> = {
            let sessions = self.sessions.write().await;
            sessions
                .iter()
                .filter(|(_, s)| s.person_unique == person_unique)
                .map(|(token, _)| token.clone())
                .collect()
        };

        for token in &tokens_to_remove {
            self.sessions.write().await.remove(token);
        }

        // 再从数据库移除
        if let Some(pool) = &self.pool {
            if let Ok(client) = pool.get().await {
                if let Err(e) = client
                    .execute(
                        "DELETE FROM auth_session WHERE person_id = $1",
                        &[&person_unique],
                    )
                    .await
                {
                    tracing::warn!(person = %person_unique, error = %e, "failed to batch-delete auth_session rows");
                }
            }
        }
    }

    /// 多实例安全注销广播
    ///
    /// 单实例场景：仅执行本地 session 移除（已在 remove_sessions_by_person 中完成），
    /// 此方法仅记录日志并返回。
    ///
    /// 多实例场景：通过数据库广播 TokenThreshold，使其他实例在验证会话时
    /// 感知注销事件。TokenThreshold 已在 safe_logout 中写入，此方法
    /// 确保所有实例的本地缓存中早于阈值的 session 被清除。
    pub async fn broadcast_logout(&self, person_unique: &str) {
        // 单实例模式：无数据库连接时直接返回
        let pool = match &self.pool {
            Some(p) => p,
            None => {
                tracing::debug!(person = %person_unique, "single-instance mode: skipping broadcast");
                return;
            }
        };

        // 多实例模式：读取最新 TokenThreshold，清除本地缓存中早于阈值的 session
        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return,
        };

        let threshold: NaiveDateTime = match client
            .query_opt(
                "SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1",
                &[&person_unique],
            )
            .await
        {
            Ok(Some(row)) => {
                let threshold_str: String = row.get("threshold_time");
                match NaiveDateTime::parse_from_str(&threshold_str, "%Y-%m-%d %H:%M:%S") {
                    Ok(t) => t,
                    Err(_) => return,
                }
            }
            _ => return,
        };

        // 清除本地缓存中早于阈值的 session
        let expired_tokens: Vec<String> = {
            let sessions = self.sessions.read().await;
            sessions
                .iter()
                .filter(|(_, s)| s.person_unique == person_unique && s.created_at < threshold)
                .map(|(token, _)| token.clone())
                .collect()
        };

        for token in &expired_tokens {
            self.sessions.write().await.remove(token);
        }

        tracing::debug!(
            person = %person_unique,
            expired_count = expired_tokens.len(),
            "multi-instance broadcast: invalidated sessions before threshold"
        );
    }
}
