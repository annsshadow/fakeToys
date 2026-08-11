use ldap::{authenticator_from_env, LdapAuthResult};
use tracing::warn;

/// 尝试 LDAP 认证。
///
/// 如果 LDAP 未启用，返回 None（调用方应回退到数据库校验）。
/// 如果 LDAP 认证成功，返回 Some(true)。
/// 如果 LDAP 认证失败或出错，返回 Some(false)（调用方可选择回退）。
pub async fn try_ldap_authenticate(username: &str, password: &str) -> Option<bool> {
    let authenticator = authenticator_from_env()?;

    match authenticator.authenticate(username, password).await {
        LdapAuthResult::Success => Some(true),
        LdapAuthResult::Failed => {
            warn!("LDAP auth failed for user {}, falling back to DB", username);
            Some(false)
        }
        LdapAuthResult::Error(e) => {
            warn!(
                "LDAP auth error for user {}: {}, falling back to DB",
                username, e
            );
            Some(false)
        }
        LdapAuthResult::Disabled => None,
    }
}
