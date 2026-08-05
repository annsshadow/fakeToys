use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// ──────────────────────────────────────────────────────────────────────────────
// session
//
// 内存会话存储（纯 Rust 侧实现，独立于服务端 Session）。
// SessionManager 由 main.rs 构造单一实例注入各 router 与认证中间件，
// 避免认证与限流状态分裂。
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub person_unique: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SessionManager {
    pub sessions: Arc<RwLock<std::collections::HashMap<String, Session>>>,
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
        }
    }

    /// 创建会话
    ///
    /// 生成一个新的 Session，有效期 2 小时，存入内存 HashMap。
    pub async fn create_session(&self, person_unique: String, token: String) -> Session {
        let now = Utc::now();
        let session = Session {
            token: token.clone(),
            person_unique,
            created_at: now,
            expires_at: now + Duration::hours(2),
        };

        self.sessions.write().await.insert(token.clone(), session.clone());
        session
    }

    /// 验证会话令牌是否有效（未过期、存在）
    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let session = {
            let sessions = self.sessions.read().await;
            sessions.get(token).cloned()
        };
        match session {
            Some(s) if s.expires_at > Utc::now() => Some(s),
            Some(_) => {
                self.remove_session(token).await;
                None
            }
            _ => None,
        }
    }

    /// 删除会话（退出登录时使用）
    pub async fn remove_session(&self, token: &str) {
        self.sessions.write().await.remove(token);
    }
}
