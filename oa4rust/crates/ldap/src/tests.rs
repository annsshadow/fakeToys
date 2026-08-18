#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn cleanup_env() {
        env::remove_var("LDAP_ENABLE");
        env::remove_var("LDAP_URL");
        env::remove_var("LDAP_BASE_DN");
        env::remove_var("LDAP_BIND_USER");
        env::remove_var("LDAP_BIND_PWD");
    }

    #[test]
    fn test_from_env_disabled() {
        cleanup_env();
        assert!(LdapConfig::from_env().is_none());
    }

    #[test]
    fn test_from_env_enabled_no_url() {
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        assert!(LdapConfig::from_env().is_none());
    }

    #[test]
    fn test_from_env_enabled_with_url() {
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:389");
        let config = LdapConfig::from_env().expect("should have config");
        assert_eq!(config.url, "ldap://localhost:389");
        assert_eq!(config.base_dn, "");
    }

    #[test]
    fn test_from_env_with_all_vars() {
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
        cleanup_env();
        assert!(authenticator_from_env().is_none());
    }

    #[test]
    fn test_authenticator_from_env_enabled() {
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:389");
        let auth = authenticator_from_env();
        assert!(auth.is_some());
    }

    #[tokio::test]
    async fn test_authenticate_returns_error_without_server() {
        cleanup_env();
        env::set_var("LDAP_ENABLE", "true");
        env::set_var("LDAP_URL", "ldap://localhost:1");
        let config = LdapConfig::from_env().unwrap();
        let auth = LdapAuthenticator::new(config);
        let result = auth.authenticate("testuser", "testpass").await;
        // Should return Error because server is not available
        assert!(matches!(result, LdapAuthResult::Error(_)));
    }
}
