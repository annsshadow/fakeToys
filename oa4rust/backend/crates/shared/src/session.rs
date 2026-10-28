// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use chrono::{Duration, NaiveDateTime, Utc};
use deadpool_postgres::Pool;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::{AuthConfig, DEFAULT_SESSION_TTL_SECONDS};
use crate::error::AppError;

// ──────────────────────────────────────────────────────────────────────────────
// session
//
// 会话存储（内存 + PostgreSQL 持久化）。单实例部署（2026-10-28 决策），
// 无跨实例缓存/吊销通道。SessionManager 由 main.rs 构造单一实例注入各 router
// 与认证中间件。
// ──────────────────────────────────────────────────────────────────────────────

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub auth_config: AuthConfig,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self::with_config(AuthConfig {
            public_origin: "http://localhost:3000".to_string(),
            cookie_secure: false,
            session_ttl_seconds: DEFAULT_SESSION_TTL_SECONDS,
        })
    }

    pub fn with_config(auth_config: AuthConfig) -> Self {
        let manager = Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: None,
            hmac_secret: std::env::var("SESSION_HMAC_SECRET").ok(),
            auth_config,
        };
        manager.start_threshold_scanner().start_session_sweeper()
    }

    pub fn with_pool(pool: Pool) -> Self {
        Self::with_pool_and_config(
            pool,
            AuthConfig {
                public_origin: "http://localhost:3000".to_string(),
                cookie_secure: false,
                session_ttl_seconds: DEFAULT_SESSION_TTL_SECONDS,
            },
        )
    }

    pub fn with_pool_and_config(pool: Pool, auth_config: AuthConfig) -> Self {
        let manager = Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: Some(pool),
            hmac_secret: std::env::var("SESSION_HMAC_SECRET").ok(),
            auth_config,
        };
        manager.start_threshold_scanner().start_session_sweeper()
    }

    /// 周期性清理内存中已过期、且不会再被访问到的会话条目。
    /// validate_session 只在会话被再次访问时惰性清除；不活跃的过期条目
    /// 若无人清扫会随运行时长无限累积（内存泄漏）。
    fn start_session_sweeper(self) -> Self {
        let sweeper = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
            loop {
                interval.tick().await;
                sweeper.cleanup_expired_sessions().await;
            }
        });
        self
    }

    fn start_threshold_scanner(self) -> Self {
        let scanner = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Some(pool) = &scanner.pool {
                    if let Ok(client) = pool.get().await {
                        if let Ok(rows) = client
                            .query(
                                "SELECT DISTINCT person_unique FROM auth_token_threshold",
                                &[],
                            )
                            .await
                        {
                            for row in rows {
                                let person_unique: String = row.get("person_unique");
                                scanner.broadcast_logout(&person_unique).await;
                            }
                        }
                    }
                }
            }
        });
        self
    }

    fn sign_token(&self, token: &str) -> String {
        if let Some(secret) = &self.hmac_secret {
            let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
            mac.update(token.as_bytes());
            let signature = mac.finalize().into_bytes();
            format!(
                "{}.{}",
                token,
                base64::engine::general_purpose::URL_SAFE.encode(signature)
            )
        } else {
            token.to_string()
        }
    }

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

    pub async fn create_session(
        &self,
        person_unique: String,
        token: String,
    ) -> Result<Session, AppError> {
        let now = Utc::now().naive_utc();
        if self.check_token_threshold(now, &person_unique).await {
            return Err(AppError::BadRequest("too many active sessions".to_string()));
        }

        let expires_at = now + Duration::seconds(self.auth_config.session_ttl_seconds as i64);
        let session = Session {
            token: token.clone(),
            person_unique,
            created_at: now,
            expires_at,
        };

        self.sessions
            .write()
            .await
            .insert(token.clone(), session.clone());

        Self::persist_to_db(
            &self.pool,
            &self.hmac_secret,
            &token,
            &session,
            &now,
            &expires_at,
        )
        .await;
        Ok(session)
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
                    format!(
                        "{}.{}",
                        token,
                        base64::engine::general_purpose::URL_SAFE.encode(signature)
                    )
                } else {
                    token.to_string()
                };
                // 列类型是 TIMESTAMP（迁移 017/schema 实测）：必须绑 NaiveDateTime。
                // 此前按 `%Y-%m-%d %H:%M:%S` 字符串绑定，tokio-postgres 对 timestamp
                // 列报 WrongType，而 `let _ =` 把错误吞掉——会话从未真正写进 PG。
                let _ = client
                    .execute(
                        "INSERT INTO auth_session (token, person_id, expires_at, created_at) \
                     VALUES ($1, $2, $3, $4) \
                     ON CONFLICT (token) DO UPDATE SET expires_at = $3",
                        &[&signed_token, &session.person_unique, expires_at, now],
                    )
                    .await;
                // 顺带清理已过期会话行（每次登录一次的廉价簿记，失败仅延迟清理不破坏
                // 登录，属有意 best-effort）：过期行此前仅在显式登出/删人时删除，
                // 长期运行下 auth_session 无限增长。
                let _ = client
                    .execute("DELETE FROM auth_session WHERE expires_at < NOW()", &[])
                    .await;
            }
        }
    }

    pub async fn check_token_threshold(
        &self,
        token_created_at: NaiveDateTime,
        person_unique: &str,
    ) -> bool {
        let pool = match &self.pool {
            Some(p) => p,
            None => return false,
        };

        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return false,
        };

        // threshold_time 列是 TIMESTAMP（迁移 017）：此前按 String 读，
        // 行存在时 `Row::get` 直接 panic（安全控制反而变成 500 源）。
        let threshold: Option<NaiveDateTime> = match client
            .query_opt(
                "SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1",
                &[&person_unique],
            )
            .await
        {
            Ok(Some(row)) => Some(row.get("threshold_time")),
            _ => return false,
        };

        if let Some(threshold) = threshold {
            token_created_at < threshold
        } else {
            false
        }
    }

    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let session = {
            let sessions = self.sessions.read().await;
            sessions.get(token).cloned()
        };

        match session {
            Some(s) if s.expires_at > Utc::now().naive_utc() => Some(s),
            Some(s) => {
                tracing::debug!(
                    token_prefix = %&token[..token.len().min(8)],
                    in_memory_expires_at = %s.expires_at,
                    now = %Utc::now().naive_utc(),
                    "session validation: in-memory session expired"
                );
                self.remove_session(token).await;
                None
            }
            None => {
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
                            let raw_token =
                                self.verify_and_extract(&row.get::<_, String>("token"))?;
                            // created_at/expires_at 列是 TIMESTAMP：按 NaiveDateTime 读。
                            // 此前按 String 读会在有行时 panic（同 threshold_time 问题）。
                            let created_at: NaiveDateTime = row.get("created_at");
                            let expires_at: NaiveDateTime = row.get("expires_at");
                            let session = Session {
                                token: raw_token,
                                person_unique: row.get("person_id"),
                                created_at,
                                expires_at,
                            };
                            self.sessions
                                .write()
                                .await
                                .insert(token.to_string(), session.clone());
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

        if let Some(pg_pool) = &self.pool {
            if let Ok(client) = pg_pool.get().await {
                let signed_token = self.sign_token(token);
                let _ = client
                    .execute(
                        "DELETE FROM auth_session WHERE token = $1",
                        &[&signed_token],
                    )
                    .await;
            }
        }
    }

    pub async fn remove_sessions_by_person(&self, person_unique: &str) {
        let tokens_to_remove: Vec<String> = {
            let sessions = self.sessions.write().await;
            sessions
                .iter()
                .filter(|(_, s)| s.person_unique == person_unique)
                .map(|(t, _)| t.clone())
                .collect()
        };
        for token in &tokens_to_remove {
            self.sessions.write().await.remove(token);
        }

        if let Some(pg_pool) = &self.pool {
            if let Ok(client) = pg_pool.get().await {
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

    /// 按 token 阈值注销本进程内早于阈值的会话。
    ///
    /// 单实例部署（2026-10-28 决策）：只做本地内存清理，不再向消息总线广播。
    /// 函数名保留是为了不改变 `auth::safe_logout` 的调用面。
    pub async fn broadcast_logout(&self, person_unique: &str) {
        let pool = match &self.pool {
            Some(p) => p,
            None => {
                tracing::debug!(person = %person_unique, "single-instance mode: skipping broadcast");
                return;
            }
        };

        let client = match pool.get().await {
            Ok(c) => c,
            Err(_) => return,
        };

        // threshold_time 列是 TIMESTAMP（迁移 017）：此前按 String 读会在有行时 panic。
        let threshold: NaiveDateTime = match client
            .query_opt(
                "SELECT threshold_time FROM auth_token_threshold WHERE person_unique = $1",
                &[&person_unique],
            )
            .await
        {
            Ok(Some(row)) => row.get("threshold_time"),
            _ => return,
        };

        let expired_tokens: Vec<String> = {
            let sessions = self.sessions.read().await;
            sessions
                .iter()
                .filter(|(_, s)| s.person_unique == person_unique && s.created_at < threshold)
                .map(|(t, _)| t.clone())
                .collect()
        };
        for token in &expired_tokens {
            self.sessions.write().await.remove(token);
        }

        tracing::debug!(person = %person_unique, expired_count = expired_tokens.len(), "single-instance mode: invalidated local sessions before threshold");
    }

    pub async fn cleanup_expired_sessions(&self) {
        let now = Utc::now().naive_utc();
        let expired: Vec<String> = {
            let sessions = self.sessions.read().await;
            sessions
                .iter()
                .filter(|(_, s)| s.expires_at < now)
                .map(|(t, _)| t.clone())
                .collect()
        };
        if expired.is_empty() {
            return;
        }
        for token in &expired {
            self.sessions.write().await.remove(token);
        }
        tracing::info!(expired_count = expired.len(), "cleaned up expired sessions");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────────────────────────────────────
    // 为什么这一组只测「无 pool（纯内存）」路径
    //
    // SessionManager 有两层存储：内存 HashMap → PostgreSQL。PG 分支要求真实
    // 外部服务，单测里只能靠 mock 才有意义（而 mock 掉连接层等于不测连接层）。
    // 剩下的**内存层**恰恰是最容易出错、也最容易漏测的部分：
    // 过期判定、惰性清除、按人批量注销——这三条都直接决定「登出到底有没有生效」。
    //
    // 之前这里只有 17% 覆盖，原因不是难写，而是构造函数会 spawn 后台任务
    // （sweeper / threshold scanner），必须在 tokio 运行时里才不 panic，
    // 而调用方从没在测试里建过它。
    // ──────────────────────────────────────────────────────────────────────

    fn manager_with_ttl(ttl_seconds: u64) -> SessionManager {
        SessionManager::with_config(AuthConfig {
            public_origin: "http://localhost:3000".to_string(),
            cookie_secure: false,
            session_ttl_seconds: ttl_seconds,
        })
    }

    fn session_at(
        token: &str,
        person: &str,
        created: NaiveDateTime,
        expires: NaiveDateTime,
    ) -> Session {
        Session {
            token: token.to_string(),
            person_unique: person.to_string(),
            created_at: created,
            expires_at: expires,
        }
    }

    async fn count(manager: &SessionManager) -> usize {
        manager.sessions.read().await.len()
    }

    #[tokio::test]
    async fn new_manager_has_no_sessions_and_rejects_unknown_token() {
        // 空管理器不能「凭 token 猜出会话」——未知 token 必须返回 None 而不是新建。
        let manager = SessionManager::new();
        assert_eq!(count(&manager).await, 0);
        assert!(manager.validate_session("never-issued").await.is_none());
        assert_eq!(
            count(&manager).await,
            0,
            "拒绝未知 token 时不应写入任何条目"
        );
    }

    #[tokio::test]
    async fn default_matches_new() {
        // Default 与 new 必须等价，否则 `#[derive(Default)]` 的调用方会拿到不同配置。
        let manager = SessionManager::default();
        assert_eq!(
            manager.auth_config.session_ttl_seconds,
            DEFAULT_SESSION_TTL_SECONDS
        );
        assert!(manager.pool.is_none());
    }

    #[tokio::test]
    async fn create_session_applies_configured_ttl_not_hardcoded_default() {
        // TTL 必须来自 AuthConfig。写死 7200 的症状是「改了配置但会话时长不变」，
        // 而这在功能上完全看不出来。
        let manager = manager_with_ttl(60);
        let session = manager
            .create_session("person-a".to_string(), "tok-a".to_string())
            .await
            .expect("no pool means no threshold rejection");

        assert_eq!(session.token, "tok-a");
        assert_eq!(session.person_unique, "person-a");
        assert_eq!((session.expires_at - session.created_at).num_seconds(), 60);
    }

    #[tokio::test]
    async fn create_then_validate_roundtrips_the_same_session() {
        let manager = manager_with_ttl(600);
        manager
            .create_session("person-a".to_string(), "tok-a".to_string())
            .await
            .unwrap();

        let got = manager
            .validate_session("tok-a")
            .await
            .expect("valid session");
        assert_eq!(got.person_unique, "person-a");
        assert_eq!(got.token, "tok-a");
    }

    #[tokio::test]
    async fn validate_evicts_expired_entry_instead_of_leaving_it() {
        // 惰性清除：过期会话被访问时必须**顺手删掉**。
        // 不删的话，不活跃会话会随运行时长无限累积（内存泄漏），
        // 而症状只是「内存慢慢涨」，几乎不可能在功能测试里发现。
        let manager = manager_with_ttl(0); // ttl=0 → expires_at == created_at → 立刻过期
        manager
            .create_session("person-a".to_string(), "tok-a".to_string())
            .await
            .unwrap();
        assert_eq!(count(&manager).await, 1, "创建时先写入内存");

        assert!(manager.validate_session("tok-a").await.is_none());
        assert_eq!(count(&manager).await, 0, "过期会话必须被清除");
    }

    #[tokio::test]
    async fn validate_keeps_valid_entry_cached() {
        // 有效会话不能被误删（过度清除的表现是「随机掉登录」）。
        let manager = manager_with_ttl(600);
        manager
            .create_session("person-a".to_string(), "tok-a".to_string())
            .await
            .unwrap();

        assert!(manager.validate_session("tok-a").await.is_some());
        assert_eq!(count(&manager).await, 1);
    }

    #[tokio::test]
    async fn remove_session_removes_only_the_target_token() {
        let manager = manager_with_ttl(600);
        for (person, token) in [("p1", "t1"), ("p2", "t2")] {
            manager
                .create_session(person.to_string(), token.to_string())
                .await
                .unwrap();
        }

        manager.remove_session("t1").await;

        assert!(manager.validate_session("t1").await.is_none());
        assert!(manager.validate_session("t2").await.is_some());
        assert_eq!(count(&manager).await, 1);
    }

    #[tokio::test]
    async fn remove_sessions_by_person_spares_other_people() {
        // 「删人」必须只清掉这个人的全部会话，不能牵连别人。
        let manager = manager_with_ttl(600);
        for (person, token) in [("alice", "a1"), ("alice", "a2"), ("bob", "b1")] {
            manager
                .create_session(person.to_string(), token.to_string())
                .await
                .unwrap();
        }

        manager.remove_sessions_by_person("alice").await;

        assert!(manager.validate_session("a1").await.is_none());
        assert!(manager.validate_session("a2").await.is_none());
        assert!(
            manager.validate_session("b1").await.is_some(),
            "bob 的会话被误删"
        );
        assert_eq!(count(&manager).await, 1);
    }

    #[tokio::test]
    async fn broadcast_logout_without_pool_is_a_noop() {
        // 单实例模式下没有 threshold 表可查：必须**保持现状**而不是清空会话。
        // 清空的话，单实例部署里每次调用都会把所有人踢下线。
        let manager = manager_with_ttl(600);
        manager
            .create_session("alice".to_string(), "a1".to_string())
            .await
            .unwrap();

        manager.broadcast_logout("alice").await;

        assert_eq!(count(&manager).await, 1);
        assert!(manager.validate_session("a1").await.is_some());
    }

    #[tokio::test]
    async fn check_token_threshold_without_pool_never_rejects() {
        // 无池时阈值检查返回 false（不拒绝登录）。返回 true 会让所有登录失败。
        let manager = manager_with_ttl(600);
        assert!(
            !manager
                .check_token_threshold(Utc::now().naive_utc(), "anyone")
                .await
        );
    }

    #[tokio::test]
    async fn cleanup_expired_sessions_drops_only_expired_entries() {
        let manager = manager_with_ttl(600);
        let now = Utc::now().naive_utc();
        {
            let mut map = manager.sessions.write().await;
            map.insert(
                "stale".to_string(),
                session_at(
                    "stale",
                    "p1",
                    now - Duration::seconds(120),
                    now - Duration::seconds(60),
                ),
            );
            map.insert(
                "live".to_string(),
                session_at(
                    "live",
                    "p1",
                    now - Duration::seconds(10),
                    now + Duration::seconds(60),
                ),
            );
        }

        manager.cleanup_expired_sessions().await;

        let map = manager.sessions.read().await;
        assert!(!map.contains_key("stale"), "过期条目未被清扫");
        assert!(map.contains_key("live"), "未过期条目被误清扫");
    }

    #[tokio::test]
    async fn cleanup_on_empty_manager_is_harmless() {
        // 清扫器每 300s 跑一次，空表是常态：不能因为「没有过期项」就 panic 或写日志爆炸。
        let manager = manager_with_ttl(600);
        manager.cleanup_expired_sessions().await;
        assert_eq!(count(&manager).await, 0);
    }

    #[tokio::test]
    async fn managers_do_not_share_state() {
        // 每个实例独立持有 HashMap。共享（比如误用 static）会让并行测试互相串扰，
        // 也会让「登出只影响自己」这个语义在单实例多租户下失效。
        let a = manager_with_ttl(600);
        let b = manager_with_ttl(600);
        a.create_session("p".to_string(), "t".to_string())
            .await
            .unwrap();

        assert_eq!(count(&a).await, 1);
        assert_eq!(count(&b).await, 0);
        assert!(b.validate_session("t").await.is_none());
    }

    // ──────────────────────────────────────────────────────────────────────
    // PG 分支（真库）。无库时显式 skip（沿用 response.rs 的守卫惯例），
    // 2026-10-29 覆盖率轮补测：此前 62% 的缺口几乎全在这里——持久化、
    // 阈值拦截、库回退重建、删行、按人批量删、阈值注销，这些路径
    // 直接决定「重启后会话还在吗 / 踢人到底踢没踢掉」。
    // ──────────────────────────────────────────────────────────────────────

    /// 固定 HMAC 秘钥的带池管理器：签名格式可预测，测试才能直查 auth_session。
    /// 字段是 `pub`，测试内直接赋值即可（不走 env，避免测试间串扰）。
    fn db_manager(ttl_seconds: u64, secret: Option<&str>) -> SessionManager {
        let mut manager = SessionManager::with_pool_and_config(
            crate::testing::test_pool(),
            AuthConfig {
                public_origin: "http://localhost:3000".to_string(),
                cookie_secure: false,
                session_ttl_seconds: ttl_seconds,
            },
        );
        manager.hmac_secret = secret.map(|s| s.to_string());
        manager
    }

    /// 手动拼装、**不 spawn 后台 sweeper/threshold scanner** 的带池管理器。
    /// 阈值类用例必须用它：扫描器首 tick 立即执行，会在断言窗口内并发清内存，
    /// 让「count==1」这类断言变成竞态。
    fn pool_manager(ttl_seconds: u64, secret: Option<&str>) -> SessionManager {
        SessionManager {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            pool: Some(crate::testing::test_pool()),
            hmac_secret: secret.map(|s| s.to_string()),
            auth_config: AuthConfig {
                public_origin: "http://localhost:3000".to_string(),
                cookie_secure: false,
                session_ttl_seconds: ttl_seconds,
            },
        }
    }

    async fn db() -> deadpool_postgres::Object {
        crate::testing::test_pool().get().await.unwrap()
    }

    async fn cleanup(prefix: &str) {
        let client = db().await;
        let _ = client
            .execute(
                "DELETE FROM auth_session WHERE person_id LIKE $1 || '%'",
                &[&prefix.to_string()],
            )
            .await;
        let _ = client
            .execute(
                "DELETE FROM auth_token_threshold WHERE person_unique LIKE $1 || '%'",
                &[&prefix.to_string()],
            )
            .await;
    }

    async fn row_count(token: &str) -> i64 {
        let client = db().await;
        client
            .query_one(
                "SELECT COUNT(*) FROM auth_session WHERE token = $1",
                &[&token.to_string()],
            )
            .await
            .unwrap()
            .get::<_, i64>(0)
    }

    #[tokio::test]
    async fn pg_create_session_persists_signed_row() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-s1").await;
        let manager = db_manager(600, Some("cov-hmac-secret"));
        let created = manager
            .create_session("cov-s1".to_string(), "cov-tok-1".to_string())
            .await
            .unwrap();

        // 签名格式：token.<base64url(hmac)>；持久化行用的是签名后的完整串
        let signed = manager.sign_token("cov-tok-1");
        assert_ne!(signed, "cov-tok-1", "有秘钥时持久化 token 必须带签名");
        assert_eq!(row_count(&signed).await, 1);

        let client = db().await;
        let row = client
            .query_one(
                "SELECT person_id, expires_at::text, date_trunc('second', created_at)::text AS created_sec \
                 FROM auth_session WHERE token = $1",
                &[&signed],
            )
            .await
            .unwrap();
        assert_eq!(row.get::<_, String>("person_id"), "cov-s1");
        // created_at 以 NaiveDateTime 绑定（微秒精度入库），按秒截断后与内存值对齐
        assert_eq!(
            row.get::<_, String>("created_sec"),
            created.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
        );
        cleanup("cov-s1").await;
        assert_eq!(row_count(&signed).await, 0, "清理必须真的删掉行");
    }

    #[tokio::test]
    async fn pg_persist_without_secret_writes_raw_token() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-s9").await;
        let manager = db_manager(600, None);
        manager
            .create_session("cov-s9".to_string(), "cov-tok-9".to_string())
            .await
            .unwrap();
        // 无秘钥时持久化的就是裸 token（历史上未配 SESSION_HMAC_SECRET 的部署形态）
        assert_eq!(row_count("cov-tok-9").await, 1);
        cleanup("cov-s9").await;
    }

    #[tokio::test]
    async fn pg_threshold_future_blocks_login_and_past_allows() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-t1").await;
        let manager = pool_manager(600, None);

        let client = db().await;
        let future = (Utc::now() + Duration::hours(1)).naive_utc();
        client
            .execute(
                "INSERT INTO auth_token_threshold (person_unique, threshold_time) VALUES ($1, $2)",
                &[&"cov-t1".to_string(), &future],
            )
            .await
            .unwrap();

        let err = manager
            .create_session("cov-t1".to_string(), "cov-tok-t".to_string())
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("too many active sessions"),
            "阈值在未来的登录必须被拦截，实际：{err}"
        );
        assert_eq!(
            row_count("cov-tok-t").await,
            0,
            "被拦截的登录不得持久化会话"
        );

        // 阈值过掉之后（登出/改密场景：把阈值拨到过去），同一人重新登录必须放行
        let past = (Utc::now() - Duration::hours(1)).naive_utc();
        client
            .execute(
                "UPDATE auth_token_threshold SET threshold_time = $2 WHERE person_unique = $1",
                &[&"cov-t1".to_string(), &past],
            )
            .await
            .unwrap();
        manager
            .create_session("cov-t1".to_string(), "cov-tok-t2".to_string())
            .await
            .unwrap();
        assert_eq!(row_count("cov-tok-t2").await, 1);
        cleanup("cov-t1").await;
    }

    #[tokio::test]
    async fn pg_validate_session_falls_back_to_db_and_rehydrates() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-v1").await;
        let manager_a = db_manager(600, Some("cov-hmac-secret"));
        manager_a
            .create_session("cov-v1".to_string(), "cov-tok-2".to_string())
            .await
            .unwrap();

        // 新进程（内存空、同一库同一秘钥）——重启后凭 token 恢复会话
        let manager_b = db_manager(600, Some("cov-hmac-secret"));
        assert_eq!(count(&manager_b).await, 0);
        let restored = manager_b.validate_session("cov-tok-2").await.unwrap();
        assert_eq!(restored.person_unique, "cov-v1");
        assert_eq!(restored.token, "cov-tok-2", "恢复出来的是裸 token");
        assert_eq!(count(&manager_b).await, 1, "库回退命中后必须回填内存缓存");

        // 秘钥不同 → 签名对不上 → 库里有行也查不到（跨部署秘钥轮换的安全性）
        let manager_c = db_manager(600, Some("other-secret"));
        assert!(manager_c.validate_session("cov-tok-2").await.is_none());
        cleanup("cov-v1").await;
    }

    #[tokio::test]
    async fn pg_validate_session_ignores_expired_db_row() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-v2").await;
        let manager = db_manager(600, None);
        let signed = manager.sign_token("cov-tok-3");
        let client = db().await;
        let past = (Utc::now() - Duration::hours(1)).naive_utc();
        client
            .execute(
                "INSERT INTO auth_session (token, person_id, expires_at, created_at) \
                 VALUES ($1, $2, $3, $3)",
                &[&signed, &"cov-v2".to_string(), &past],
            )
            .await
            .unwrap();

        // 库里有行但已过期：SELECT 的 expires_at > NOW() 过滤掉，不得恢复
        assert!(manager.validate_session("cov-tok-3").await.is_none());
        cleanup("cov-v2").await;
    }

    #[tokio::test]
    async fn pg_remove_session_deletes_db_row() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-r1").await;
        let manager = db_manager(600, Some("cov-hmac-secret"));
        manager
            .create_session("cov-r1".to_string(), "cov-tok-4".to_string())
            .await
            .unwrap();
        assert_eq!(row_count(&manager.sign_token("cov-tok-4")).await, 1);

        manager.remove_session("cov-tok-4").await;
        assert_eq!(
            row_count(&manager.sign_token("cov-tok-4")).await,
            0,
            "登出必须同时删掉 PG 行，否则重启后凭旧 token 复活"
        );
        cleanup("cov-r1").await;
    }

    #[tokio::test]
    async fn pg_remove_sessions_by_person_deletes_only_that_persons_rows() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-m1").await;
        cleanup("cov-m2").await;
        let manager = db_manager(600, Some("cov-hmac-secret"));
        manager
            .create_session("cov-m1".to_string(), "cov-tok-5".to_string())
            .await
            .unwrap();
        manager
            .create_session("cov-m2".to_string(), "cov-tok-6".to_string())
            .await
            .unwrap();

        manager.remove_sessions_by_person("cov-m1").await;
        assert_eq!(row_count(&manager.sign_token("cov-tok-5")).await, 0);
        assert_eq!(
            row_count(&manager.sign_token("cov-tok-6")).await,
            1,
            "删人只删该人的会话，不得波及其它人"
        );
        cleanup("cov-m1").await;
        cleanup("cov-m2").await;
    }

    #[tokio::test]
    async fn pg_broadcast_logout_clears_memory_sessions_before_threshold() {
        if !crate::testing::is_db_available().await {
            eprintln!("SKIP: no database available (set DATABASE_URL)");
            return;
        }
        cleanup("cov-b1").await;
        let manager = pool_manager(600, None);
        let client = db().await;
        // 真实时序（safe_logout）：先有会话，登出时才写入阈值 = 登出时刻。
        // 会话 created_at = now < threshold = now+1s → 广播后必须被清。
        manager
            .create_session("cov-b1".to_string(), "cov-tok-7".to_string())
            .await
            .unwrap();
        assert_eq!(count(&manager).await, 1);

        let threshold = (Utc::now() + Duration::seconds(1)).naive_utc();
        client
            .execute(
                "INSERT INTO auth_token_threshold (person_unique, threshold_time) VALUES ($1, $2)",
                &[&"cov-b1".to_string(), &threshold],
            )
            .await
            .unwrap();

        manager.broadcast_logout("cov-b1").await;
        assert_eq!(
            count(&manager).await,
            0,
            "阈值注销必须清掉本进程内早于阈值的会话"
        );

        // 无阈值行的人：直接返回，不得 panic
        manager.broadcast_logout("cov-b1-nobody").await;
        // 阈值行被删（query_opt 返回 None）：直接返回，不得 panic
        client
            .execute(
                "DELETE FROM auth_token_threshold WHERE person_unique = $1",
                &[&"cov-b1".to_string()],
            )
            .await
            .unwrap();
        manager.broadcast_logout("cov-b1").await;
        cleanup("cov-b1").await;
    }
}
