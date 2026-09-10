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
