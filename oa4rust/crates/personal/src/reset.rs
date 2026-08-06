use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use auth::password::hash_password;

// --- 重置验证码存储 ---

/// 验证码有效期（5 分钟）
pub const CODE_TTL_MINUTES: i64 = 5;
/// 验证码尝试上限
pub const MAX_ATTEMPTS: u32 = 5;
/// 验证码位数
const CODE_LENGTH: u32 = 6;

#[derive(Debug, Clone)]
pub struct ResetCodeEntry {
    pub code: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub remaining_attempts: u32,
}

#[derive(Clone, Default)]
pub struct ResetCodeStore {
    inner: Arc<RwLock<HashMap<String, ResetCodeEntry>>>,
}

/// 验证码校验失败原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetCodeError {
    NotFound,
    Expired,
    WrongCode,
    TooManyAttempts,
}

impl fmt::Display for ResetCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResetCodeError::NotFound => write!(f, "验证码不存在"),
            ResetCodeError::Expired => write!(f, "验证码已过期"),
            ResetCodeError::WrongCode => write!(f, "验证码不正确"),
            ResetCodeError::TooManyAttempts => write!(f, "验证码尝试次数过多"),
        }
    }
}

impl ResetCodeStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// 生成并保存一条验证码（覆盖该凭据的旧码），返回 6 位数字验证码。
    ///
    /// key 使用人员的唯一标识（unique_id），5 分钟有效，尝试上限 5 次。
    pub async fn issue(&self, key: &str) -> String {
        let code = new_code();
        let entry = ResetCodeEntry {
            code: code.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(CODE_TTL_MINUTES),
            remaining_attempts: MAX_ATTEMPTS,
        };
        self.inner.write().await.insert(key.to_string(), entry);
        code
    }

    /// 校验并消费验证码：校验通过立即失效（一次性）；失败按错误类型计数。
    pub async fn verify_and_consume(&self, key: &str, code: &str) -> Result<(), ResetCodeError> {
        let now = chrono::Utc::now();
        let mut map = self.inner.write().await;

        let mut entry = map.remove(key).ok_or(ResetCodeError::NotFound)?;

        if entry.expires_at <= now {
            return Err(ResetCodeError::Expired);
        }

        if entry.remaining_attempts == 0 {
            return Err(ResetCodeError::TooManyAttempts);
        }

        if entry.code != code {
            entry.remaining_attempts -= 1;
            let remaining = entry.remaining_attempts;
            if remaining > 0 {
                map.insert(key.to_string(), entry);
            }
            return if remaining == 0 {
                Err(ResetCodeError::TooManyAttempts)
            } else {
                Err(ResetCodeError::WrongCode)
            };
        }

        // 校验通过：条目已在 remove 时出栈，立即失效
        Ok(())
    }

    /// 测试辅助：写入一条已过期条目
    #[cfg(test)]
    pub async fn insert_expired(&self, key: &str, code: &str) {
        let entry = ResetCodeEntry {
            code: code.to_string(),
            expires_at: chrono::Utc::now() - chrono::Duration::minutes(1),
            remaining_attempts: MAX_ATTEMPTS,
        };
        self.inner.write().await.insert(key.to_string(), entry);
    }
}

/// 依据 UUID 字节生成 6 位数字验证码
fn new_code() -> String {
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let mut seed: u32 = 0;
    for b in bytes {
        seed = seed.wrapping_mul(31).wrapping_add(*b as u32);
    }
    format!("{:0width$}", seed % 1_000_000, width = CODE_LENGTH as usize)
}

// --- 请求 DTO ---

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub credential: String,
    pub code: String,
    pub password: String,
}

// --- 处理器（Java ResetAction 契约路径） ---

/// GET /jaxrs/reset/check/credential/{credential}
///
/// 校验凭据（unique_id）是否存在且可用。
pub async fn check_credential(
    pool: Extension<Pool>,
    Path(credential): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL)",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let registered: bool = row.get("exists");
    Ok(Json(ActionResult::success(json!({ "registered": registered }))))
}

/// GET /jaxrs/reset/check/password/{password}
///
/// 校验新密码是否符合安全规则（原型规则：长度 6 至 64）。
///
/// 注意：密码以明文形式出现在 URL path 中，此端点仅用于开发联调。
/// 生产环境应通过内部网络或防火墙限制调用来源，避免密码泄露到访问日志或代理服务器日志中。
/// 密码仅用于本地校验，不存储、不记录日志。
pub async fn check_password(
    Path(password): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let passed = is_password_acceptable(&password);
    Ok(Json(ActionResult::success(json!({ "passed": passed }))))
}

/// 新密码规则：长度 6 至 64，且至少包含一个字母和一个数字
pub fn is_password_acceptable(password: &str) -> bool {
    let len = password.chars().count();
    if !(6..=64).contains(&len) {
        return false;
    }
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());
    has_letter && has_digit
}

/// GET /jaxrs/reset/code/credential/{credential}
///
/// 校验凭据后生成一次性重置验证码（5 分钟有效、尝试上限 5 次）。
/// 原型阶段未接入短信/邮件渠道，验证码在响应中返回以便联调。
pub async fn send_code(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    Path(credential): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT 1 FROM auth_person WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if exists.is_none() {
        return Ok(Json(ActionResult::error("用户不存在或不可用")));
    }

    store.issue(&credential).await;
    Ok(Json(ActionResult::success(json!({
        "sent": true,
    }))))
}

/// PUT /jaxrs/reset —— 校验验证码并重置当前凭据密码
pub async fn reset_password(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<ResetPasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    apply_reset(pool, store, &req.credential, &req.code, &req.password).await
}

/// POST /jaxrs/reset/password/anonymous —— 匿名重置密码（校验验证码）
pub async fn reset_password_anonymous(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<ResetPasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    apply_reset(pool, store, &req.credential, &req.code, &req.password).await
}

async fn apply_reset(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    credential: &str,
    code: &str,
    password: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if !is_password_acceptable(password) {
        return Ok(Json(ActionResult::error("新密码不符合规则（长度需 6 至 64 位）")));
    }

    if let Err(e) = store.verify_and_consume(credential, code).await {
        return Ok(Json(ActionResult::error(e.to_string())));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let person_id: String = r.get("id");
            let new_hash = hash_password(password);
            client
                .execute(
                    "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                    &[&new_hash, &person_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(json!({ "success": true }))))
        }
        None => Ok(Json(ActionResult::error("用户不存在或不可用"))),
    }
}