use std::env;
use std::time::Duration;
use tracing::{error, info, warn};

/// LDAP 配置，从环境变量读取。
/// 所有环境变量均可选；LDAP_ENABLE 默认 false。
#[derive(Debug, Clone)]
pub struct LdapConfig {
    pub url: String,
    pub base_dn: String,
    pub bind_user: String,
    pub bind_password: String,
}

impl LdapConfig {
    /// 从环境变量加载 LDAP 配置。
    /// 仅当 LDAP_ENABLE=true 且 LDAP_URL 非空时返回 Some。
    pub fn from_env() -> Option<Self> {
        let enable = env::var("LDAP_ENABLE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase();

        if enable != "true" {
            return None;
        }

        let url = env::var("LDAP_URL").ok()?;
        if url.is_empty() {
            warn!("LDAP_ENABLE=true but LDAP_URL is empty, LDAP disabled");
            return None;
        }

        Some(Self {
            url,
            base_dn: env::var("LDAP_BASE_DN").unwrap_or_default(),
            bind_user: env::var("LDAP_BIND_USER").unwrap_or_default(),
            bind_password: env::var("LDAP_BIND_PWD").unwrap_or_default(),
        })
    }

    /// 检查 LDAP 是否已启用且配置完整
    pub fn is_enabled(&self) -> bool {
        !self.url.is_empty()
    }
}

/// LDAP 认证结果
#[derive(Debug, PartialEq, Eq)]
pub enum LdapAuthResult {
    /// LDAP 认证成功
    Success,
    /// LDAP 认证失败（凭据无效）
    Failed,
    /// LDAP 未启用或配置不完整
    Disabled,
    /// LDAP 连接/绑定出错（超时、网络等）
    Error(String),
}

/// LDAP 认证器：尝试通过 LDAP simple bind 验证凭据。
/// 若 LDAP 未启用或连接失败，返回相应结果供调用方回退到数据库校验。
pub struct LdapAuthenticator {
    config: LdapConfig,
}

impl LdapAuthenticator {
    pub fn new(config: LdapConfig) -> Self {
        Self { config }
    }

    /// 尝试 LDAP 认证。
    ///
    /// - 返回 `Success`：LDAP bind 成功
    /// - 返回 `Failed`：LDAP bind 失败（密码错误等）
    /// - 返回 `Error(...)`：连接超时或网络错误
    pub async fn authenticate(&self, username: &str, password: &str) -> LdapAuthResult {
        let base_dn = &self.config.base_dn;

        let bind_dn = if base_dn.is_empty() {
            format!("uid={},{}", username, "ou=users,dc=example,dc=com")
        } else {
            format!("uid={},{}", username, base_dn)
        };

        let result = tokio::time::timeout(
            Duration::from_secs(3),
            self.try_bind(&bind_dn, password),
        )
        .await;

        match result {
            Ok(Ok(())) => {
                info!("LDAP auth succeeded for user: {}", username);
                LdapAuthResult::Success
            }
            Ok(Err(e)) => {
                // 凭据无效（密码错误、账号不存在等）— 应回退到 DB 校验
                warn!("LDAP bind failed for user {}: {}", username, e);
                LdapAuthResult::Failed
            }
            Err(_) => {
                // 连接超时或网络错误 — 不应静默回退，应告警
                error!("LDAP connection error for user {}: service unavailable", username);
                LdapAuthResult::Error("connection timeout".to_string())
            }
        }
    }

    async fn try_bind(&self, bind_dn: &str, password: &str) -> Result<(), String> {
        use ldap3::LdapConnAsync;

        let (conn, mut ldap) = LdapConnAsync::new(&self.config.url)
            .await
            .map_err(|e| format!("failed to connect: {e}"))?;

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

        ldap.simple_bind(bind_dn, password)
            .await
            .map_err(|e| format!("user bind failed: {e}"))?
            .success()
            .map_err(|e| format!("user bind failed: {e}"))?;

        ldap.unbind().await.ok();
        Ok(())
    }
}

/// 便利函数：根据环境变量创建 LDAP 认证器（如果已启用）。
pub fn authenticator_from_env() -> Option<LdapAuthenticator> {
    LdapConfig::from_env().map(LdapAuthenticator::new)
}

/// LDAP 用户自动同步（plan002 U7b，env 门控）
pub mod sync;

#[cfg(test)]
mod tests_generated;
