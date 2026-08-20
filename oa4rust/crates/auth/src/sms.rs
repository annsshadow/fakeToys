use axum::{routing::post, Json, Router};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use sms::{is_valid_phone, mock_sms_gateway, SmsGateway};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SmsSendRequest {
    pub phone: String,
}

// ──────────────────────────────────────────────────────────────────────────────
// SMS 验证码存储（手机号 → 一次性验证码）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct SmsCodeEntry {
    code: String,
    expires_at: DateTime<Utc>,
    attempts: u32,
}

const SMS_CODE_TTL_MINUTES: i64 = 5;
const SMS_CODE_MAX_ATTEMPTS: u32 = 5;

struct SmsCodeStore {
    entries: Mutex<HashMap<String, SmsCodeEntry>>,
}

impl SmsCodeStore {
    fn new() -> Self {
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

    /// 为手机号生成 6 位数字验证码并存储，返回明文（发送渠道使用）
    fn issue(&self, phone: &str) -> String {
        self.cleanup();
        let code = format!("{:06}", rand_code());
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                phone.to_string(),
                SmsCodeEntry {
                    code: code.clone(),
                    expires_at: Utc::now() + Duration::minutes(SMS_CODE_TTL_MINUTES),
                    attempts: 0,
                },
            );
        }
        code
    }

    /// 校验验证码：通过删除（一次性），错误达上限删除
    fn verify(&self, phone: &str, code: &str) -> bool {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return false;
        };
        let Some(entry) = map.get_mut(phone) else {
            return false;
        };
        if entry.expires_at <= Utc::now() {
            map.remove(phone);
            return false;
        }
        if entry.code == code.trim() {
            map.remove(phone);
            return true;
        }
        entry.attempts += 1;
        if entry.attempts >= SMS_CODE_MAX_ATTEMPTS {
            map.remove(phone);
        }
        false
    }
}

fn rand_code() -> u32 {
    (Uuid::new_v4().as_u128() % 1_000_000) as u32
}

fn sms_code_store() -> &'static SmsCodeStore {
    static STORE: OnceLock<SmsCodeStore> = OnceLock::new();
    STORE.get_or_init(SmsCodeStore::new)
}

// ──────────────────────────────────────────────────────────────────────────────
// 发送与校验
// ──────────────────────────────────────────────────────────────────────────────

/// 发送短信验证码（供 lib.rs 调用）
pub async fn sms_send(phone: &str) -> Result<ActionResult<Value>, AppError> {
    if phone.is_empty() {
        return Ok(ActionResult::error("phone is required"));
    }
    if !is_valid_phone(phone) {
        return Ok(ActionResult::error("invalid phone number"));
    }
    let plain_code = sms_code_store().issue(phone);
    let gateway = mock_sms_gateway();
    let result = gateway
        .send_verification_code(phone, &plain_code, None)
        .await;
    match result {
        Ok(_) => Ok(ActionResult::success(json!({
            "message": "sms sent",
        }))),
        Err(e) => Ok(ActionResult::error(format!("sms send failed: {e}"))),
    }
}

/// POST /jaxrs/authentication/sms/send —— 发送短信验证码
pub async fn sms_send_handler(
    Json(payload): Json<SmsSendRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    sms_send(&payload.phone)
        .await
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct SmsVerifyRequest {
    pub phone: String,
    pub code: String,
}

/// POST /jaxrs/authentication/sms/verify —— 校验短信验证码
pub async fn sms_verify_handler(
    Json(req): Json<SmsVerifyRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.phone.is_empty() || req.code.is_empty() {
        return Ok(Json(ActionResult::error("phone and code are required")));
    }
    if !is_valid_phone(&req.phone) {
        return Ok(Json(ActionResult::error("invalid phone number")));
    }
    let valid = sms_code_store().verify(&req.phone, &req.code);
    if valid {
        Ok(Json(ActionResult::success(json!({
            "message": "sms code verified",
        }))))
    } else {
        Ok(Json(ActionResult::error("invalid sms code")))
    }
}

pub fn sms_router() -> Router {
    Router::new()
        .route("/jaxrs/authentication/sms/send", post(sms_send_handler))
        .route("/jaxrs/authentication/sms/verify", post(sms_verify_handler))
}
