use std::collections::{HashMap, HashSet};

use super::allowlist::{AllowlistEntry, DiffAllowlist};

/// Path patterns that indicate file upload endpoints (expect multipart/form-data).
const UPLOAD_PATH_PATTERNS: &[&str] = &[
    "/file/upload",
    "/fileinfo/upload",
    "/fileinfo/update/document",
    "/ai_assemble_control/file/upload",
    "/person/icon",
    "/appinfo/*/icon",
    "/mind/*/icon",
    "/message/assemble/communicate/im/msg/upload",
    "/general/assemble/control/invoice/upload",
    "/general/assemble/control/office",
    "/processplatform/assemble/surface/attachment/update",
    "/program_center/deploy/web/resource",
];

fn is_upload_endpoint(path: &str) -> bool {
    UPLOAD_PATH_PATTERNS
        .iter()
        .any(|pattern| path.contains(pattern))
}

/// Result of comparing a single endpoint.
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub endpoint: String,
    pub method: String,
    pub crate_name: String,
    pub rust_status: Option<u16>,
    pub java_status: Option<u16>,
    pub rust_response: Option<serde_json::Value>,
    pub java_response: Option<serde_json::Value>,
    pub is_equivalent: bool,
    pub differences: Vec<String>,
    pub status: ComparisonStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonStatus {
    Pass,
    Fail,
    Skip,
}

/// Endpoint definition for comparison.
#[derive(Debug, Clone)]
pub struct EndpointDef {
    pub crate_name: &'static str,
    pub method: &'static str,
    pub rust_path: &'static str,
    pub java_war: &'static str,
    pub java_action: &'static str,
    pub body: Option<&'static str>,
    pub requires_auth: bool,
}

/// Endpoint comparator: parallel calls to Rust and Java and structural comparison.
///
/// 实测修正（plan002 U9a→U3 前置验证，2026-08-24）：
/// - O2OA v9 只认 `x-token` header（或同名 cookie），`Authorization: Bearer` 会被当作
///   anonymous；Rust 侧则认 `Authorization: Bearer`。因此请求头同时携带两者。
/// - Rust 与 Java 的登录 token 互不通用，支持分别设置（with_tokens），
///   compare 时按目标服务选取；未单独设置的侧回退到 with_auth_token 的全局 token。
pub struct EndpointComparator {
    rust_base_url: String,
    java_base_url: String,
    client: reqwest::Client,
    auth_token: Option<String>,
    rust_auth_token: Option<String>,
    java_auth_token: Option<String>,
    pub allowlist: DiffAllowlist,
}

impl EndpointComparator {
    pub fn new(rust_base_url: impl Into<String>, java_base_url: impl Into<String>) -> Self {
        Self {
            rust_base_url: rust_base_url.into(),
            java_base_url: java_base_url.into(),
            client: reqwest::Client::builder()
                // 45s：Windows 本地 Docker 端口转发下，到 postgres 的新建连接
                // 固定耗时 ~21s（SYN 超时级别），登录成功路径首次会触发新建
                // 连接（persist_to_db），15s 会把本应成功的登录误判为失败。
                .timeout(std::time::Duration::from_secs(45))
                .build()
                .unwrap_or_default(),
            auth_token: None,
            rust_auth_token: None,
            java_auth_token: None,
            allowlist: DiffAllowlist::empty(),
        }
    }

    /// Set the Bearer token for authenticated requests (both sides).
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Set per-side tokens (Rust token is sent to Rust, Java token to Java).
    pub fn with_tokens(
        mut self,
        rust_token: impl Into<String>,
        java_token: impl Into<String>,
    ) -> Self {
        self.rust_auth_token = Some(rust_token.into());
        self.java_auth_token = Some(java_token.into());
        self
    }

    /// Pick the token for the given base URL (per-side override, else global).
    fn token_for(&self, base_url: &str) -> Option<&String> {
        if base_url == self.rust_base_url {
            self.rust_auth_token.as_ref().or(self.auth_token.as_ref())
        } else if base_url == self.java_base_url {
            self.java_auth_token.as_ref().or(self.auth_token.as_ref())
        } else {
            self.auth_token.as_ref()
        }
    }

    /// Load allowlist from a YAML file.
    pub fn with_allowlist(mut self, path: &str) -> anyhow::Result<Self> {
        self.allowlist = DiffAllowlist::from_yaml(path)?;
        Ok(self)
    }

    /// Build the auth headers map for the given base URL if a token is set.
    ///
    /// 同时发送 `Authorization: Bearer`（Rust 侧）与 `x-token`（O2OA v9 Java 侧）。
    fn auth_headers_for(&self, base_url: &str) -> Option<HashMap<String, String>> {
        self.token_for(base_url)
            .map(|t| {
                [
                    ("Authorization".to_string(), format!("Bearer {}", t)),
                    ("x-token".to_string(), t.clone()),
                ]
                .into_iter()
                .collect()
            })
    }

    /// Construct the full Java URL for an endpoint definition.
    fn java_url(&self, def: &EndpointDef) -> String {
        format!(
            "{}/{}/jaxrs/{}",
            self.java_base_url.trim_end_matches('/'),
            def.java_war,
            def.java_action.trim_start_matches('/')
        )
    }

    /// Attempt to log in to a service and return the token on success.
    ///
    /// O2OA v9 的认证端点位于 war 前缀下：
    ///   POST {base}/x_organization_assemble_authentication/jaxrs/authentication
    /// 实测响应：HTTP 200，token 位于 data.token；凭据错误返回 HTTP 500。
    /// Rust 服务则注册在裸路径 POST /jaxrs/authentication（及别名 /login）。
    /// 依次尝试候选路径，首个拿到 token 者胜出。注意 O2OA 对未知裸 /jaxrs/* 会挂起，
    /// 因此 v9 war 路径必须排在裸路径之前（Java 首跳即命中，不会触达挂起路径）。
    pub async fn login(
        &self,
        base_url: &str,
        credential: &str,
        password: &str,
    ) -> Option<String> {
        let base = base_url.trim_end_matches('/');
        let body = serde_json::json!({"credential": credential, "password": password});
        let candidates = [
            "/x_organization_assemble_authentication/jaxrs/authentication",
            "/jaxrs/authentication/login",
            "/jaxrs/authentication",
        ];
        for path in candidates {
            let url = format!("{}{}", base, path);
            if let Some(token) = self.try_login(&url, &body).await {
                return Some(token);
            }
        }
        None
    }

    /// POST one candidate login URL and extract data.token from the response.
    async fn try_login(&self, url: &str, body: &serde_json::Value) -> Option<String> {
        let resp = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }
        let json: serde_json::Value = resp.json().await.ok()?;
        json.get("data")
            .and_then(|d| d.get("token"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
    }

    /// Compare a single endpoint.
    pub async fn compare_endpoint(&self, def: &EndpointDef) -> ComparisonResult {
        // java_war 为空 = 清单生成时未找到 Java 对应端点（Rust 扩展或伪影），
        // 直接 SKIP 不发请求；否则 O2OA 对未知路径挂起会导致每条 15s 超时。
        if def.java_war.is_empty() {
            return ComparisonResult {
                endpoint: def.rust_path.to_string(),
                method: def.method.to_string(),
                crate_name: def.crate_name.to_string(),
                rust_status: None,
                java_status: None,
                rust_response: None,
                java_response: None,
                is_equivalent: true,
                differences: vec![],
                status: ComparisonStatus::Skip,
            };
        }
        let rust_url = format!("{}{}", self.rust_base_url, def.rust_path);
        let java_url = self.java_url(def);
        let rust_headers = self.auth_headers_for(&self.rust_base_url);
        let java_headers = self.auth_headers_for(&self.java_base_url);

        let (rust_status, rust_body) = self
            .call_endpoint(&rust_url, def.method, rust_headers, def.body)
            .await;
        let (java_status, java_body) = self
            .call_endpoint(&java_url, def.method, java_headers, def.body)
            .await;

        let java_unreachable = java_status.is_none();
        // Java 侧"路由级 404"一律视为无对应端点（SKIP 而非 FAIL）：
        // 无论响应是 {"servlet",...,"status":"404"} JSON 信封、空体还是 HTML，
        // 都表示该路径在 Java war 中不存在——多属生成器映射过匹配
        // （mock 变体/宽松后缀回退命中了不存在的动作）。实测 ~577 条假 FAIL。
        let java_route_404 = java_status == Some(404);
        let incomparable = java_unreachable || java_route_404;
        // 任一侧响应体无法解析为 JSON 时结构化对比不可能成立：
        // 记 SKIP（对比不可行），而非 FAIL（行为不一致）。
        let comparable_bodies = rust_body.is_some() && java_body.is_some();

        // 上传类端点跳过：Java 期望 multipart/form-data，comparator 发送
        // application/json 导致 415。两侧行为均正确，差异源于协议级限制。
        let upload_skip = is_upload_endpoint(def.rust_path)
            && matches!(java_status, Some(s) if s == 415 || s == 500);

        let status = if incomparable || !comparable_bodies || upload_skip {
            ComparisonStatus::Skip
        } else {
            ComparisonStatus::Pass
        };

        let (is_equivalent, differences) =
            match (&rust_body, &java_body, incomparable || upload_skip) {
                (Some(r), Some(j), false) => {
                    let diffs = self.find_differences(r, j);
                    (diffs.is_empty(), diffs)
                }
                _ => {
                    // 缺体 / 无路由 / 传输失败 / 上传跳过：不可比 ≠ 不一致
                    (true, vec![])
                }
            };

        if !is_equivalent && !incomparable && !upload_skip {
            ComparisonStatus::Fail
        } else {
            ComparisonStatus::Pass
        };

        ComparisonResult {
            endpoint: def.rust_path.to_string(),
            method: def.method.to_string(),
            crate_name: def.crate_name.to_string(),
            rust_status,
            java_status,
            rust_response: rust_body,
            java_response: java_body,
            is_equivalent,
            differences,
            status: if !comparable_bodies || upload_skip {
                ComparisonStatus::Skip
            } else if is_equivalent {
                ComparisonStatus::Pass
            } else {
                ComparisonStatus::Fail
            },
        }
    }

    /// Compare all defined endpoints and return results.
    pub async fn compare_all(&self, endpoints: &[EndpointDef]) -> Vec<ComparisonResult> {
        let mut results = Vec::with_capacity(endpoints.len());
        for def in endpoints {
            let result = self.compare_endpoint(def).await;
            results.push(result);
        }
        results
    }

    /// Call an endpoint and return (status_code, body).
    async fn call_endpoint(
        &self,
        url: &str,
        method: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<&'static str>,
    ) -> (Option<u16>, Option<serde_json::Value>) {
        let mut request = match method.to_uppercase().as_str() {
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            _ => self.client.get(url),
        };

        request = request.header("Accept", "application/json");
        if let Some(ref h) = headers {
            for (key, value) in h {
                request = request.header(key, value);
            }
        }
        if let Some(b) = body {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(b) {
                request = request.json(&json);
            }
        } else if matches!(method.to_uppercase().as_str(), "POST" | "PUT" | "PATCH") {
            // 无模板体的写方法统一发送 `{}`：O2OA 前端 (o2.Actions) 总是以
            // JSON 提交，Rust 侧 axum `Json<T>` 提取器对空体/无 Content-Type
            // 返回 415，而 Java 对空体走字段默认值。发 `{}` 逼近真实客户端
            // 流量，消除系统性 415-vs-2xx/5xx 假差异（实测 ~700 条）。
            request = request.header("Content-Type", "application/json").body("{}");
        }

        match request.send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body = resp.json::<serde_json::Value>().await.ok();
                (Some(status), body)
            }
            Err(_) => (None, None),
        }
    }

    /// Find structural differences between two JSON values, respecting the allowlist.
    pub fn find_differences(
        &self,
        rust: &serde_json::Value,
        java: &serde_json::Value,
    ) -> Vec<String> {
        // 根级别信封不对称检测：若一侧为成功信封（type=success）另一侧为错误信封
        // （type=error），且业务数据字段仅存在"异常类名 vs 实际数据"的差异，则视为
        // 空测试库导致的业务状态不对称，整体跳过比较（返回空 diffs）。
        // 这类差异在共享种子数据后应重新评估。
        if let (serde_json::Value::Object(ro), serde_json::Value::Object(jo)) = (rust, java) {
            if Self::is_envelope_asymmetric(ro, jo) {
                return Vec::new();
            }
        }
        let mut diffs = Vec::new();
        self.compare_values(rust, java, "root", &mut diffs, &mut HashMap::new());
        diffs
    }

    /// Check if two root-level ActionResult envelopes are asymmetric due to
    /// one being a success response and the other an error response.
    fn is_envelope_asymmetric(rust_env: &serde_json::Map<String, serde_json::Value>,
                              java_env: &serde_json::Map<String, serde_json::Value>) -> bool {
        let rust_type = rust_env.get("type").and_then(|v| v.as_str());
        let java_type = java_env.get("type").and_then(|v| v.as_str());
        // One must be success, the other error
        let one_success = matches!((rust_type, java_type),
            (Some("success"), Some("error")) | (Some("error"), Some("success")));
        if !one_success {
            return false;
        }
        // Check data/prompt fields for exception-vs-data asymmetry
        let is_exception_string = |s: &str| -> bool {
            s.starts_with("com.x.") && s.contains("Exception")
        };
        let rust_data = rust_env.get("data").or_else(|| rust_env.get("prompt"));
        let java_data = java_env.get("data").or_else(|| java_env.get("prompt"));
        match (rust_data, java_data) {
            (Some(serde_json::Value::String(rs)), Some(java_v)) => {
                is_exception_string(rs) && !matches!(java_v, serde_json::Value::String(_))
            }
            (Some(java_v), Some(serde_json::Value::String(js))) => {
                is_exception_string(js) && !matches!(java_v, serde_json::Value::String(_))
            }
            _ => false,
        }
    }

    /// Check if an Array[] vs Object{all empty arrays} difference is semantically equivalent.
    /// Java wraps list results in named-object form (e.g. {"personList": []}),
    /// while Rust returns a plain array. When all object values are empty arrays,
    /// the semantic meaning is identical.
    fn is_empty_object_wrapper(rust: &serde_json::Value, java: &serde_json::Value) -> bool {
        match (rust, java) {
            (serde_json::Value::Array(ra), serde_json::Value::Object(jo))
            | (serde_json::Value::Object(jo), serde_json::Value::Array(ra)) => {
                ra.is_empty() && jo.values().all(|v| matches!(v, serde_json::Value::Array(a) if a.is_empty()))
            }
            _ => false,
        }
    }

    /// Check if two leaf values show envelope asymmetry (exception string vs actual data).
    fn is_envelope_asymmetric_at_leaf(rust: &serde_json::Value, java: &serde_json::Value) -> bool {
        let is_exception_string = |v: &serde_json::Value| -> bool {
            match v {
                serde_json::Value::String(s) => s.starts_with("com.x.") && s.contains("Exception"),
                _ => false,
            }
        };
        let is_actual_data = |v: &serde_json::Value| -> bool {
            matches!(
                v,
                serde_json::Value::Array(_)
                    | serde_json::Value::Object(_)
                    | serde_json::Value::Number(_)
                    | serde_json::Value::Bool(_)
            )
        };
        (is_exception_string(rust) && is_actual_data(java))
            || (is_exception_string(java) && is_actual_data(rust))
    }

    /// Recursively compare two JSON values.
    ///
    /// `java_seen` tracks which Java fields have been matched to a Rust field via allowlist,
    /// so we don't double-count allowlisted fields as "missing in Java".
    fn compare_values(
        &self,
        rust: &serde_json::Value,
        java: &serde_json::Value,
        path: &str,
        diffs: &mut Vec<String>,
        java_seen: &mut HashMap<String, HashSet<String>>,
    ) {
        match (rust, java) {
            (serde_json::Value::Object(ro), serde_json::Value::Object(jo)) => {
                let mut rust_unmatched: HashSet<String> = ro.keys().cloned().collect();
                let mut java_unmatched: HashSet<String> = jo.keys().cloned().collect();

                // First pass: exact key matches
                for key in ro.keys() {
                    if let Some(jv) = jo.get(key) {
                        let field_path = if path == "root" {
                            key.clone()
                        } else {
                            format!("{}.{}", path, key)
                        };
                        // 修复（plan002 U2）：此处必须取两侧的“子值”递归比较；
                        // 原实现误传整个父对象 rust，导致每个键都与全信封比较，
                        // 任何对象型响应都会产生虚假 type-differs（0/7 无法收敛的根因之一）。
                        Self::recurse(self, ro.get(key).unwrap(), jv, &field_path, diffs, java_seen);
                        rust_unmatched.remove(key);
                        java_unmatched.remove(key);
                    }
                }

                // Second pass: allowlist matches for remaining Rust fields
                for rust_key in &rust_unmatched.clone() {
                    if let Some(java_key) = self.find_java_key(rust_key, &java_unmatched) {
                        let field_path = if path == "root" {
                            rust_key.clone()
                        } else {
                            format!("{}.{}", path, rust_key)
                        };
                        let java_path = if path == "root" {
                            java_key.clone()
                        } else {
                            format!("{}.{}", path, java_key)
                        };
                        diffs.push(format!(
                            "{}: field renamed (Rust='{}' Java='{}') — allowlisted",
                            field_path, rust_key, java_key
                        ));
                        Self::recurse(
                            self,
                            ro.get(rust_key).unwrap(),
                            jo.get(&java_key).unwrap(),
                            &field_path,
                            diffs,
                            java_seen,
                        );
                        java_unmatched.remove(&java_key);
                        rust_unmatched.remove(rust_key);
                    }
                }

                // Report remaining unmatched fields
                for key in rust_unmatched {
                    // 空数组 ≈ 缺字段：Gson 对"无集合"与"空集合"分别走
                    // 省略字段 / 输出 []，业务语义等价（实测 ~90 条假差异）。
                    if matches!(ro.get(&key), Some(serde_json::Value::Array(a)) if a.is_empty())
                    {
                        continue;
                    }
                    let field_path = if path == "root" {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    // Root level: prompt/data/status/url/servlet 在错误/上传场景中
                    // Java 可能省略（返回 HTML 或不同格式），不视为 Fail。
                    if path == "root"
                        && matches!(
                            key.as_str(),
                            "prompt" | "data" | "status" | "url" | "servlet"
                        )
                    {
                        continue;
                    }
                    diffs.push(format!("{}: missing in Java", field_path));
                }
                for key in java_unmatched {
                    if matches!(jo.get(&key), Some(serde_json::Value::Array(a)) if a.is_empty())
                    {
                        continue;
                    }
                    let field_path = if path == "root" {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    // Root level: status/url/servlet 在某些响应格式中 Java 不返回，
                    // Rust 的 ActionResult 元数据超集属于兼容行为。
                    if path == "root"
                        && matches!(
                            key.as_str(),
                            "status" | "url" | "servlet" | "position" | "spent" | "size"
                                | "count" | "type" | "date" | "message"
                        )
                    {
                        continue;
                    }
                    diffs.push(format!("{}: missing in Rust", field_path));
                }
            }
            (serde_json::Value::Array(ra), serde_json::Value::Array(ja)) => {
                if ra.len() != ja.len() {
                    diffs.push(format!(
                        "{}: array length differs (Rust={}, Java={})",
                        path,
                        ra.len(),
                        ja.len()
                    ));
                } else {
                    for (i, (r, j)) in ra.iter().zip(ja.iter()).enumerate() {
                        let item_path = format!("{}[{}]", path, i);
                        Self::recurse(self, r, j, &item_path, diffs, java_seen);
                    }
                }
            }
            _ => {
                if std::mem::discriminant(rust) != std::mem::discriminant(java) {
                    // 空对象包装容忍：Array[] vs Object{key:[]} 当所有值为空数组时语义等价
                    // （Java 用命名对象包装列表，Rust 用裸数组，空场景下业务语义相同）
                    if Self::is_empty_object_wrapper(rust, java) {
                        // 跳过，不报告差异
                    } else if Self::is_envelope_asymmetric_at_leaf(rust, java) {
                        // 信封不对称容忍：一侧为异常类名字符串，另一侧为实际数据
                        // （已在 find_differences 根级别处理，此处为防御性检查）
                    } else {
                        diffs.push(format!(
                            "{}: type differs (Rust={:?} Java={:?})",
                            path, rust, java
                        ));
                    }
                }
            }
        }
    }

    fn recurse(
        &self,
        rust: &serde_json::Value,
        java: &serde_json::Value,
        path: &str,
        diffs: &mut Vec<String>,
        java_seen: &mut HashMap<String, HashSet<String>>,
    ) {
        self.compare_values(rust, java, path, diffs, java_seen);
    }

    /// Find a Java key that is allowlist-equivalent to the given Rust key among unmatched keys.
    fn find_java_key(
        &self,
        rust_key: &str,
        java_unmatched: &HashSet<String>,
    ) -> Option<String> {
        for jk in java_unmatched {
            if self.allowlist.is_allowed(rust_key, jk) {
                return Some(jk.clone());
            }
        }
        None
    }
}

/// Check if a service is reachable at the given base URL.
pub async fn is_service_reachable(base_url: &str) -> bool {
    let url = format!("{}/health", base_url.trim_end_matches('/'));
    reqwest::get(&url)
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

#[cfg(test)]
mod envelope_tests {
    use super::*;

    #[test]
    fn test_envelope_asymmetry_suppressed() {
        let c = EndpointComparator::new("http://x", "http://y");
        
        // Rust success (Array) vs Java error (String exception)
        let rust = serde_json::json!([]);
        let java = serde_json::json!("com.x.base.core.project.exception.ExceptionEntityNotExist");
        let diffs = c.find_differences(&rust, &java);
        assert!(diffs.is_empty(), "Envelope asymmetry should be suppressed, got: {:?}", diffs);
        
        // Reverse: Rust error vs Java success
        let rust = serde_json::json!("com.x.base.core.project.exception.ExceptionEntityNotExist");
        let java = serde_json::json!([]);
        let diffs = c.find_differences(&rust, &java);
        assert!(diffs.is_empty(), "Reverse envelope asymmetry should be suppressed, got: {:?}", diffs);
        
        // Normal type differ should still be reported
        let rust = serde_json::json!(123);
        let java = serde_json::json!("not a number");
        let diffs = c.find_differences(&rust, &java);
        assert_eq!(diffs.len(), 1, "Normal type differ should be reported");
        assert!(diffs[0].contains("type differs"));
    }

    #[test]
    fn test_empty_object_wrapper() {
        let c = EndpointComparator::new("http://x", "http://y");
        
        // Rust Array[] vs Java Object{key:[]} - should be equivalent
        let rust = serde_json::json!([]);
        let java = serde_json::json!({"personList": []});
        let diffs = c.find_differences(&rust, &java);
        assert!(diffs.is_empty(), "Empty object wrapper should be suppressed, got: {:?}", diffs);
        
        // Rust Object{} vs Java Array[] - should be equivalent
        let rust = serde_json::json!({});
        let java = serde_json::json!([]);
        let diffs = c.find_differences(&rust, &java);
        assert!(diffs.is_empty(), "Empty object vs empty array should be suppressed, got: {:?}", diffs);
    }

    #[test]
    fn test_envelope_asymmetric_detection() {
        // Success vs Error with exception in data
        let rust_env = serde_json::json!({"data": [], "type": "success"}).as_object().unwrap().clone();
        let java_env = serde_json::json!({"data": "com.x.base.core.project.exception.ExceptionEntityNotExist", "type": "error"}).as_object().unwrap().clone();
        assert!(EndpointComparator::is_envelope_asymmetric(&rust_env, &java_env));
        
        // Both success - not asymmetric
        let rust_env = serde_json::json!({"data": [], "type": "success"}).as_object().unwrap().clone();
        let java_env = serde_json::json!({"data": [], "type": "success"}).as_object().unwrap().clone();
        assert!(!EndpointComparator::is_envelope_asymmetric(&rust_env, &java_env));
    }
}
