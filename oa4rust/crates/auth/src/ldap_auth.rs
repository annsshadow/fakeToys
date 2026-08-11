use ldap::{authenticator_from_env, LdapAuthResult};
use shared::error::AppError;
use tracing::warn;

/// LDAP 认证结果。
pub enum LdapAuthOutcome {
    /// LDAP 认证成功，跳过数据库校验
    Success,
    /// LDAP 认证失败（凭据无效），回退到数据库校验
    Failed,
    /// LDAP 连接错误（超时/网络），应返回服务不可用而非静默回退
    Error,
    /// LDAP 未启用
    Disabled,
}

/// 尝试 LDAP 认证。
///
/// - `Success`：LDAP bind 成功，跳过数据库密码校验
/// - `Failed`：LDAP bind 失败（密码错误），回退到数据库校验
/// - `Error`：LDAP 连接/超时错误，**不**回退到数据库（返回 AppError）
/// - `Disabled`：LDAP 未启用，回退到数据库校验
pub async fn try_ldap_authenticate(
    username: &str,
    password: &str,
) -> Result<Option<LdapAuthOutcome>, AppError> {
    let authenticator = match authenticator_from_env() {
        Some(a) => a,
        None => return Ok(None), // LDAP 未启用
    };

    match authenticator.authenticate(username, password).await {
        LdapAuthResult::Success => Ok(Some(LdapAuthOutcome::Success)),
        LdapAuthResult::Failed => {
            warn!("LDAP auth failed for user {}, falling back to DB", username);
            Ok(Some(LdapAuthOutcome::Failed))
        }
        LdapAuthResult::Error(_) => {
            // 连接错误：不静默回退到 DB，返回内部错误让调用方处理
            Err(AppError::Internal)
        }
        LdapAuthResult::Disabled => Ok(None),
    }
}
