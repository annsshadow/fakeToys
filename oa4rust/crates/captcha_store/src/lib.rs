use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use thiserror::Error;
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────────────────────
// captcha_store — 验证码生成与校验库
// ──────────────────────────────────────────────────────────────────────────────

const TTL_MINUTES: i64 = 5;
const MAX_ATTEMPTS: u32 = 5;
const DEFAULT_WIDTH: u32 = 120;
const DEFAULT_HEIGHT: u32 = 40;
const MAX_WIDTH: u32 = 500;
const MAX_HEIGHT: u32 = 200;

#[derive(Debug, Error)]
pub enum CaptchaError {
    #[error("captcha not found")]
    NotFound,
    #[error("captcha expired")]
    Expired,
    #[error("wrong answer")]
    WrongAnswer,
    #[error("too many attempts")]
    TooManyAttempts,
}

#[derive(Debug, Clone)]
pub struct CaptchaEntry {
    pub answer: String,
    pub expires_at: DateTime<Utc>,
    pub attempts: u32,
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

    /// 生成随机 captchaId 并存储答案，返回 captchaId
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

    /// 检查 captchaId 是否存在且未过期
    pub fn exists(&self, id: &str) -> bool {
        let Ok(map) = self.entries.lock() else {
            return false;
        };
        let Some(entry) = map.get(id) else {
            return false;
        };
        entry.expires_at > Utc::now()
    }

    /// 清理过期条目，返回清理数量
    pub fn cleanup_count(&self) -> usize {
        let now = Utc::now();
        let Ok(mut map) = self.entries.lock() else {
            return 0;
        };
        let before = map.len();
        map.retain(|_, e| e.expires_at > now);
        before - map.len()
    }
}

impl CaptchaStore {
    /// 仅测试用：强制插入带有自定义过期时间的条目
    pub fn force_insert(&self, id: &str, answer: &str, expires_at: DateTime<Utc>) {
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                id.to_string(),
                CaptchaEntry {
                    answer: answer.to_string(),
                    expires_at,
                    attempts: 0,
                },
            );
        }
    }
}

/// 全局验证码存储单例
pub fn captcha_store() -> &'static CaptchaStore {
    static STORE: OnceLock<CaptchaStore> = OnceLock::new();
    STORE.get_or_init(CaptchaStore::new)
}

// ──────────────────────────────────────────────────────────────────────────────
// PNG 渲染（使用 captcha workspace crate）
// ──────────────────────────────────────────────────────────────────────────────

/// 渲染验证码 PNG，返回 (答案, PNG 字节)
pub fn render_png(width: u32, height: u32) -> Result<(String, Vec<u8>), CaptchaError> {
    if width == 0 || height == 0 || width > MAX_WIDTH || height > MAX_HEIGHT {
        return Err(CaptchaError::NotFound);
    }
    let mut c = captcha::Captcha::new();
    c.add_chars(4)
        .apply_filter(captcha::filters::Grid::new(20, 20))
        .view(width, height);
    c.as_tuple()
        .ok_or(CaptchaError::NotFound)
        .map(|(chars, png)| (chars, png))
}

/// 生成验证码并存储，返回 (captcha_id, PNG 字节)
pub fn generate(width: u32, height: u32) -> Result<(String, Vec<u8>), CaptchaError> {
    let (answer, png) = render_png(width, height)?;
    let id = captcha_store().insert(answer);
    Ok((id, png))
}

/// 校验验证码
pub fn verify(id: &str, answer: &str) -> VerifyResult {
    captcha_store().verify(id, answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_verify_ok() {
        let store = CaptchaStore::new();
        let id = store.insert("Ab3D".to_string());
        assert_eq!(store.verify(&id, "ab3d"), VerifyResult::Ok);
        assert_eq!(store.verify(&id, "ab3d"), VerifyResult::NotFound);
    }

    #[test]
    fn test_verify_wrong_answer() {
        let store = CaptchaStore::new();
        let id = store.insert("1234".to_string());
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer); // attempts=1
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer); // attempts=2
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer); // attempts=3
        assert_eq!(store.verify(&id, "0000"), VerifyResult::WrongAnswer); // attempts=4
        // 5th wrong attempt triggers TooManyAttempts (attempts=5 >= MAX_ATTEMPTS=5)
        assert_eq!(store.verify(&id, "0000"), VerifyResult::TooManyAttempts);
        assert_eq!(store.verify(&id, "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_verify_unknown_id() {
        let store = CaptchaStore::new();
        assert_eq!(store.verify("nonexistent", "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_verify_expired() {
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
    fn test_exists() {
        let store = CaptchaStore::new();
        let id = store.insert("abcd".to_string());
        assert!(store.exists(&id));
        store.verify(&id, "abcd");
        assert!(!store.exists(&id));
    }

    #[test]
    fn test_cleanup() {
        let store = CaptchaStore::new();
        let old_id = "old-id".to_string();
        if let Ok(mut map) = store.entries.lock() {
            map.insert(
                old_id.clone(),
                CaptchaEntry {
                    answer: "1234".to_string(),
                    expires_at: Utc::now() - Duration::minutes(1),
                    attempts: 0,
                },
            );
        }
        let count = store.cleanup_count();
        assert!(count > 0);
    }

    #[test]
    fn test_generate_valid_dimensions() {
        let (id, png) = generate(120, 40).unwrap();
        assert!(!id.is_empty());
        assert!(!png.is_empty());
    }

    #[test]
    fn test_generate_invalid_dimensions() {
        assert!(generate(0, 40).is_err());
        assert!(generate(120, 0).is_err());
        assert!(generate(501, 40).is_err());
        assert!(generate(120, 201).is_err());
    }
}
