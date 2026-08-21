use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde::Serialize;
use tracing::{info, warn};

// ──────────────────────────────────────────────────────────────────────────────
// LDAP 用户自动同步（plan002 U7b）
//
// 门控：LDAP_SYNC_ENABLE=true 启用；配置复用现有
//   LDAP_URL / LDAP_BASE_DN / LDAP_BIND_USER / LDAP_BIND_PWD
//
// 流程：定时搜索 base_dn 下的用户条目（uid/mail）→ 与 auth_person 比对 →
//   缺失用户自动创建（unique_id=uid，随机密码不可知 → 无法密码登录）；
//   已存在用户 mail 变化时增量更新；其余跳过。
//
// 可测试性：LDAP 协议交互抽象为 `LdapDirectory`，数据库交互抽象为
// `PersonStore`。真实实现为 Ldap3Directory / PgPersonStore；单元测试使用
// 内存 MockDirectory + InMemoryPersonStore 验证创建/增量/跳过逻辑。
// ──────────────────────────────────────────────────────────────────────────────

/// 单轮同步结果报告
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct SyncReport {
    /// 新创建的用户数
    pub created: usize,
    /// 增量更新（mail 变化）的用户数
    pub updated: usize,
    /// 无变化而跳过的用户数
    pub skipped: usize,
}

/// LDAP 用户条目（同步关注的最小字段集）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LdapUser {
    pub uid: String,
    pub mail: Option<String>,
}

/// LDAP 目录抽象：屏蔽协议细节，便于 mock 测试。
#[async_trait]
pub trait LdapDirectory: Send + Sync {
    /// 搜索 base_dn 下所有用户条目（uid 必填，mail 可选）
    async fn search_users(&self) -> Result<Vec<LdapUser>, String>;
}

/// 本地人员存储抽象：auth_person 表的读写接口。
#[async_trait]
pub trait PersonStore: Send + Sync {
    /// 返回 unique_id -> 当前 email（None 表示未设置）
    async fn list_person_mail(&self) -> Result<HashMap<String, Option<String>>, String>;
    /// 创建用户（unique_id=uid，随机密码禁止登录）
    async fn create_person(&self, uid: &str, mail: Option<&str>) -> Result<(), String>;
    /// 更新用户 email
    async fn update_person_mail(&self, uid: &str, mail: &str) -> Result<(), String>;
}

/// 同步核心逻辑：目录条目与 auth_person 比对，执行创建/增量更新/跳过。
pub async fn sync_from_directory(
    directory: &dyn LdapDirectory,
    store: &dyn PersonStore,
) -> Result<SyncReport, String> {
    let ldap_users = directory.search_users().await?;
    let existing = store.list_person_mail().await?;

    let mut report = SyncReport::default();
    // 同一轮内去重：LDAP 中出现重复 uid 时只处理第一条
    let mut seen: HashSet<String> = HashSet::new();

    for user in ldap_users {
        if user.uid.is_empty() {
            continue;
        }
        if !seen.insert(user.uid.clone()) {
            continue;
        }
        match existing.get(&user.uid) {
            None => {
                store.create_person(&user.uid, user.mail.as_deref()).await?;
                report.created += 1;
                info!("LDAP sync: created user {}", user.uid);
            }
            Some(current_mail) => {
                let new_mail = user.mail.as_deref().filter(|m| !m.is_empty());
                match new_mail {
                    Some(m) if current_mail.as_deref() != Some(m) => {
                        store.update_person_mail(&user.uid, m).await?;
                        report.updated += 1;
                        info!("LDAP sync: updated mail for user {}", user.uid);
                    }
                    _ => {
                        report.skipped += 1;
                    }
                }
            }
        }
    }

    Ok(report)
}

/// 从环境变量读取同步配置。
/// 仅当 LDAP_SYNC_ENABLE=true 且 LDAP_URL 非空时返回 Some。
#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub url: String,
    pub base_dn: String,
    pub bind_user: String,
    pub bind_password: String,
}

impl SyncConfig {
    pub fn from_env() -> Option<Self> {
        let enable = env::var("LDAP_SYNC_ENABLE").unwrap_or_else(|_| "false".to_string());
        if !Self::is_enabled(&enable) {
            return None;
        }
        let url = env::var("LDAP_URL").ok()?;
        if url.is_empty() {
            warn!("LDAP_SYNC_ENABLE=true but LDAP_URL is empty; sync disabled");
            return None;
        }
        Some(Self {
            url,
            base_dn: env::var("LDAP_BASE_DN").unwrap_or_default(),
            bind_user: env::var("LDAP_BIND_USER").unwrap_or_default(),
            bind_password: env::var("LDAP_BIND_PWD").unwrap_or_default(),
        })
    }

    /// 门控解析（纯函数，便于测试）：仅接受 "true"（大小写不敏感）
    fn is_enabled(enable: &str) -> bool {
        enable.trim().eq_ignore_ascii_case("true")
    }

    /// 同步间隔秒数（LDAP_SYNC_INTERVAL_SECS，默认 3600；非法值回退默认）
    pub fn interval_secs_from_env() -> u64 {
        Self::parse_interval_secs(env::var("LDAP_SYNC_INTERVAL_SECS").ok().as_deref())
    }

    fn parse_interval_secs(raw: Option<&str>) -> u64 {
        raw.and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|s| *s > 0)
            .unwrap_or(DEFAULT_SYNC_INTERVAL_SECS)
    }
}

const DEFAULT_SYNC_INTERVAL_SECS: u64 = 3600;
/// 单轮同步的超时保护，避免挂起的 LDAP 连接永久阻塞 worker 循环
const SYNC_RUN_TIMEOUT_SECS: u64 = 300;

/// 真实 LDAP 目录实现（ldap3 协议）。
pub struct Ldap3Directory {
    config: SyncConfig,
}

impl Ldap3Directory {
    pub fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    /// 从环境变量构建（未启用时返回 None）
    pub fn from_env() -> Option<Self> {
        SyncConfig::from_env().map(Self::new)
    }
}

#[async_trait]
impl LdapDirectory for Ldap3Directory {
    async fn search_users(&self) -> Result<Vec<LdapUser>, String> {
        use ldap3::{LdapConnAsync, Scope, SearchEntry};

        if self.config.base_dn.is_empty() {
            return Err("LDAP_BASE_DN is empty; cannot search users".to_string());
        }

        let (conn, mut ldap) = LdapConnAsync::new(&self.config.url)
            .await
            .map_err(|e| format!("failed to connect to LDAP: {e}"))?;
        ldap3::drive!(conn);

        if !self.config.bind_user.is_empty() {
            let bind_pw = if self.config.bind_password.is_empty() {
                self.config.bind_user.as_str()
            } else {
                self.config.bind_password.as_str()
            };
            ldap.simple_bind(&self.config.bind_user, bind_pw)
                .await
                .map_err(|e| format!("service bind failed: {e}"))?
                .success()
                .map_err(|e| format!("service bind failed: {e}"))?;
        }

        let (entries, _res) = ldap
            .search(
                &self.config.base_dn,
                Scope::Subtree,
                "(uid=*)",
                vec!["uid", "mail"],
            )
            .await
            .map_err(|e| format!("LDAP search failed: {e}"))?
            .success()
            .map_err(|e| format!("LDAP search failed: {e}"))?;

        let mut users = Vec::new();
        for entry in entries {
            let se = SearchEntry::construct(entry);
            let uid = se.attrs.get("uid").and_then(|v| v.first()).cloned();
            let mail = se
                .attrs
                .get("mail")
                .and_then(|v| v.first())
                .cloned()
                .filter(|m| !m.is_empty());
            if let Some(uid) = uid {
                users.push(LdapUser { uid, mail });
            }
        }

        ldap.unbind().await.ok();
        Ok(users)
    }
}

/// PostgreSQL 实现：auth_person 表读写。
pub struct PgPersonStore {
    pool: Pool,
}

impl PgPersonStore {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonStore for PgPersonStore {
    async fn list_person_mail(&self) -> Result<HashMap<String, Option<String>>, String> {
        let client = self.pool.get().await.map_err(|e| format!("db pool: {e}"))?;
        let rows = client
            .query("SELECT unique_id, email FROM auth_person WHERE deleted_at IS NULL", &[])
            .await
            .map_err(|e| format!("query auth_person: {e}"))?;
        let mut map = HashMap::new();
        for row in rows {
            map.insert(row.get::<_, String>("unique_id"), row.get::<_, Option<String>>("email"));
        }
        Ok(map)
    }

    async fn create_person(&self, uid: &str, mail: Option<&str>) -> Result<(), String> {
        let client = self.pool.get().await.map_err(|e| format!("db pool: {e}"))?;
        client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, email, password_hash, locked, created_at) \
                 VALUES ($1, $2, $3, $4, $5, false, NOW())",
                &[&uuid::Uuid::new_v4().to_string(), &uid.to_string(), &uid.to_string(), &mail, &disabled_login_password_hash()],
            )
            .await
            .map_err(|e| format!("insert auth_person: {e}"))?;
        Ok(())
    }

    async fn update_person_mail(&self, uid: &str, mail: &str) -> Result<(), String> {
        let client = self.pool.get().await.map_err(|e| format!("db pool: {e}"))?;
        client
            .execute(
                "UPDATE auth_person SET email = $1, updated_at = NOW() WHERE unique_id = $2",
                &[&mail.to_string(), &uid.to_string()],
            )
            .await
            .map_err(|e| format!("update auth_person: {e}"))?;
        Ok(())
    }
}

/// 生成随机明文对应的 bcrypt 哈希（{bcrypt} 前缀与 auth::password 约定一致）。
/// 明文为一次性随机 UUID 且不落库，等效于禁止密码登录。
fn disabled_login_password_hash() -> String {
    let random = uuid::Uuid::new_v4().to_string();
    let hash = bcrypt::hash(random, bcrypt::DEFAULT_COST).unwrap_or_default();
    format!("{{bcrypt}}{hash}")
}

/// 执行一轮真实同步：从环境配置连接 LDAP，并与 auth_person 表比对。
/// 用法：`sync_users_once(&pool).await`
pub async fn sync_users_once(pool: &Pool) -> Result<SyncReport, String> {
    let directory =
        Ldap3Directory::from_env().ok_or_else(|| "LDAP sync disabled or misconfigured".to_string())?;
    let store = PgPersonStore::new(pool.clone());
    sync_from_directory(&directory, &store).await
}

static SYNC_WORKER_STARTED: OnceLock<()> = OnceLock::new();

/// 启动后台定时同步 worker（幂等：重复调用只启动一次）。
/// 间隔由 LDAP_SYNC_INTERVAL_SECS 控制，默认 3600 秒。
pub fn spawn_sync_worker(pool: Pool) {
    if SYNC_WORKER_STARTED.set(()).is_err() {
        // 已被其他调用方启动：不重复 spawn
        return;
    }

    let interval_secs = SyncConfig::interval_secs_from_env();
    info!(
        "spawning LDAP user sync worker (interval={}s)",
        interval_secs
    );

    tokio::spawn(async move {
        let mut timer = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            timer.tick().await;
            let run = sync_users_once(&pool);
            match tokio::time::timeout(Duration::from_secs(SYNC_RUN_TIMEOUT_SECS), run).await {
                Ok(Ok(report)) => info!(
                    "LDAP user sync finished: created={}, updated={}, skipped={}",
                    report.created, report.updated, report.skipped
                ),
                Ok(Err(e)) => warn!("LDAP user sync failed: {e}"),
                Err(_) => warn!(
                    "LDAP user sync timed out after {}s; will retry next tick",
                    SYNC_RUN_TIMEOUT_SECS
                ),
            }
        }
    });
}

/// 门控入口：仅当 LDAP_SYNC_ENABLE=true 且 LDAP_URL 非空时启动 worker。
/// 在应用初始化处调用（参考 mpweixin spawn_template_queue_worker 幂等模式）。
/// 返回是否实际启动。
pub fn init_from_env(pool: Pool) -> bool {
    if SyncConfig::from_env().is_none() {
        return false;
    }
    if tokio::runtime::Handle::try_current().is_err() {
        warn!("LDAP sync enabled but no tokio runtime; worker not started");
        return false;
    }
    spawn_sync_worker(pool);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 内存 Mock 目录：返回固定条目列表
    #[derive(Debug, Default)]
    struct MockDirectory {
        users: Vec<LdapUser>,
    }

    impl MockDirectory {
        fn new(users: Vec<LdapUser>) -> Self {
            Self { users }
        }
    }

    #[async_trait]
    impl LdapDirectory for MockDirectory {
        async fn search_users(&self) -> Result<Vec<LdapUser>, String> {
            Ok(self.users.clone())
        }
    }

    /// 内存 PersonStore：模拟 auth_person（unique_id -> email）
    #[derive(Debug, Default)]
    struct InMemoryPersonStore {
        persons: std::sync::Mutex<HashMap<String, Option<String>>>,
    }

    impl InMemoryPersonStore {
        fn seed(&self, uid: &str, mail: Option<&str>) {
            self.persons
                .lock()
                .unwrap()
                .insert(uid.to_string(), mail.map(|s| s.to_string()));
        }

        fn get_mail(&self, uid: &str) -> Option<Option<String>> {
            self.persons.lock().unwrap().get(uid).cloned()
        }

        fn contains(&self, uid: &str) -> bool {
            self.persons.lock().unwrap().contains_key(uid)
        }
    }

    #[async_trait]
    impl PersonStore for InMemoryPersonStore {
        async fn list_person_mail(&self) -> Result<HashMap<String, Option<String>>, String> {
            Ok(self.persons.lock().unwrap().clone())
        }

        async fn create_person(&self, uid: &str, mail: Option<&str>) -> Result<(), String> {
            let mut persons = self.persons.lock().unwrap();
            if persons.contains_key(uid) {
                return Err(format!("person already exists: {uid}"));
            }
            // 模拟随机密码禁登录：验证哈希生成器可用且每次不同
            assert!(disabled_login_password_hash().starts_with("{bcrypt}"));
            persons.insert(uid.to_string(), mail.map(|s| s.to_string()));
            Ok(())
        }

        async fn update_person_mail(&self, uid: &str, mail: &str) -> Result<(), String> {
            let mut persons = self.persons.lock().unwrap();
            if !persons.contains_key(uid) {
                return Err(format!("person not found: {uid}"));
            }
            persons.insert(uid.to_string(), Some(mail.to_string()));
            Ok(())
        }
    }

    fn user(uid: &str, mail: Option<&str>) -> LdapUser {
        LdapUser { uid: uid.to_string(), mail: mail.map(|s| s.to_string()) }
    }

    #[tokio::test]
    async fn test_sync_creates_missing_users() {
        let dir = MockDirectory::new(vec![user("alice", Some("alice@example.com")), user("bob", None)]);
        let store = InMemoryPersonStore::default();

        let report = sync_from_directory(&dir, &store).await.unwrap();

        assert_eq!(report.created, 2);
        assert_eq!(report.updated, 0);
        assert_eq!(report.skipped, 0);
        assert_eq!(store.get_mail("alice"), Some(Some("alice@example.com".to_string())));
        assert_eq!(store.get_mail("bob"), Some(None));
    }

    #[tokio::test]
    async fn test_sync_updates_changed_mail() {
        let dir = MockDirectory::new(vec![user("alice", Some("alice@new.example.com"))]);
        let store = InMemoryPersonStore::default();
        store.seed("alice", Some("alice@old.example.com"));

        let report = sync_from_directory(&dir, &store).await.unwrap();

        assert_eq!(report.updated, 1);
        assert_eq!(report.created, 0);
        assert_eq!(report.skipped, 0);
        assert_eq!(store.get_mail("alice"), Some(Some("alice@new.example.com".to_string())));
    }

    #[tokio::test]
    async fn test_sync_skips_unchanged_users() {
        let dir = MockDirectory::new(vec![user("alice", Some("alice@example.com")), user("bob", None)]);
        let store = InMemoryPersonStore::default();
        store.seed("alice", Some("alice@example.com"));
        store.seed("bob", None);

        let report = sync_from_directory(&dir, &store).await.unwrap();

        assert_eq!(report.skipped, 2);
        assert_eq!(report.created, 0);
        assert_eq!(report.updated, 0);
    }

    #[tokio::test]
    async fn test_sync_mixed_scenario() {
        // alice 不变(跳过)、bob mail 变化(更新)、carol 新增、dave 新增无 mail
        let dir = MockDirectory::new(vec![
            user("alice", Some("alice@example.com")),
            user("bob", Some("bob@new.example.com")),
            user("carol", Some("carol@example.com")),
            user("dave", None),
        ]);
        let store = InMemoryPersonStore::default();
        store.seed("alice", Some("alice@example.com"));
        store.seed("bob", Some("bob@old.example.com"));

        let report = sync_from_directory(&dir, &store).await.unwrap();

        assert_eq!(report.created, 2);
        assert_eq!(report.updated, 1);
        assert_eq!(report.skipped, 1);
    }

    #[tokio::test]
    async fn test_sync_ignores_empty_uid_and_deduplicates() {
        let dir = MockDirectory::new(vec![
            user("", Some("ghost@example.com")),
            user("alice", Some("a@example.com")),
            user("alice", Some("dup@example.com")),
        ]);
        let store = InMemoryPersonStore::default();

        let report = sync_from_directory(&dir, &store).await.unwrap();

        assert_eq!(report.created, 1);
        assert_eq!(store.get_mail("alice"), Some(Some("a@example.com".to_string())));
        assert!(!store.contains(""));
    }

    #[test]
    fn test_sync_config_gate_parsing() {
        // 门控解析为纯函数判断，避免测试间全局环境变量竞争
        assert!(SyncConfig::is_enabled("true"));
        assert!(SyncConfig::is_enabled("TRUE"));
        assert!(!SyncConfig::is_enabled("false"));
        assert!(!SyncConfig::is_enabled(""));
        assert!(!SyncConfig::is_enabled("1"));
    }

    #[test]
    fn test_interval_parse_fallback_to_default() {
        // 非法/零值必须回退默认 3600，防止 interval(0) panic
        assert_eq!(SyncConfig::parse_interval_secs(Some("1800")), 1800);
        assert_eq!(SyncConfig::parse_interval_secs(None), DEFAULT_SYNC_INTERVAL_SECS);
        assert_eq!(SyncConfig::parse_interval_secs(Some("abc")), DEFAULT_SYNC_INTERVAL_SECS);
        assert_eq!(SyncConfig::parse_interval_secs(Some("0")), DEFAULT_SYNC_INTERVAL_SECS);
        assert_eq!(SyncConfig::parse_interval_secs(Some("-5")), DEFAULT_SYNC_INTERVAL_SECS);
    }

    #[test]
    fn test_disabled_login_password_hash_is_random_bcrypt() {
        let h1 = disabled_login_password_hash();
        let h2 = disabled_login_password_hash();
        assert!(h1.starts_with("{bcrypt}"));
        assert!(h2.starts_with("{bcrypt}"));
        // 随机明文 + bcrypt 盐 ⇒ 两次生成的哈希必然不同
        assert_ne!(h1, h2);
    }
}
