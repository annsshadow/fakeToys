use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use thiserror::Error;
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────────────────────
// sms — 短信网关抽象
//
// 提供统一的短信发送 trait，内置 Mock 实现用于测试。
// 生产环境可接入阿里云、腾讯云、华为云等 SMS 网关。
// ──────────────────────────────────────────────────────────────────────────────

/// 短信发送错误
#[derive(Debug, Error)]
pub enum SmsError {
    #[error("invalid phone number: {0}")]
    InvalidPhone(String),
    #[error("gateway error: {0}")]
    Gateway(String),
    #[error("rate limited")]
    RateLimited,
    #[error("network error")]
    Network,
}

/// 短信发送结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsResult {
    pub message_id: String,
    pub phone: String,
    pub sent_at: String,
    pub status: SmsStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SmsStatus {
    Sent,
    Failed,
    Pending,
}

impl SmsStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SmsStatus::Sent => "sent",
            SmsStatus::Failed => "failed",
            SmsStatus::Pending => "pending",
        }
    }
}

/// 短信网关 trait
///
/// 实现者负责将短信实际投递给运营商网关。
/// 签名/模板等参数按具体网关约定传递。
#[async_trait]
pub trait SmsGateway: Send + Sync {
    /// 发送验证码短信
    async fn send_verification_code(
        &self,
        phone: &str,
        code: &str,
        template_id: Option<&str>,
    ) -> Result<SmsResult, SmsError>;

    /// 发送普通通知短信
    async fn send_notification(
        &self,
        phone: &str,
        content: &str,
    ) -> Result<SmsResult, SmsError>;

    /// 网关名称（用于日志和监控）
    fn name(&self) -> &'static str;
}

// ──────────────────────────────────────────────────────────────────────────────
// Mock 网关（测试用）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct MockSmsGateway {
    pub sent_messages: Mutex<Vec<SmsResult>>,
    pub should_fail: Mutex<bool>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Console 网关（开发/调试用：输出到标准错误）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct ConsoleSmsGateway;

#[async_trait]
impl SmsGateway for ConsoleSmsGateway {
    async fn send_verification_code(
        &self,
        phone: &str,
        code: &str,
        template_id: Option<&str>,
    ) -> Result<SmsResult, SmsError> {
        let result = SmsResult {
            message_id: Uuid::new_v4().to_string(),
            phone: phone.to_string(),
            sent_at: chrono::Utc::now().to_rfc3339(),
            status: SmsStatus::Sent,
        };
        eprintln!(
            "[SMS:console] to={} code={} template_id={:?} status=sent",
            phone, code, template_id
        );
        Ok(result)
    }

    async fn send_notification(
        &self,
        phone: &str,
        content: &str,
    ) -> Result<SmsResult, SmsError> {
        let result = SmsResult {
            message_id: Uuid::new_v4().to_string(),
            phone: phone.to_string(),
            sent_at: chrono::Utc::now().to_rfc3339(),
            status: SmsStatus::Sent,
        };
        eprintln!(
            "[SMS:console] to={} content={} status=sent",
            phone, content
        );
        Ok(result)
    }

    fn name(&self) -> &'static str {
        "console"
    }
}

#[async_trait]
impl SmsGateway for MockSmsGateway {
    async fn send_verification_code(
        &self,
        phone: &str,
        _code: &str,
        _template_id: Option<&str>,
    ) -> Result<SmsResult, SmsError> {
        if *self.should_fail.lock().unwrap() {
            return Err(SmsError::Gateway("mock failure".to_string()));
        }
        let result = SmsResult {
            message_id: Uuid::new_v4().to_string(),
            phone: phone.to_string(),
            sent_at: chrono::Utc::now().to_rfc3339(),
            status: SmsStatus::Sent,
        };
        self.sent_messages.lock().unwrap().push(result.clone());
        Ok(result)
    }

    async fn send_notification(
        &self,
        phone: &str,
        _content: &str,
    ) -> Result<SmsResult, SmsError> {
        if *self.should_fail.lock().unwrap() {
            return Err(SmsError::Gateway("mock failure".to_string()));
        }
        let result = SmsResult {
            message_id: Uuid::new_v4().to_string(),
            phone: phone.to_string(),
            sent_at: chrono::Utc::now().to_rfc3339(),
            status: SmsStatus::Sent,
        };
        self.sent_messages.lock().unwrap().push(result.clone());
        Ok(result)
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

impl MockSmsGateway {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置网关是否模拟失败
    pub fn set_fail(&self, fail: bool) {
        *self.should_fail.lock().unwrap() = fail;
    }

    /// 重置已发送记录
    pub fn reset(&self) {
        self.sent_messages.lock().unwrap().clear();
    }

    /// 获取发送记录数量
    pub fn sent_count(&self) -> usize {
        self.sent_messages.lock().unwrap().len()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 短信发送器（组合 gateway + 限流）
// ──────────────────────────────────────────────────────────────────────────────

const SMS_COOLDOWN_SECONDS: i64 = 60;
const SMS_RATE_LIMIT_PER_HOUR: usize = 10;

pub struct SmsSender {
    gateway: Box<dyn SmsGateway>,
    rate_limiter: Mutex<HashMap<String, Vec<i64>>>,
}

impl SmsSender {
    pub fn new(gateway: Box<dyn SmsGateway>) -> Self {
        Self {
            gateway,
            rate_limiter: Mutex::new(HashMap::new()),
        }
    }

    /// 发送验证码短信（含限流）
    pub async fn send_code(&self, phone: &str, code: &str) -> Result<SmsResult, SmsError> {
        if !is_valid_phone(phone) {
            return Err(SmsError::InvalidPhone(phone.to_string()));
        }
        self.check_rate_limit(phone)?;
        self.gateway.send_verification_code(phone, code, None).await
    }

    fn check_rate_limit(&self, phone: &str) -> Result<(), SmsError> {
        let now = chrono::Utc::now().timestamp();
        let Ok(mut limiter) = self.rate_limiter.lock() else {
            return Ok(());
        };
        let timestamps = limiter.entry(phone.to_string()).or_default();
        // 清理 1 小时前的记录
        timestamps.retain(|&t| now - t < 3600);
        // 检查 1 小时内是否超过上限
        if timestamps.len() >= SMS_RATE_LIMIT_PER_HOUR {
            return Err(SmsError::RateLimited);
        }
        // 检查冷却时间（60 秒内只能发一次）
        if let Some(&last) = timestamps.last() {
            if now - last < SMS_COOLDOWN_SECONDS {
                return Err(SmsError::RateLimited);
            }
        }
        timestamps.push(now);
        Ok(())
    }
}

/// 验证手机号格式（中国大陆手机号）
pub fn is_valid_phone(phone: &str) -> bool {
    phone.len() == 11
        && phone.starts_with('1')
        && phone.chars().all(|c| c.is_ascii_digit())
}

// ──────────────────────────────────────────────────────────────────────────────
// 全局 SMS 发送器单例（测试用）
// ──────────────────────────────────────────────────────────────────────────────

static MOCK_SENDER: OnceLock<MockSmsGateway> = OnceLock::new();

pub fn mock_sms_gateway() -> &'static MockSmsGateway {
    MOCK_SENDER.get_or_init(MockSmsGateway::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_gateway_send() {
        let gateway = MockSmsGateway::new();
        let result = gateway.send_verification_code("13800138000", "123456", None).await;
        assert!(result.is_ok());
        assert_eq!(gateway.sent_count(), 1);
        let r = result.unwrap();
        assert_eq!(r.phone, "13800138000");
        assert_eq!(r.status, SmsStatus::Sent);
    }

    #[tokio::test]
    async fn test_mock_gateway_failure() {
        let gateway = MockSmsGateway::new();
        gateway.set_fail(true);
        let result = gateway.send_verification_code("13800138000", "123456", None).await;
        assert!(result.is_err());
        assert_eq!(gateway.sent_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_gateway_reset() {
        let gateway = MockSmsGateway::new();
        let _ = gateway.send_verification_code("13800138000", "123456", None).await;
        assert_eq!(gateway.sent_count(), 1);
        gateway.reset();
        assert_eq!(gateway.sent_count(), 0);
    }

    #[test]
    fn test_is_valid_phone() {
        assert!(is_valid_phone("13800138000"));
        assert!(is_valid_phone("15912345678"));
        // Starts with 1 and 11 digits = valid format (even if area code is fake)
        assert!(is_valid_phone("12345678901"));
        assert!(!is_valid_phone("23800138000"));
        assert!(!is_valid_phone("1380013800"));
        assert!(!is_valid_phone("138001380000"));
        assert!(!is_valid_phone("13800138abc"));
    }

    #[tokio::test]
    async fn test_sms_sender_rate_limit() {
        let gateway = MockSmsGateway::new();
        let sender = SmsSender::new(Box::new(gateway));
        // First send should succeed
        let r1 = sender.send_code("13800138000", "123456").await;
        assert!(r1.is_ok());
        // Immediate second send should fail (cooldown)
        let r2 = sender.send_code("13800138000", "123456").await;
        assert!(r2.is_err());
    }

    #[tokio::test]
    async fn test_sms_sender_invalid_phone() {
        let gateway = MockSmsGateway::new();
        let sender = SmsSender::new(Box::new(gateway));
        let result = sender.send_code("invalid", "123456").await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_generated;
