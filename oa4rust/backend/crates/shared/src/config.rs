// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;

use anyhow::{bail, Context};
use url::Url;

pub const DEFAULT_SESSION_TTL_SECONDS: u64 = 7200;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthConfig {
    pub public_origin: String,
    pub cookie_secure: bool,
    pub session_ttl_seconds: u64,
}

impl AuthConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let production = env::var("RUST_ENV")
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "production" | "prod"
                )
            })
            .unwrap_or(false);

        let public_origin = match env::var("APP_PUBLIC_ORIGIN") {
            Ok(value) if !value.trim().is_empty() => normalize_origin(&value)?,
            _ if production => bail!("APP_PUBLIC_ORIGIN must be set in production"),
            _ => "http://localhost:3000".to_string(),
        };

        let cookie_secure = match env::var("AUTH_COOKIE_SECURE") {
            Ok(value) => parse_bool("AUTH_COOKIE_SECURE", &value)?,
            Err(_) if production => bail!("AUTH_COOKIE_SECURE must be set in production"),
            Err(_) => false,
        };
        if production && !cookie_secure {
            bail!("AUTH_COOKIE_SECURE must be true in production");
        }
        if production && !public_origin.starts_with("https://") {
            bail!("APP_PUBLIC_ORIGIN must use https in production");
        }

        let session_ttl_seconds = match env::var("SESSION_TTL_SECONDS") {
            Ok(value) => value
                .parse::<u64>()
                .context("SESSION_TTL_SECONDS must be a positive integer")?,
            Err(_) if production => bail!("SESSION_TTL_SECONDS must be set in production"),
            Err(_) => DEFAULT_SESSION_TTL_SECONDS,
        };
        if session_ttl_seconds == 0 || session_ttl_seconds > i64::MAX as u64 {
            bail!("SESSION_TTL_SECONDS must be between 1 and {}", i64::MAX);
        }

        Ok(Self {
            public_origin,
            cookie_secure,
            session_ttl_seconds,
        })
    }
}

fn normalize_origin(value: &str) -> anyhow::Result<String> {
    let value = value.trim();
    let parsed =
        Url::parse(value).context("APP_PUBLIC_ORIGIN must be an absolute http(s) origin")?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || parsed.username() != ""
        || parsed.password().is_some()
        || parsed.path() != "/"
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        bail!("APP_PUBLIC_ORIGIN must contain only scheme, host, and optional port");
    }
    Ok(parsed.origin().ascii_serialization())
}

fn parse_bool(name: &str, value: &str) -> anyhow::Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => bail!("{} must be true/false or 1/0", name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ──────────────────────────────────────────────────────────────────────
    // AuthConfig::from_env 是**生产环境安全闸门**：它决定 cookie 是否 Secure、
    // 会话活多久、公开源站是什么。这三条任何一条被静默放宽，症状都是
    // 「本地一切正常、线上凭据明文传」——所以它的每一条 bail! 都必须有用例。
    //
    // ⚠️ 环境变量是进程级全局状态，测试线程并行 → 必须放在**同一个测试函数**里
    // 顺序执行（不引入 serial_test 依赖）。函数结尾用 Drop 守卫还原原值，
    // 即使中途断言 panic 也不会把脏环境留给后面的用例。
    // ──────────────────────────────────────────────────────────────────────

    const KEYS: [&str; 4] = [
        "RUST_ENV",
        "APP_PUBLIC_ORIGIN",
        "AUTH_COOKIE_SECURE",
        "SESSION_TTL_SECONDS",
    ];

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn capture() -> Self {
            let saved = KEYS.iter().map(|k| (*k, std::env::var(k).ok())).collect();
            for k in KEYS {
                std::env::remove_var(k);
            }
            Self { saved }
        }

        fn set(&self, key: &str, value: &str) {
            std::env::set_var(key, value);
        }

        fn clear_all(&self) {
            for k in KEYS {
                std::env::remove_var(k);
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, value) in &self.saved {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn from_env_enforces_every_security_invariant() {
        let env = EnvGuard::capture();

        // ① 完全空环境：走开发默认值，且默认**不**开 Secure cookie
        let dev = AuthConfig::from_env().expect("dev defaults");
        assert_eq!(dev.public_origin, "http://localhost:3000");
        assert!(
            !dev.cookie_secure,
            "开发默认不得静默开启 Secure（否则本地 http 登不上）"
        );
        assert_eq!(dev.session_ttl_seconds, DEFAULT_SESSION_TTL_SECONDS);

        // ② 源站只允许 scheme + host + 可选 port
        for bad in [
            "https://example.com/path",
            "https://example.com/?q=1",
            "https://example.com/#frag",
            "https://user:pw@example.com",
            "ftp://example.com",
            "not-a-url",
        ] {
            env.clear_all();
            env.set("APP_PUBLIC_ORIGIN", bad);
            assert!(
                AuthConfig::from_env().is_err(),
                "APP_PUBLIC_ORIGIN={bad:?} 必须被拒（只允许 scheme+host+port）"
            );
        }

        // ③ 合法源站要被归一化（小写 host、去尾斜杠、保留端口）
        env.clear_all();
        env.set("APP_PUBLIC_ORIGIN", "  HTTPS://Example.COM:8443  ");
        let normalized = AuthConfig::from_env().unwrap();
        assert_eq!(normalized.public_origin, "https://example.com:8443");

        // ④ 布尔解析只认 true/false/1/0
        env.clear_all();
        for (raw, expected) in [
            ("1", true),
            ("TRUE", true),
            ("0", false),
            (" false ", false),
        ] {
            env.set("AUTH_COOKIE_SECURE", raw);
            assert_eq!(
                AuthConfig::from_env().unwrap().cookie_secure,
                expected,
                "AUTH_COOKIE_SECURE={raw:?}"
            );
        }
        env.set("AUTH_COOKIE_SECURE", "yes");
        assert!(AuthConfig::from_env().is_err(), "非布尔字面量必须被拒");

        // ⑤ TTL 必须是正数且不溢出
        env.clear_all();
        env.set("SESSION_TTL_SECONDS", "60");
        assert_eq!(AuthConfig::from_env().unwrap().session_ttl_seconds, 60);
        for bad in ["0", "abc", "-1", "99999999999999999999"] {
            env.set("SESSION_TTL_SECONDS", bad);
            assert!(
                AuthConfig::from_env().is_err(),
                "SESSION_TTL_SECONDS={bad:?} 必须被拒"
            );
        }

        // ⑥ 生产模式：缺任何一项都必须启动失败（fail closed）
        env.clear_all();
        env.set("RUST_ENV", "production");
        assert!(
            AuthConfig::from_env().is_err(),
            "生产缺 APP_PUBLIC_ORIGIN 必须失败"
        );

        env.set("APP_PUBLIC_ORIGIN", "https://oa.example.com");
        assert!(
            AuthConfig::from_env().is_err(),
            "生产缺 AUTH_COOKIE_SECURE 必须失败"
        );

        env.set("AUTH_COOKIE_SECURE", "false");
        assert!(
            AuthConfig::from_env().is_err(),
            "生产里 AUTH_COOKIE_SECURE=false 必须失败"
        );

        env.set("AUTH_COOKIE_SECURE", "true");
        assert!(
            AuthConfig::from_env().is_err(),
            "生产缺 SESSION_TTL_SECONDS 必须失败"
        );

        env.set("SESSION_TTL_SECONDS", "3600");
        let prod = AuthConfig::from_env().expect("production config with all required values");
        assert_eq!(prod.public_origin, "https://oa.example.com");
        assert!(prod.cookie_secure);
        assert_eq!(prod.session_ttl_seconds, 3600);

        // ⑦ 生产 + http 源站必须被拒（否则 cookie 会明文回传）
        env.set("APP_PUBLIC_ORIGIN", "http://oa.example.com");
        assert!(AuthConfig::from_env().is_err(), "生产源站必须是 https");

        // ⑧ `prod` 是 `production` 的别名，且大小写/空白不敏感
        env.clear_all();
        env.set("RUST_ENV", "  PROD  ");
        assert!(
            AuthConfig::from_env().is_err(),
            "RUST_ENV=prod 必须同样按生产模式校验"
        );

        // ⑨ 非生产值（如 staging）不触发生产校验
        env.clear_all();
        env.set("RUST_ENV", "staging");
        assert!(AuthConfig::from_env().is_ok(), "staging 不套用生产强约束");
    }
}
