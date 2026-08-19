#[cfg(test)]
mod tests {
    use crate::{authenticator_from_env, LdapAuthenticator, LdapConfig};

    // Serialize env-dependent tests to prevent env var races
    static LDAP_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct EnvGuard {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn new() -> Self {
            Self { vars: Vec::new() }
        }

        fn set(mut self, key: &'static str, value: impl Into<String>) -> Self {
            let prev = std::env::var(key).ok();
            self.vars.push((key, prev));
            std::env::set_var(key, value.into());
            self
        }

        fn remove(mut self, key: &'static str) -> Self {
            let prev = std::env::var(key).ok();
            self.vars.push((key, prev));
            std::env::remove_var(key);
            self
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, prev) in self.vars.drain(..).rev() {
                match prev {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn test_ldap_config_from_env_disabled() {
        let _lock = LDAP_ENV_LOCK.lock().unwrap();
        let _g = EnvGuard::new()
            .set("LDAP_ENABLE", "false")
            .remove("LDAP_URL");
        let config = LdapConfig::from_env();
        assert!(config.is_none());
    }

    #[test]
    fn test_ldap_config_from_env_enabled_but_no_url() {
        let _lock = LDAP_ENV_LOCK.lock().unwrap();
        let _g = EnvGuard::new()
            .set("LDAP_ENABLE", "true")
            .remove("LDAP_URL")
            .set("LDAP_BASE_DN", "")
            .set("LDAP_BIND_USER", "")
            .set("LDAP_BIND_PWD", "");
        let config = LdapConfig::from_env();
        assert!(config.is_none());
    }

    #[test]
    fn test_ldap_config_from_env_enabled_with_url() {
        let _lock = LDAP_ENV_LOCK.lock().unwrap();
        let _g = EnvGuard::new()
            .set("LDAP_ENABLE", "true")
            .set("LDAP_URL", "ldap://localhost:389")
            .set("LDAP_BASE_DN", "dc=example,dc=com")
            .set("LDAP_BIND_USER", "cn=admin,dc=example,dc=com")
            .set("LDAP_BIND_PWD", "secret");
        let config = LdapConfig::from_env();
        assert!(config.is_some(), "LDAP_ENABLE={:?}, LDAP_URL={:?}", std::env::var("LDAP_ENABLE"), std::env::var("LDAP_URL"));
        let c = config.unwrap();
        assert_eq!(c.url, "ldap://localhost:389");
        assert_eq!(c.base_dn, "dc=example,dc=com");
        assert_eq!(c.bind_user, "cn=admin,dc=example,dc=com");
        assert_eq!(c.bind_password, "secret");
    }

    #[test]
    fn test_ldap_config_is_enabled() {
        let config = LdapConfig {
            url: "ldap://localhost:389".to_string(),
            base_dn: "dc=example,dc=com".to_string(),
            bind_user: String::new(),
            bind_password: String::new(),
        };
        assert!(config.is_enabled());

        let disabled = LdapConfig {
            url: String::new(),
            base_dn: String::new(),
            bind_user: String::new(),
            bind_password: String::new(),
        };
        assert!(!disabled.is_enabled());
    }

    #[tokio::test]
    async fn test_ldap_authenticator_connection_timeout() {
        let auth = LdapAuthenticator::new(LdapConfig {
            url: "ldap://192.0.2.1:389".to_string(),
            base_dn: "dc=example,dc=com".to_string(),
            bind_user: String::new(),
            bind_password: String::new(),
        });
        let result = auth.authenticate("user", "pass").await;
        assert!(matches!(result, crate::LdapAuthResult::Error(_)));
    }

    #[test]
    fn test_bind_dn_format_with_base_dn() {
        let config = LdapConfig {
            url: "ldap://localhost:389".to_string(),
            base_dn: "dc=example,dc=com".to_string(),
            bind_user: String::new(),
            bind_password: String::new(),
        };
        let auth = LdapAuthenticator::new(config);
        assert_eq!(auth.config.base_dn, "dc=example,dc=com");
    }

    #[test]
    fn test_bind_dn_format_without_base_dn() {
        let config = LdapConfig {
            url: "ldap://localhost:389".to_string(),
            base_dn: String::new(),
            bind_user: String::new(),
            bind_password: String::new(),
        };
        let auth = LdapAuthenticator::new(config);
        assert!(auth.config.base_dn.is_empty());
    }
}
