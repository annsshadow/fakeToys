use ldap::{LdapAuthResult, LdapConfig};

static LDAP_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn cleanup_env() {
    std::env::remove_var("LDAP_ENABLE");
    std::env::remove_var("LDAP_URL");
    std::env::remove_var("LDAP_BASE_DN");
    std::env::remove_var("LDAP_BIND_USER");
    std::env::remove_var("LDAP_BIND_PWD");
}

#[test]
fn test_ldap_config_from_env_disabled_by_default() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    let config = LdapConfig::from_env();
    assert!(config.is_none(), "LDAP should be disabled by default");
}

#[test]
fn test_ldap_config_from_env_enabled() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    std::env::set_var("LDAP_ENABLE", "true");
    std::env::set_var("LDAP_URL", "ldap://localhost:389");
    std::env::set_var("LDAP_BASE_DN", "ou=users,dc=example,dc=com");
    std::env::set_var("LDAP_BIND_USER", "cn=admin,dc=example,dc=com");
    std::env::set_var("LDAP_BIND_PWD", "admin_pass");

    let config = LdapConfig::from_env();
    assert!(config.is_some(), "LDAP should be enabled");

    let cfg = config.unwrap();
    assert_eq!(cfg.url, "ldap://localhost:389");
    assert_eq!(cfg.base_dn, "ou=users,dc=example,dc=com");
    assert_eq!(cfg.bind_user, "cn=admin,dc=example,dc=com");
    assert_eq!(cfg.bind_password, "admin_pass");
    assert!(cfg.is_enabled());

    cleanup_env();
}

#[test]
fn test_ldap_config_empty_url_disables() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    std::env::set_var("LDAP_ENABLE", "true");
    std::env::set_var("LDAP_URL", "");

    let config = LdapConfig::from_env();
    assert!(config.is_none(), "Empty LDAP_URL should disable LDAP");

    cleanup_env();
}

#[test]
fn test_ldap_config_case_insensitive_enable() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    std::env::set_var("LDAP_ENABLE", "TRUE");
    std::env::set_var("LDAP_URL", "ldap://localhost:389");

    let config = LdapConfig::from_env();
    assert!(config.is_some(), "LDAP_ENABLE=TRUE (uppercase) should work");

    cleanup_env();
}

#[test]
fn test_ldap_config_partial_env() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    std::env::set_var("LDAP_ENABLE", "true");
    std::env::set_var("LDAP_URL", "ldap://localhost:389");

    let config = LdapConfig::from_env();
    assert!(config.is_some());

    let cfg = config.unwrap();
    assert_eq!(cfg.base_dn, "");
    assert_eq!(cfg.bind_user, "");
    assert_eq!(cfg.bind_password, "");

    cleanup_env();
}

#[test]
fn test_ldap_auth_result_equality() {
    assert_eq!(LdapAuthResult::Success, LdapAuthResult::Success);
    assert_eq!(LdapAuthResult::Failed, LdapAuthResult::Failed);
    assert_eq!(LdapAuthResult::Disabled, LdapAuthResult::Disabled);
    assert_ne!(LdapAuthResult::Success, LdapAuthResult::Failed);
}

#[test]
fn test_ldap_config_is_enabled_with_url() {
    let cfg = LdapConfig {
        url: "ldap://localhost:389".to_string(),
        base_dn: "ou=users,dc=example,dc=com".to_string(),
        bind_user: "cn=admin".to_string(),
        bind_password: "pass".to_string(),
    };
    assert!(cfg.is_enabled());
}

#[test]
fn test_ldap_config_is_enabled_empty_url() {
    let cfg = LdapConfig {
        url: String::new(),
        base_dn: String::new(),
        bind_user: String::new(),
        bind_password: String::new(),
    };
    assert!(!cfg.is_enabled());
}

#[test]
fn test_ldap_authenticator_creation() {
    let cfg = LdapConfig {
        url: "ldap://localhost:389".to_string(),
        base_dn: "ou=users,dc=example,dc=com".to_string(),
        bind_user: String::new(),
        bind_password: String::new(),
    };
    let auth = ldap::LdapAuthenticator::new(cfg);
    let _ = auth;
}

#[tokio::test]
async fn test_ldap_authenticate_timeout_on_invalid_server() {
    let cfg = LdapConfig {
        url: "ldap://192.0.2.1:389".to_string(),
        base_dn: "ou=users,dc=example,dc=com".to_string(),
        bind_user: String::new(),
        bind_password: String::new(),
    };
    let auth = ldap::LdapAuthenticator::new(cfg);

    let start = std::time::Instant::now();
    let result = auth.authenticate("testuser", "testpass").await;
    let elapsed = start.elapsed();

    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "LDAP auth should timeout quickly, took {:?}",
        elapsed
    );

    match result {
        LdapAuthResult::Error(_) => {}
        LdapAuthResult::Failed => {}
        other => panic!("Expected Error or Failed, got {:?}", other),
    }
}

#[test]
fn test_authenticator_from_env_disabled() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    let auth = ldap::authenticator_from_env();
    assert!(auth.is_none());
}

#[test]
fn test_authenticator_from_env_enabled() {
    let _lock = LDAP_ENV_LOCK.lock().unwrap();
    cleanup_env();
    std::env::set_var("LDAP_ENABLE", "true");
    std::env::set_var("LDAP_URL", "ldap://localhost:389");

    let auth = ldap::authenticator_from_env();
    assert!(auth.is_some());

    cleanup_env();
}
