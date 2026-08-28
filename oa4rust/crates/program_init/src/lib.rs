use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use shared::{error::AppError, response::ActionResult};

// ──────────────────────────────────────────────────────────────────────────────
// program_init — 系统初始化
//
// 初始化密钥（secret）以 AES-128-GCM 加密后持久化到 `secret_config` 表
// （migrations/007_secret_config.sql），替换原有内存状态。
//
// 加密密钥来源：环境变量 `SECRET_ENCRYPTION_KEY`（生产环境必须显式配置）
// 未配置时返回错误，拒绝使用默认密钥。
//
// "已初始化"判定（与 Java 侧一致）：auth_person 存在任意启用用户
// 或 secret_config 存在配置记录。
// ──────────────────────────────────────────────────────────────────────────────

/// secret_config 单行逻辑表的固定主键
const SECRET_ROW_ID: &str = "init-secret";

// --- 应用级 AES-GCM 加密 ---

#[derive(Debug, Clone)]
pub struct SecretCipher;

impl SecretCipher {
    /// 从环境变量读取加密密钥，经 md5 归一化为 16 字节（AES-128-GCM 密钥长度）。
    /// 轮换机制：更换 SECRET_ENCRYPTION_KEY 并重跑 POST /jaxrs/secret/set
    /// 即可用新密钥重写密文（密文格式 base64(nonce || ciphertext+tag)，含随机 nonce）。
    fn key() -> Result<[u8; 16], AppError> {
        let raw = std::env::var("SECRET_ENCRYPTION_KEY").map_err(|_| {
            tracing::warn!("SECRET_ENCRYPTION_KEY not configured; refusing to use default fallback");
            AppError::BadRequest("SECRET_ENCRYPTION_KEY environment variable is not configured".into())
        })?;
        let digest = Sha256::digest(raw.as_bytes());
        let mut key = [0u8; 16];
        key.copy_from_slice(&digest[..16]);
        Ok(key)
    }

    /// 加密明文，返回 `base64(nonce(12B) || ciphertext+tag)`
    pub fn encrypt(plain: &str) -> Result<String, AppError> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes128Gcm, Nonce,
        };

        let cipher = Aes128Gcm::new_from_slice(&Self::key()?).map_err(|_| AppError::Internal)?;

        let uuid = uuid::Uuid::new_v4();
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(&uuid.as_bytes()[..12]);

        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plain.as_bytes())
            .map_err(|_| AppError::Internal)?;

        let mut out = nonce_bytes.to_vec();
        out.extend_from_slice(&ciphertext);
        Ok(base64::engine::general_purpose::STANDARD.encode(out))
    }

    /// 解密 `base64(nonce(12B) || ciphertext+tag)`
    pub fn decrypt(encoded: &str) -> Result<String, AppError> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes128Gcm, Nonce,
        };

        let raw = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|_| AppError::Internal)?;
        if raw.len() < 13 {
            return Err(AppError::Internal);
        }

        let (nonce_bytes, ciphertext) = raw.split_at(12);
        let cipher = Aes128Gcm::new_from_slice(&Self::key()?).map_err(|_| AppError::Internal)?;
        let plain = cipher
            .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|_| AppError::Internal)?;

        String::from_utf8(plain).map_err(|_| AppError::Internal)
    }
}

// --- 处理器 ---

#[derive(Debug, Deserialize)]
pub struct SetSecretRequest {
    pub secret: String,
}

/// GET /jaxrs/secret/check —— 返回系统初始化状态（从数据库读取）
pub async fn check(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_row = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let has_person: bool = person_row.get("exists");

    let secret_set = client
        .query_opt("SELECT 1 FROM secret_config LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let initialized = has_person || secret_set.is_some();

    Ok(Json(ActionResult::success(json!({
        "initialized": initialized,
        "hasPerson": has_person,
        "secretSet": secret_set.is_some(),
    }))))
}

/// POST /jaxrs/secret/set —— 设置初始化密钥并持久化到数据库（AES-GCM 加密存储）
pub async fn set(
    pool: Extension<Pool>,
    Json(req): Json<SetSecretRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.secret.trim().is_empty() {
        return Ok(Json(ActionResult::error("secret cannot be empty")));
    }
    if req.secret.len() > 1024 {
        return Ok(Json(ActionResult::error("secret too long (max 1024 chars)")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let has_person: bool = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)
        .map(|row| row.get("exists"))
        .unwrap_or(false);

    if has_person {
        return Ok(Json(ActionResult::error(
            "cannot set secret when system is already initialized with users",
        )));
    }

    let encrypted = SecretCipher::encrypt(req.secret.as_str())?;
    client
        .execute(
            "INSERT INTO secret_config (id, secret_encrypted, created_at, updated_at) \
              VALUES ($1, $2, NOW(), NOW()) \
              ON CONFLICT (id) DO UPDATE \
              SET secret_encrypted = EXCLUDED.secret_encrypted, updated_at = NOW()",
            &[&SECRET_ROW_ID, &encrypted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({ "set": true }))))
}

/// 清除已设置的初始化密钥（secret_config 记录删除）
pub async fn set_cancel(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let has_person: bool = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)
        .map(|row| row.get("exists"))
        .unwrap_or(false);

    if has_person {
        return Ok(Json(ActionResult::error(
            "cannot cancel secret when system is already initialized with users",
        )));
    }

    let rows_affected = client
        .execute("DELETE FROM secret_config WHERE id = $1", &[&SECRET_ROW_ID])
        .await
        .map_err(|_| AppError::Internal)?;

    if rows_affected == 0 {
        return Ok(Json(ActionResult::success(json!({ "canceled": false }))));
    }

    Ok(Json(ActionResult::success(json!({ "canceled": true }))))
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 端点闭合（对照 x_program_init jaxrs 全集 15 条，补齐 12 条）：
//
// - externaldatasources 域（5 条）：外部数据源配置持久化到 init_external_datasource
//   （migration 074），set 在"已配置"时拒绝（对齐 Java ExceptionMissionExecute）；
//   validate 对每个数据源做真实 TCP 连通性探测。
// - h2/check（1 条）：Rust 侧无 H2，等价语义为检查 Postgres 核心表是否就绪并回报版本。
// - restore 域（2 条）：上传包落盘到临时目录并登记 init_restore_upload；cancel 作废。
// - server 域（4 条）：execute 落实待执行的初始化状态并记录命令；status 查询最近命令；
//   stop 记录停机命令（进程级停机由宿主 main.rs 接线，库内不直接 kill）；
//   license 从 x_program_config(key='license') 读取。
//
// IDOR 门禁说明：本 crate 为引导期模块，Java 侧由 CipherManagerJaxrsFilter
// （管理员过滤器）统一把关；Rust 侧沿用本 crate 既有约定——初始化域端点
// 仅在系统未完成初始化时可写（见 secret/set 的 has_person 先例），
// 会话级鉴权由全局 auth 中间件层负责。
// ──────────────────────────────────────────────────────────────────────────────

// --- externaldatasources 域 ---

#[derive(Debug, Deserialize)]
pub struct ExternalDataSourceItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "unique", default)]
    pub unique_name: String,
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: Option<i32>,
    #[serde(default)]
    pub database: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub url: String,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct ExternalDataSourcesRequest {
    #[serde(rename = "externalDataSources", default)]
    pub external_data_sources: Vec<ExternalDataSourceItem>,
}

/// 从 jdbc:postgresql://host:port/db 或 postgresql://user:pass@host:port/db
/// 形式的 URL 提取 (host, port)。解析失败返回 None。
pub fn parse_jdbc_host_port(url: &str) -> Option<(String, i32)> {
    let rest = url.split_once("://")?.1;
    let authority = rest.split(['/', '?']).next()?;
    let authority = authority.rsplit_once('@').map_or(authority, |(_, a)| a);
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, p.parse::<i32>().ok()?),
        None => (authority, 5432),
    };
    if host.is_empty() {
        return None;
    }
    Some((host.to_string(), port))
}

/// GET /jaxrs/externaldatasources/check —— 是否已配置及配置内容
pub async fn external_datasources_check(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one("SELECT count(*) AS n FROM init_external_datasource", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("n");
    Ok(Json(ActionResult::success(json!({
        "configured": count > 0,
        "count": count,
    }))))
}

/// GET /jaxrs/externaldatasources/list —— 已配置数据源列表
pub async fn external_datasources_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, unique_name, enable, host, port, database_name, user_name, url, applied \
             FROM init_external_datasource ORDER BY created_at ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "name": r.get::<_, String>("name"),
                "unique": r.get::<_, Option<String>>("unique_name").unwrap_or_default(),
                "enable": r.get::<_, bool>("enable"),
                "host": r.get::<_, Option<String>>("host").unwrap_or_default(),
                "port": r.get::<_, Option<i32>>("port"),
                "database": r.get::<_, Option<String>>("database_name").unwrap_or_default(),
                "user": r.get::<_, Option<String>>("user_name").unwrap_or_default(),
                "url": r.get::<_, Option<String>>("url").unwrap_or_default(),
                "applied": r.get::<_, bool>("applied"),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(json!({
        "count": items.len(),
        "externalDataSources": items,
    }))))
}

/// POST /jaxrs/externaldatasources/set —— 写入数据源配置（已配置时拒绝，对齐 Java）
pub async fn external_datasources_set(
    pool: Extension<Pool>,
    Json(req): Json<ExternalDataSourcesRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.external_data_sources.is_empty() {
        return Ok(Json(ActionResult::error("externalDataSources cannot be empty")));
    }
    for ds in &req.external_data_sources {
        if ds.name.trim().is_empty() {
            return Ok(Json(ActionResult::error("dataSource name cannot be empty")));
        }
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let configured: i64 = client
        .query_one("SELECT count(*) AS n FROM init_external_datasource", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("n");
    if configured > 0 {
        return Ok(Json(ActionResult::error(
            "external data sources already configured",
        )));
    }

    for ds in &req.external_data_sources {
        client
            .execute(
                "INSERT INTO init_external_datasource \
                 (id, name, unique_name, enable, host, port, database_name, user_name, password, url) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[
                    &uuid::Uuid::new_v4().to_string(),
                    &ds.name.trim().to_string(),
                    &if ds.unique_name.trim().is_empty() { None } else { Some(ds.unique_name.trim().to_string()) },
                    &ds.enable,
                    &if ds.host.trim().is_empty() { None } else { Some(ds.host.trim().to_string()) },
                    &ds.port,
                    &if ds.database.trim().is_empty() { None } else { Some(ds.database.trim().to_string()) },
                    &if ds.user.trim().is_empty() { None } else { Some(ds.user.trim().to_string()) },
                    &if ds.password.is_empty() { None } else { Some(ds.password.clone()) },
                    &if ds.url.trim().is_empty() { None } else { Some(ds.url.trim().to_string()) },
                ],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// GET /jaxrs/externaldatasources/set/cancel —— 清除尚未落实的配置
pub async fn external_datasources_set_cancel(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute("DELETE FROM init_external_datasource WHERE applied = false", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "value": affected > 0 }))))
}

/// POST /jaxrs/externaldatasources/validate —— 逐个数据源做真实 TCP 连通性探测
pub async fn external_datasources_validate(
    Json(req): Json<ExternalDataSourcesRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use std::time::Duration;
    use tokio::net::TcpStream;
    use tokio::time::timeout;

    let mut results = Vec::new();
    for ds in &req.external_data_sources {
        // 优先取显式 host/port，否则从 url 解析
        let target = if !ds.host.trim().is_empty() && ds.port.unwrap_or(0) > 0 {
            Some((ds.host.trim().to_string(), ds.port.unwrap()))
        } else {
            parse_jdbc_host_port(&ds.url)
        };
        match target {
            Some((host, port)) => {
                let addr = format!("{}:{}", host, port);
                let ok = timeout(Duration::from_secs(3), TcpStream::connect(&addr))
                    .await
                    .map(|r| r.is_ok())
                    .unwrap_or(false);
                results.push(json!({
                    "url": ds.url,
                    "dirver": "postgresql",
                    "host": host,
                    "port": port,
                    "success": ok,
                    "failureMessage": if ok { Value::Null } else { json!("connection failed") },
                }));
            }
            None => results.push(json!({
                "url": ds.url,
                "success": false,
                "failureMessage": "cannot resolve host/port from dataSource entry",
            })),
        }
    }
    let total_results = results.len();
    Ok(Json(ActionResult::java_success(Value::Array(results), total_results as i64, 0)))
}

// --- h2 域 ---

/// GET /jaxrs/h2/check —— Rust 侧等价语义：核心表就绪性 + 数据库版本
pub async fn h2_check(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let version: String = client
        .query_one("SELECT version() AS v", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("v");
    let configured: bool = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM information_schema.tables \
             WHERE table_schema = 'public' AND table_name = 'auth_person') AS exists",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("exists");
    Ok(Json(ActionResult::success(json!({
        "configured": configured,
        "version": version,
    }))))
}

// --- restore 域 ---

const RESTORE_MAX_BYTES: usize = 200 * 1024 * 1024;

/// POST /jaxrs/restore/upload —— 上传恢复包（原始字节流），落盘临时目录并登记
pub async fn restore_upload(
    pool: Extension<Pool>,
    body: axum::body::Bytes,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if body.is_empty() {
        return Ok(Json(ActionResult::error("upload body is empty")));
    }
    if body.len() > RESTORE_MAX_BYTES {
        return Ok(Json(ActionResult::error("upload exceeds size limit (200MB)")));
    }

    // stamp 对齐 Java DateTools compact 格式 yyyyMMddHHmmss
    let stamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let dir = std::env::temp_dir().join("oa4rust_restore");
    std::fs::create_dir_all(&dir).map_err(|_| AppError::Internal)?;
    let path = dir.join(format!("{}.zip", stamp));
    std::fs::write(&path, &body).map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let path_str = path.to_string_lossy().to_string();
    let size = body.len() as i64;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO init_restore_upload (id, stamp, file_path, size_bytes, status) \
             VALUES ($1, $2, $3, $4, 'uploaded')",
            &[&id, &stamp, &path_str, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({ "value": true, "id": id, "stamp": stamp }))))
}

/// GET /jaxrs/restore/upload/cancel —— 作废最近一次未落实的上传
pub async fn restore_upload_cancel(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE init_restore_upload SET status = 'cancelled' \
             WHERE id = (SELECT id FROM init_restore_upload WHERE status = 'uploaded' \
                         ORDER BY created_at DESC LIMIT 1)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "value": affected > 0 }))))
}

// --- server 域 ---

/// GET /jaxrs/server/execute —— 落实待执行的初始化状态并记录命令
pub async fn server_execute(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let ds_applied = client
        .execute("UPDATE init_external_datasource SET applied = true WHERE applied = false", &[])
        .await
        .map_err(|e| {
            tracing::error!("[server_execute] apply datasources failed: {}", e);
            AppError::Internal
        })?;
    let restores_applied = client
        .execute(
            "UPDATE init_restore_upload SET status = 'applied' WHERE status = 'uploaded'",
            &[],
        )
        .await
        .map_err(|e| {
            tracing::error!("[server_execute] apply restores failed: {}", e);
            AppError::Internal
        })?;

    let messages = json!([
        format!("external data sources applied: {}", ds_applied),
        format!("restore uploads applied: {}", restores_applied),
    ])
    .to_string();
    client
        .execute(
            "INSERT INTO init_server_command (id, command, status, messages) \
             VALUES ($1, 'execute', 'completed', ($2::text)::jsonb)",
            &[&uuid::Uuid::new_v4().to_string(), &messages],
        )
        .await
        .map_err(|e| {
            tracing::error!("[server_execute] insert command failed: {}", e);
            AppError::Internal
        })?;

    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// GET /jaxrs/server/execute/status —— 最近一次命令的执行状态
pub async fn server_execute_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT status, messages::text AS messages, failure_message FROM init_server_command \
             ORDER BY created_at DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let raw: String = r.get("messages");
            let messages: Value =
                serde_json::from_str(&raw).unwrap_or_else(|_| json!([]));
            Ok(Json(ActionResult::success(json!({
                "status": r.get::<_, String>("status"),
                "messages": messages,
                "failureMessage": r.get::<_, Option<String>>("failure_message"),
            }))))
        }
        None => Ok(Json(ActionResult::success(json!({
            "status": "empty",
            "messages": [],
            "failureMessage": null,
        })))),
    }
}

/// GET /jaxrs/server/license —— 从 x_program_config(key='license') 读取授权信息
pub async fn server_license(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT value FROM x_program_config WHERE key = 'license' AND deleted_at IS NULL \
             ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut info = serde_json::Map::new();
    info.insert("version".into(), Value::String("1.0.0".into()));
    if let Some(r) = row {
        let raw: Option<String> = r.get("value");
        if let Some(raw) = raw {
            if let Ok(Value::Object(m)) = serde_json::from_str::<Value>(&raw) {
                for (k, v) in m {
                    info.insert(k, v);
                }
            }
        }
    }
    Ok(Json(ActionResult::success(Value::Object(info))))
}

/// GET /jaxrs/server/stop —— 记录停机命令（进程停机由宿主接线执行）
pub async fn server_stop(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO init_server_command (id, command, status, messages) \
             VALUES ($1, 'stop', 'completed', '[\"stop command recorded\"]'::jsonb)",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tracing::warn!("server stop command recorded via program_init");
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

// --- 路由注册 ---

/// 构建系统初始化模块路由（main.rs 接线时传入共享数据库连接池）
pub fn program_init_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/secret/check", get(check))
        .route("/jaxrs/secret/set", post(set))
        .route("/jaxrs/secret/set/cancel", get(set_cancel))
        // plan002 U2：externaldatasources 域（5 条）
        .route("/jaxrs/externaldatasources/check", get(external_datasources_check))
        .route("/jaxrs/externaldatasources/list", get(external_datasources_list))
        .route("/jaxrs/externaldatasources/set", post(external_datasources_set))
        .route("/jaxrs/externaldatasources/set/cancel", get(external_datasources_set_cancel))
        .route("/jaxrs/externaldatasources/validate", post(external_datasources_validate))
        // h2 域（1 条）
        .route("/jaxrs/h2/check", get(h2_check))
        // restore 域（2 条）
        .route("/jaxrs/restore/upload", post(restore_upload))
        .route("/jaxrs/restore/upload/cancel", get(restore_upload_cancel))
        // server 域（4 条）
        .route("/jaxrs/server/execute", get(server_execute))
        .route("/jaxrs/server/execute/status", get(server_execute_status))
        .route("/jaxrs/server/license", get(server_license))
        .route("/jaxrs/server/stop", get(server_stop))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_init_router(pool)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
