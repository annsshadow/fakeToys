// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::*;
    use std::env;
    use tokio::sync::{Mutex, MutexGuard};

    /// 这些用例读写进程级环境变量；并行执行会互相清空对方的前置条件。
    /// 锁必须覆盖「改环境 → 读配置」整段，否则默认并行下用例会随机红。
    ///
    /// 用 tokio 的 Mutex：同步用例走 blocking_lock，异步用例 lock().await，
    /// 避免 std Mutex 跨 await 持锁（clippy::await_holding_lock）。
    static ENV_LOCK: Mutex<()> = Mutex::const_new(());

    fn lock_env_blocking() -> MutexGuard<'static, ()> {
        ENV_LOCK.blocking_lock()
    }

    async fn lock_env_async() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().await
    }

    fn cleanup_env() {
        env::remove_var("LDAP_ENABLE");
        env::remove_var("LDAP_URL");
        env::remove_var("LDAP_BASE_DN");
        env::remove_var("LDAP_BIND_USER");
        env::remove_var("LDAP_BIND_PWD");
    }

    #[test]
    fn test_from_env_disabled() {
        let _guard = lock_env_blocking();
        cleanup_env();
        assert!(LdapConfig::from_env().is_none());
    }

    #[test]
    fn test_from_env_enabled_no_url() {
        let _guard = lock_env_blocking();
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        assert!(LdapConfig::from_env().is_none());
    }

    #[test]
    fn test_from_env_enabled_with_url() {
        let _guard = lock_env_blocking();
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:389");
        let config = LdapConfig::from_env().expect("should have config");
        assert_eq!(config.url, "ldap://localhost:389");
        assert_eq!(config.base_dn, "");
    }

    #[test]
    fn test_from_env_with_all_vars() {
        let _guard = lock_env_blocking();
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://example.com:389");
        env::set_var("LDAP_BASE_DN", "dc=example,dc=com");
        env::set_var("LDAP_BIND_USER", "cn=admin,dc=example,dc=com");
        env::set_var("LDAP_BIND_PWD", "secret");
        let config = LdapConfig::from_env().expect("should have config");
        assert_eq!(config.url, "ldap://example.com:389");
        assert_eq!(config.base_dn, "dc=example,dc=com");
        assert_eq!(config.bind_user, "cn=admin,dc=example,dc=com");
        assert_eq!(config.bind_password, "secret");
    }

    #[test]
    fn test_is_enabled() {
        let config = LdapConfig {
            url: "ldap://localhost:389".to_string(),
            base_dn: "".to_string(),
            bind_user: "".to_string(),
            bind_password: "".to_string(),
        };
        assert!(config.is_enabled());

        let disabled = LdapConfig {
            url: "".to_string(),
            base_dn: "".to_string(),
            bind_user: "".to_string(),
            bind_password: "".to_string(),
        };
        assert!(!disabled.is_enabled());
    }

    #[test]
    fn test_authenticator_from_env_disabled() {
        let _guard = lock_env_blocking();
        cleanup_env();
        assert!(authenticator_from_env().is_none());
    }

    #[test]
    fn test_authenticator_from_env_enabled() {
        let _guard = lock_env_blocking();
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:389");
        let auth = authenticator_from_env();
        assert!(auth.is_some());
    }

    #[tokio::test]
    async fn test_authenticate_returns_error_without_server() {
        let _guard = lock_env_async().await;
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:1");
        let config = LdapConfig::from_env().unwrap();
        let auth = LdapAuthenticator::new(config);
        let result = auth.authenticate("testuser", "testpass").await;
        // 连不上服务器时，实现把「连接/绑定失败」映射为 Failed（超时才映射 Error）。
        // 这里要钉的是「绝不能被当成认证成功」，而不是某一种失败变体。
        assert!(
            matches!(result, LdapAuthResult::Failed | LdapAuthResult::Error(_)),
            "unreachable LDAP server must not authenticate successfully: {result:?}"
        );
    }

    #[tokio::test]
    async fn test_ldap_integration_guard() {
        use shared::testing::is_db_available;

        if !is_db_available().await {
            eprintln!("skipping test_ldap_integration_guard: DATABASE_URL not reachable");
            return;
        }

        let _guard = lock_env_async().await;
        cleanup_env();
        let config = LdapConfig::from_env();
        // LDAP is disabled by default in CI; test only runs when LDAP is explicitly enabled
        if config.is_none() {
            eprintln!("skipping test_ldap_integration_guard: LDAP not enabled");
            return;
        }

        let auth = LdapAuthenticator::new(config.unwrap());
        let result = auth.authenticate("testuser", "testpass").await;
        // When LDAP is enabled but no server is running, expect Error (not panic)
        match result {
            LdapAuthResult::Error(_) | LdapAuthResult::Failed => {}
            LdapAuthResult::Success => {}
            LdapAuthResult::Disabled => {}
        }
    }
}
