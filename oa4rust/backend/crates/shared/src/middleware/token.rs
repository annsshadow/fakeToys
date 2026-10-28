// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::Pool;

use super::constants::*;
use super::security::{is_auth_exempt, path_matches, SecurityState};
use crate::error::AppError;

pub const SESSION_COOKIE_NAME: &str = "oa4rust_session";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    Cookie(String),
    Bearer(String),
}

impl Authentication {
    pub fn token(&self) -> &str {
        match self {
            Self::Cookie(token) | Self::Bearer(token) => token,
        }
    }
}

/// Cookie 一旦存在就优先使用；空值或无效值也不回退 Bearer。
pub fn extract_authentication(headers: &HeaderMap) -> Option<Authentication> {
    if let Some(token) = session_cookie(headers) {
        return Some(Authentication::Cookie(token.to_string()));
    }
    let auth = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = auth.strip_prefix("Bearer ")?.trim();
    (!token.is_empty()).then(|| Authentication::Bearer(token.to_string()))
}

pub fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    extract_authentication(headers).and_then(|authentication| {
        let token = authentication.token();
        (!token.is_empty()).then(|| token.to_string())
    })
}

fn session_cookie(headers: &HeaderMap) -> Option<&str> {
    for cookie in headers.get_all(header::COOKIE) {
        let Ok(cookie) = cookie.to_str() else {
            continue;
        };
        for part in cookie.split(';') {
            let Some((name, value)) = part.trim().split_once('=') else {
                continue;
            };
            if name.trim() == SESSION_COOKIE_NAME {
                return Some(value.trim().trim_matches('"'));
            }
        }
    }
    None
}

pub async fn csrf_middleware(
    State(state): State<SecurityState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if !matches!(
        *request.method(),
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    ) {
        return next.run(request).await;
    }
    if !matches!(
        extract_authentication(request.headers()),
        Some(Authentication::Cookie(_))
    ) {
        return next.run(request).await;
    }
    // Cookie 认证的写请求须携带与站点一致的 Origin（CSRF 防护）。
    // 同源经回环镜像访问（localhost ⇄ 127.0.0.1）同样是可信站点，
    // 与 CORS 白名单保持同一镜像语义。
    let allowed = state.session_manager.auth_config.public_origin.clone();
    let mirror = {
        let origin_url = url::Url::parse(&allowed).ok();
        match origin_url {
            Some(url) => match url.host_str() {
                Some("localhost") => Some(format!(
                    "{}://127.0.0.1{}",
                    url.scheme(),
                    url.port().map(|p| format!(":{p}")).unwrap_or_default()
                )),
                Some("127.0.0.1") => Some(format!(
                    "{}://localhost{}",
                    url.scheme(),
                    url.port().map(|p| format!(":{p}")).unwrap_or_default()
                )),
                _ => None,
            },
            None => None,
        }
    };
    let matches = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
        .map(|origin| origin == allowed || Some(origin) == mirror.as_deref())
        .unwrap_or(false);
    if !matches {
        return AppError::Forbidden.into_response();
    }
    next.run(request).await
}

/// 系统是否未初始化：查询失败时 fail-closed。
pub(crate) async fn system_uninitialized(pool: &Pool) -> bool {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return false,
    };
    match client
        .query(
            "SELECT 1 FROM auth_person WHERE locked = false AND deleted_at IS NULL LIMIT 1",
            &[],
        )
        .await
    {
        Ok(rows) => rows.is_empty(),
        Err(_) => false,
    }
}

pub async fn auth_middleware(
    State(state): State<SecurityState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    // 双锁一致性（四轮 §三.7）：豁免表与权限注册表是两份独立权威。本函数早退时
    // **不插 Session**，而 `authorize_middleware` 只在注册表判定 Public 时放行，
    // 取不到 Session 就 401 —— 所以「豁免 ∧ ¬Public」的路对**任何人**（实测含带合法
    // 会话的 admin）都是 401 死态，尺子第 6c 段量出 19 条、`tests/exempt_authorize_parity.rs`
    // 逐臂实测 12 条红。现在豁免必须有注册表 Public 佐证，半开的锁不再可能出现。
    // `SECRET_INIT_PATHS` 的严格性保持不变：命中它就只允许未初始化窗口匿名通过
    // （即使注册表把 `/api/secret/*` 记为 Public），已初始化后仍要求会话。
    let secret_hit = SECRET_INIT_PATHS.iter().any(|p| path_matches(&path, p));
    let init_window = secret_hit && system_uninitialized(&state.pool).await;
    if is_auth_exempt(&path)
        && (init_window || (!secret_hit && super::rbac::is_registry_public(&path)))
    {
        return next.run(request).await;
    }

    let Some(authentication) = extract_authentication(request.headers()) else {
        return AppError::Unauthorized.into_response();
    };
    let token = authentication.token();
    if token.is_empty() {
        return AppError::Unauthorized.into_response();
    }
    // Migration observability: count legacy Bearer credentials reaching a protected route.
    // Intentionally no token/cookie/User-Agent value is logged.
    if matches!(authentication, Authentication::Bearer(_)) {
        tracing::warn!(
            auth_compat = "bearer",
            method = %request.method().as_str(),
            path = %path,
            "bearer compatibility credential used on protected route"
        );
    }
    match state.session_manager.validate_session(token).await {
        Some(session) => {
            request.extensions_mut().insert(session);
            next.run(request).await
        }
        None => AppError::Unauthorized.into_response(),
    }
}

#[cfg(test)]
mod token_parsing_tests {
    use super::*;
    use axum::http::HeaderValue;

    // ──────────────────────────────────────────────────────────────────────
    // 这一组只测**凭据提取**（纯函数），不测中间件。
    //
    // 提取逻辑是整条认证链的入口：它多接受一种格式，就等于多开一个绕过口；
    // 它少接受一种，用户就登不上。`src/tests.rs` 已经钉住了「Cookie 优先」
    // 与「空 Cookie 不回退 Bearer」两条主契约，这里补的是**解析器的边界**：
    // 多个 Cookie 头、带引号的值、畸形片段、非 UTF-8、大小写与前缀形式。
    // ──────────────────────────────────────────────────────────────────────

    fn cookie_headers(values: &[&str]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for v in values {
            headers.append(header::COOKIE, HeaderValue::from_str(v).unwrap());
        }
        headers
    }

    #[test]
    fn session_cookie_scans_every_cookie_header() {
        // 浏览器可以把 cookie 拆到多个 Cookie 头里；只看第一个会漏掉会话。
        let headers = cookie_headers(&["theme=dark", "oa4rust_session=abc"]);
        assert_eq!(session_cookie(&headers), Some("abc"));
    }

    #[test]
    fn session_cookie_picks_the_named_pair_out_of_a_list() {
        let headers = cookie_headers(&["a=1; oa4rust_session=xyz; b=2"]);
        assert_eq!(session_cookie(&headers), Some("xyz"));
    }

    #[test]
    fn session_cookie_unquotes_the_value() {
        // 带引号的 cookie 值必须被剥掉引号，否则查库查不到、表现为「随机掉登录」。
        let headers = cookie_headers(&[r#"oa4rust_session="abc""#]);
        assert_eq!(session_cookie(&headers), Some("abc"));
    }

    #[test]
    fn session_cookie_skips_malformed_segments() {
        // 没有 '=' 的片段、空片段都要跳过而不是整体放弃。
        let headers = cookie_headers(&["garbage; ; =novalue; oa4rust_session=ok"]);
        assert_eq!(session_cookie(&headers), Some("ok"));
    }

    #[test]
    fn session_cookie_ignores_non_utf8_header_values() {
        // 非 UTF-8 的 Cookie 头不能让提取逻辑 panic 或提前返回：
        // 跳过它，继续看下一个头。
        let mut headers = HeaderMap::new();
        headers.append(
            header::COOKIE,
            HeaderValue::from_bytes(b"\xff\xfe").unwrap(),
        );
        headers.append(
            header::COOKIE,
            HeaderValue::from_str("oa4rust_session=after-binary").unwrap(),
        );
        assert_eq!(session_cookie(&headers), Some("after-binary"));
    }

    #[test]
    fn session_cookie_returns_none_when_absent() {
        assert_eq!(session_cookie(&cookie_headers(&["theme=dark"])), None);
        assert_eq!(session_cookie(&HeaderMap::new()), None);
    }

    #[test]
    fn bearer_prefix_is_case_sensitive() {
        // `bearer` / `BEARER` 一律不认：放宽大小写会让代理/网关上的
        // 非标准头意外通过，属于认证面扩大。
        for value in ["bearer tok", "BEARER tok", "Bearer", "Bearerx tok"] {
            let mut headers = HeaderMap::new();
            headers.insert(header::AUTHORIZATION, HeaderValue::from_str(value).unwrap());
            assert_eq!(
                extract_authentication(&headers),
                None,
                "must reject authorization value {value:?}"
            );
        }
    }

    #[test]
    fn bearer_value_is_trimmed_and_empties_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str("Bearer   tok-with-spaces  ").unwrap(),
        );
        assert_eq!(
            extract_authentication(&headers),
            Some(Authentication::Bearer("tok-with-spaces".to_string()))
        );

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str("Bearer    ").unwrap(),
        );
        assert_eq!(
            extract_authentication(&headers),
            None,
            "blank bearer token must not authenticate"
        );
    }

    #[test]
    fn empty_session_cookie_does_not_fall_back_to_bearer() {
        // 与 src/tests.rs 同一条契约，这里从 token() 侧再钉一次：
        // 空 Cookie 得到的是空 token，而不是 Bearer 的值。
        let mut headers = cookie_headers(&["oa4rust_session="]);
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str("Bearer bearer-token").unwrap(),
        );

        assert_eq!(
            extract_token_from_headers(&headers),
            None,
            "空 Cookie 必须短路，不得回退 Bearer"
        );
    }

    #[test]
    fn token_accessor_returns_inner_value_for_both_variants() {
        assert_eq!(Authentication::Cookie("c".into()).token(), "c");
        assert_eq!(Authentication::Bearer("b".into()).token(), "b");
    }

    #[test]
    fn extract_token_from_headers_matches_authentication() {
        let headers = cookie_headers(&["oa4rust_session=abc"]);
        assert_eq!(
            extract_token_from_headers(&headers),
            Some("abc".to_string())
        );

        let headers = HeaderMap::new();
        assert_eq!(extract_token_from_headers(&headers), None);
    }

    // ── 2026-10-29 覆盖率轮：初始化窗口判定的真库路径 ──
    #[cfg(test)]
    mod system_init_tests {
        use super::*;
        use crate::testing::is_db_available;

        #[tokio::test]
        async fn uninitialized_window_flips_when_first_person_exists() {
            if !is_db_available().await {
                eprintln!("SKIP: no database available (set DATABASE_URL)");
                return;
            }
            let pool = crate::testing::test_pool();
            let client = pool.get().await.unwrap();
            client
                .execute("DELETE FROM auth_person WHERE id LIKE 'cov-init%'", &[])
                .await
                .unwrap();

            let base: i64 = client
            .query_one(
                "SELECT COUNT(*) AS c FROM auth_person WHERE locked = false AND deleted_at IS NULL",
                &[],
            )
            .await
            .unwrap()
            .get("c");
            if base == 0 {
                assert!(
                    system_uninitialized(&pool).await,
                    "无任何未锁定用户时系统处于未初始化窗口"
                );
            } else {
                assert!(
                    !system_uninitialized(&pool).await,
                    "已有未锁定用户时窗口必须处于关闭态"
                );
            }
            client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) VALUES ('cov-init-p1', 'cov-init-p1', 'init', 'x')",
                &[],
            )
            .await
            .unwrap();
            assert!(
                !system_uninitialized(&pool).await,
                "存在未锁定用户后窗口必须关闭"
            );
            client
                .execute("DELETE FROM auth_person WHERE id LIKE 'cov-init%'", &[])
                .await
                .unwrap();
        }
    }
}
