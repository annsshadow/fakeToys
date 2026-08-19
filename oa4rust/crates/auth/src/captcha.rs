use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────────────────────
// captcha — 验证码
//
// 本地生成 PNG 验证码（captcha crate），答案以 captchaId 为键存进程内存储：
// 5 分钟有效、一次性（校验通过即删除）、尝试次数上限（超过即删除）。
// 与 SessionManager 一致采用进程内方案（重启即失效）。
// ──────────────────────────────────────────────────────────────────────────────

const TTL_MINUTES: i64 = 5;
const MAX_ATTEMPTS: u32 = 5;
const DEFAULT_WIDTH: u32 = 120;
const DEFAULT_HEIGHT: u32 = 40;
const MAX_WIDTH: u32 = 500;
const MAX_HEIGHT: u32 = 200;

#[derive(Debug, Clone)]
struct CaptchaEntry {
    answer: String,
    expires_at: DateTime<Utc>,
    attempts: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerifyResult {
    Ok,
    NotFound,
    Expired,
    WrongAnswer,
    TooManyAttempts,
}

/// 验证码存储（可独立构造用于测试；运行时使用全局单例）
pub struct CaptchaStore {
    entries: Mutex<HashMap<String, CaptchaEntry>>,
}

impl Default for CaptchaStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptchaStore {
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

    /// 生成随机 captchaId 并存储答案
    pub fn insert(&self, answer: String) -> String {
        self.cleanup();
        let id = Uuid::new_v4().to_string();
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                id.clone(),
                CaptchaEntry {
                    answer,
                    expires_at: Utc::now() + Duration::minutes(TTL_MINUTES),
                    attempts: 0,
                },
            );
        }
        id
    }

    /// 校验答案：大小写不敏感。通过即删除（一次性）；连续错误达上限即删除。
    /// 过期条目单独判定（Expired）后删除，其余过期条目由 insert 时惰性清理。
    pub fn verify(&self, id: &str, answer: &str) -> VerifyResult {
        let Ok(mut map) = self.entries.lock() else {
            return VerifyResult::NotFound;
        };
        let Some(entry) = map.get_mut(id) else {
            return VerifyResult::NotFound;
        };
        if entry.expires_at <= Utc::now() {
            map.remove(id);
            return VerifyResult::Expired;
        }
        if entry.answer.eq_ignore_ascii_case(answer.trim()) {
            map.remove(id);
            return VerifyResult::Ok;
        }
        entry.attempts += 1;
        if entry.attempts >= MAX_ATTEMPTS {
            map.remove(id);
            return VerifyResult::TooManyAttempts;
        }
        VerifyResult::WrongAnswer
    }
}

pub fn captcha_store() -> &'static CaptchaStore {
    static STORE: OnceLock<CaptchaStore> = OnceLock::new();
    STORE.get_or_init(CaptchaStore::new)
}

/// 渲染验证码 PNG，返回 (答案, PNG 字节)
fn render_png(width: u32, height: u32) -> Result<(String, Vec<u8>), AppError> {
    let mut c = captcha::Captcha::new();
    c.add_chars(4)
        .apply_filter(captcha::filters::Grid::new(20, 20))
        .view(width, height);
    c.as_tuple().ok_or(AppError::Internal)
}

fn image_data_uri(png: &[u8]) -> String {
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(png)
    )
}

fn validate_dimensions(width: u32, height: u32) -> Result<(), AppError> {
    if width == 0 || height == 0 || width > MAX_WIDTH || height > MAX_HEIGHT {
        return Err(AppError::BadRequest(format!(
            "invalid captcha dimensions: {width}x{height}"
        )));
    }
    Ok(())
}

/// GET /jaxrs/authentication/captcha —— 默认尺寸验证码
pub async fn captcha_default() -> Result<Json<ActionResult<Value>>, AppError> {
    generate_captcha(DEFAULT_WIDTH, DEFAULT_HEIGHT)
}

/// GET /jaxrs/authentication/captcha/width/{width}/height/{height}
pub async fn captcha_with_size(
    Path((width, height)): Path<(u32, u32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    generate_captcha(width, height)
}

fn generate_captcha(width: u32, height: u32) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_dimensions(width, height)?;
    let (answer, png) = render_png(width, height)?;
    let id = captcha_store().insert(answer);
    Ok(Json(ActionResult::success(json!({
        "captchaId": id,
        "image": image_data_uri(&png),
    }))))
}

#[derive(Debug, Deserialize)]
pub struct VerifyCaptchaRequest {
    pub captchaId: String,
    pub answer: String,
}

/// POST /jaxrs/secret/captcha/verify —— 验证码校验（与初始化检查 GET /jaxrs/secret/check 区分）
pub async fn verify(
    Json(req): Json<VerifyCaptchaRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match captcha_store().verify(&req.captchaId, &req.answer) {
        VerifyResult::Ok => Ok(Json(ActionResult::success(json!({ "passed": true })))),
        VerifyResult::TooManyAttempts => Ok(Json(ActionResult::error(
            "too many attempts, captcha invalidated",
        ))),
        VerifyResult::Expired => Ok(Json(ActionResult::error("captcha expired"))),
        _ => Ok(Json(ActionResult::error("invalid captcha"))),
    }
}

pub fn captcha_router() -> Router {
    Router::new()
        .route("/jaxrs/authentication/captcha", get(captcha_default))
        .route(
            "/jaxrs/authentication/captcha/width/{width}/height/{height}",
            get(captcha_with_size),
        )
        .route("/jaxrs/secret/captcha/verify", post(verify))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha_ok_and_one_time() {
        let store = CaptchaStore::new();
        let id = store.insert("Ab3D".to_string());
        assert_eq!(store.verify(&id, "ab3d"), VerifyResult::Ok);
        assert_eq!(store.verify(&id, "ab3d"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_wrong_answer_attempts() {
        let store = CaptchaStore::new();
        let id = store.insert("1234".to_string());
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(store.verify(&id, "0000"), VerifyResult::TooManyAttempts);
        assert_eq!(store.verify(&id, "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_unknown_id() {
        let store = CaptchaStore::new();
        assert_eq!(store.verify("nope", "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_expired() {
        let store = CaptchaStore::new();
        let id = "expired-id".to_string();
        if let Ok(mut map) = store.entries.lock() {
            map.insert(
                id.clone(),
                CaptchaEntry {
                    answer: "1234".to_string(),
                    expires_at: Utc::now() - Duration::minutes(1),
                    attempts: 0,
                },
            );
        }
        assert_eq!(store.verify(&id, "1234"), VerifyResult::Expired);
    }

    #[test]
    fn test_dimension_validation() {
        assert!(validate_dimensions(120, 40).is_ok());
        assert!(validate_dimensions(0, 40).is_err());
        assert!(validate_dimensions(120, 0).is_err());
        assert!(validate_dimensions(501, 40).is_err());
        assert!(validate_dimensions(120, 201).is_err());
    }
}
