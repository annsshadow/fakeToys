// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! netguard — 远程 URL 拉取的 SSRF 防护（upload/with/url 族端点共用）。
//!
//! 防护面：
//! 1. scheme 白名单：仅 http/https；
//! 2. 目标 IP 黑名单：DNS 解析出的**每一个**地址都必须全局可路由（环回/私网/
//!    CGNAT/链路本地/组播/保留/文档段一律拒绝），防打内网与云元数据端点；
//! 3. 重定向：禁用 reqwest 自动跟随，手动逐跳校验（每跳重新过 1+2），上限 5 跳；
//! 4. 体积：Content-Length 预检 + 流式累计上限，超限即断；
//! 5. 超时：连接 10s、总时长 60s。
//!
//! 段位判断手写位比较（不依赖 `Ipv4Addr::is_shared` 等 1.87 才 stable 的方法）。

use std::net::IpAddr;
use std::time::Duration;

pub const MAX_REDIRECTS: usize = 5;
pub const MAX_BODY_BYTES: usize = 100 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("url scheme not allowed (http/https only)")]
    SchemeNotAllowed,
    #[error("url host is empty or invalid")]
    InvalidHost,
    #[error("url host resolves to a non-global (private/loopback/link-local) address")]
    PrivateAddress,
    #[error("too many redirects (limit {MAX_REDIRECTS})")]
    TooManyRedirects,
    #[error("remote body exceeds {MAX_BODY_BYTES} bytes limit")]
    BodyTooLarge,
    #[error("redirect location missing or invalid")]
    BadRedirect,
    #[error("fetch failed: {0}")]
    Http(String),
}

/// IPv4 目标段黑名单。
fn v4_denied(o: [u8; 4]) -> bool {
    o[0] == 0                                        // 0.0.0.0/8 "this network"
        || o[0] == 10                                // 10.0.0.0/8 私网
        || (o[0] == 100 && (o[1] & 0xC0) == 64)      // 100.64.0.0/10 CGNAT
        || o[0] == 127                               // 127.0.0.0/8 环回
        || (o[0] == 169 && o[1] == 254)              // 169.254.0.0/16 链路本地(云元数据)
        || (o[0] == 172 && (o[1] & 0xF0) == 16)      // 172.16.0.0/12 私网
        || (o[0] == 192 && o[1] == 168)              // 192.168.0.0/16 私网
        || (o[0] == 192 && o[1] == 0 && o[2] == 2)   // 192.0.2.0/24 TEST-NET-1
        || (o[0] == 198 && (o[1] & 0xFE) == 18)      // 198.18.0.0/15 基准测试
        || (o[0] == 198 && o[1] == 51 && o[2] == 100) // 198.51.100.0/24 TEST-NET-2
        || (o[0] == 203 && o[1] == 0 && o[2] == 113) // 203.0.113.0/24 TEST-NET-3
        || o[0] >= 224 // 224/4 组播 + 240/4 保留 + 广播
}

/// IPv6 目标段黑名单（含 v4 映射地址的递归判定）。
fn v6_denied(s: [u16; 8]) -> bool {
    let all_zero = s.iter().all(|&x| x == 0);
    let is_loopback = s[7] == 1 && s[..7].iter().all(|&x| x == 0); // ::1
    let is_v4_mapped = s[5] == 0xFFFF && s[..5].iter().all(|&x| x == 0); // ::ffff:a.b.c.d
    all_zero                                       // :: 未指定
        || is_loopback
        || (s[0] & 0xFE00) == 0xFC00               // fc00::/7 ULA
        || (s[0] & 0xFFC0) == 0xFE80               // fe80::/10 链路本地
        || (s[0] & 0xFF00) == 0xFF00               // ff00::/8 组播
        || (s[0] == 0x2001 && s[1] == 0x0DB8)      // 2001:db8::/32 文档
        || (is_v4_mapped
            && v4_denied([
                (s[6] >> 8) as u8,
                s[6] as u8,
                (s[7] >> 8) as u8,
                s[7] as u8,
            ]))
}

/// 单个 IP 是否允许作为抓取目标。
pub fn ip_allowed_for_fetch(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => !v4_denied(v4.octets()),
        IpAddr::V6(v6) => !v6_denied(v6.segments()),
    }
}

/// 校验 URL：scheme 白名单 + host 全部解析结果均为全局地址。
/// hostname 走 DNS 解析后逐 IP 判定（防域名 rebinding 到内网）。
pub async fn validate_target(url: &reqwest::Url) -> Result<(), FetchError> {
    match url.scheme() {
        "http" | "https" => {}
        _ => return Err(FetchError::SchemeNotAllowed),
    }
    let Some(host) = url.host_str() else {
        return Err(FetchError::InvalidHost);
    };
    let port = url.port_or_known_default().unwrap_or(80);
    // 字面量 IP 直接判定；hostname 解析后逐个判定。
    if let Ok(ip) = host
        .trim_start_matches('[')
        .trim_end_matches(']')
        .parse::<IpAddr>()
    {
        return if ip_allowed_for_fetch(ip) {
            Ok(())
        } else {
            Err(FetchError::PrivateAddress)
        };
    }
    let addrs = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| FetchError::InvalidHost)?;
    let mut any = false;
    for a in addrs {
        any = true;
        if !ip_allowed_for_fetch(a.ip()) {
            return Err(FetchError::PrivateAddress);
        }
    }
    if !any {
        return Err(FetchError::InvalidHost);
    }
    Ok(())
}

/// 带防护的远程抓取：手动跟随重定向（每跳完整校验），流式累计体积上限。
/// 返回 (最终 URL, 字节)。
pub async fn fetch_limited(
    client: &reqwest::Client,
    url: &str,
) -> Result<(String, Vec<u8>), FetchError> {
    let mut current = reqwest::Url::parse(url).map_err(|_| FetchError::InvalidHost)?;
    for _ in 0..=MAX_REDIRECTS {
        validate_target(&current).await?;
        let resp = client
            .get(current.clone())
            .send()
            .await
            .map_err(|e| FetchError::Http(e.to_string()))?;
        let status = resp.status();
        if status.is_redirection() {
            let loc = resp
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .ok_or(FetchError::BadRedirect)?;
            current = current.join(loc).map_err(|_| FetchError::BadRedirect)?;
            continue;
        }
        if !status.is_success() {
            return Err(FetchError::Http(format!("remote answered {status}")));
        }
        if let Some(n) = resp.content_length() {
            if n as usize > MAX_BODY_BYTES {
                return Err(FetchError::BodyTooLarge);
            }
        }
        let mut body = Vec::new();
        let mut resp = resp;
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| FetchError::Http(e.to_string()))?
        {
            if body.len() + chunk.len() > MAX_BODY_BYTES {
                return Err(FetchError::BodyTooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        return Ok((current.to_string(), body));
    }
    Err(FetchError::TooManyRedirects)
}

/// 抓取专用 client：禁自动重定向 + 双超时。
pub fn fetch_client() -> Result<reqwest::Client, FetchError> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| FetchError::Http(e.to_string()))
}

/// 从原始 URL 或（重定向后的）最终 URL 提取文件名：取路径最后一个非空段并
/// 百分号解码；都取不到时回退 `remote-file`。
pub fn filename_from_url(raw: &str, final_url: &str) -> String {
    for candidate in [final_url, raw] {
        let Ok(u) = reqwest::Url::parse(candidate) else {
            continue;
        };
        if let Some(seg) = u
            .path_segments()
            .and_then(|segs| segs.rev().find(|s| !s.is_empty()).map(|s| s.to_string()))
        {
            let decoded = urlencoding::decode(&seg)
                .map(|c| c.to_string())
                .unwrap_or(seg);
            if !decoded.is_empty() {
                return decoded;
            }
        }
    }
    "remote-file".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_private_ranges_denied() {
        for ip in [
            "0.1.2.3",
            "10.0.0.1",
            "100.64.0.1",
            "100.127.255.254",
            "127.0.0.1",
            "169.254.169.254",
            "172.16.0.1",
            "172.31.255.254",
            "192.168.1.1",
            "192.0.2.1",
            "198.18.0.1",
            "198.19.255.254",
            "198.51.100.7",
            "203.0.113.9",
            "224.0.0.1",
            "240.0.0.1",
            "255.255.255.255",
        ] {
            let ip: std::net::Ipv4Addr = ip.parse().unwrap();
            assert!(!ip_allowed_for_fetch(IpAddr::V4(ip)), "{ip} must be denied");
        }
    }

    #[test]
    fn v4_global_ranges_allowed() {
        for ip in [
            "8.8.8.8",
            "1.1.1.1",
            "172.32.0.1",
            "100.128.0.1",
            "11.0.0.1",
            "198.20.0.1",
        ] {
            let ip: std::net::Ipv4Addr = ip.parse().unwrap();
            assert!(ip_allowed_for_fetch(IpAddr::V4(ip)), "{ip} must be allowed");
        }
    }

    #[test]
    fn v6_private_ranges_denied() {
        for ip in [
            "::",
            "::1",
            "fc00::1",
            "fd12:3456::1",
            "fe80::1",
            "ff02::1",
            "2001:db8::1",
        ] {
            let ip: std::net::Ipv6Addr = ip.parse().unwrap();
            assert!(!ip_allowed_for_fetch(IpAddr::V6(ip)), "{ip} must be denied");
        }
        // v4 映射私网
        let mapped: std::net::Ipv6Addr = "::ffff:10.0.0.1".parse().unwrap();
        assert!(!ip_allowed_for_fetch(IpAddr::V6(mapped)));
    }

    #[test]
    fn v6_global_allowed() {
        let ip: std::net::Ipv6Addr = "2606:4700::1".parse().unwrap();
        assert!(ip_allowed_for_fetch(IpAddr::V6(ip)));
        let mapped: std::net::Ipv6Addr = "::ffff:8.8.8.8".parse().unwrap();
        assert!(ip_allowed_for_fetch(IpAddr::V6(mapped)));
    }

    #[tokio::test]
    async fn validate_target_rejects_scheme_and_private_literals() {
        let client_ip = "http://127.0.0.1/x".parse::<reqwest::Url>().unwrap();
        assert!(matches!(
            validate_target(&client_ip).await,
            Err(FetchError::PrivateAddress)
        ));
        let metadata = "http://169.254.169.254/latest/meta-data"
            .parse::<reqwest::Url>()
            .unwrap();
        assert!(matches!(
            validate_target(&metadata).await,
            Err(FetchError::PrivateAddress)
        ));
        let v6 = "http://[::1]:8080/x".parse::<reqwest::Url>().unwrap();
        assert!(matches!(
            validate_target(&v6).await,
            Err(FetchError::PrivateAddress)
        ));
        for scheme in ["ftp://example.com/x", "file:///etc/passwd", "gopher://x/y"] {
            let u = scheme.parse::<reqwest::Url>().unwrap();
            assert!(matches!(
                validate_target(&u).await,
                Err(FetchError::SchemeNotAllowed)
            ));
        }
    }

    #[tokio::test]
    async fn validate_target_rejects_localhost_hostname() {
        // localhost 经 hosts 解析到 127.0.0.1，覆盖「hostname→DNS→私网」路径
        let u = "http://localhost/x".parse::<reqwest::Url>().unwrap();
        assert!(matches!(
            validate_target(&u).await,
            Err(FetchError::PrivateAddress)
        ));
    }

    #[tokio::test]
    async fn fetch_limited_rejects_private_target_before_request() {
        let client = fetch_client().unwrap();
        let err = fetch_limited(&client, "http://127.0.0.1:5432/").await;
        assert!(matches!(err, Err(FetchError::PrivateAddress)));
    }
}
