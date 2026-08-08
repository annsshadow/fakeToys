use std::collections::{HashMap, HashSet};

use super::allowlist::{AllowlistEntry, DiffAllowlist};

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
    pub body: Option<serde_json::Value>,
    pub requires_auth: bool,
}

/// Endpoint comparator: parallel calls to Rust and Java and structural comparison.
pub struct EndpointComparator {
    rust_base_url: String,
    java_base_url: String,
    client: reqwest::Client,
    auth_token: Option<String>,
    pub allowlist: DiffAllowlist,
}

impl EndpointComparator {
    pub fn new(rust_base_url: impl Into<String>, java_base_url: impl Into<String>) -> Self {
        Self {
            rust_base_url: rust_base_url.into(),
            java_base_url: java_base_url.into(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap_or_default(),
            auth_token: None,
            allowlist: DiffAllowlist::empty(),
        }
    }

    /// Set the Bearer token for authenticated requests.
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Load allowlist from a YAML file.
    pub fn with_allowlist(mut self, path: &str) -> anyhow::Result<Self> {
        self.allowlist = DiffAllowlist::from_yaml(path)?;
        Ok(self)
    }

    /// Build the auth headers map if a token is set.
    fn auth_headers(&self) -> Option<HashMap<String, String>> {
        self.auth_token
            .as_ref()
            .map(|t| [("Authorization".to_string(), format!("Bearer {}", t))].iter().cloned().collect())
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
    pub async fn login(
        &self,
        base_url: &str,
        credential: &str,
        password: &str,
    ) -> Option<String> {
        let url = format!("{}/jaxrs/authentication/login", base_url.trim_end_matches('/'));
        let body = serde_json::json!({"credential": credential, "password": password});
        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
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
        let rust_url = format!("{}{}", self.rust_base_url, def.rust_path);
        let java_url = self.java_url(def);
        let headers = self.auth_headers();

        let (rust_status, rust_body) = self
            .call_endpoint(&rust_url, def.method, headers.clone(), def.body.as_ref())
            .await;
        let (java_status, java_body) = self
            .call_endpoint(&java_url, def.method, headers.clone(), def.body.as_ref())
            .await;

        let java_unreachable = java_status.is_none() && java_body.is_none();
        let status = if java_unreachable {
            ComparisonStatus::Skip
        } else {
            ComparisonStatus::Pass
        };

        let (is_equivalent, differences) =
            match (&rust_body, &java_body, java_unreachable) {
                (Some(r), Some(j), false) => {
                    let diffs = self.find_differences(r, j);
                    (diffs.is_empty(), diffs)
                }
                (None, None, false) => (true, vec![]),
                (None, None, true) => (true, vec![]),
                _ => {
                    let mut diffs = vec![];
                    if let (Some(rs), Some(js)) = (rust_status, java_status) {
                        if rs != js {
                            diffs.push(format!(
                                "HTTP status differs: Rust={} Java={}",
                                rs, js
                            ));
                        }
                    }
                    diffs.push("One or both responses are missing".to_string());
                    (false, diffs)
                }
            };

        if !is_equivalent && !java_unreachable {
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
            status: if java_unreachable {
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
        body: Option<&serde_json::Value>,
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
            request = request.json(b);
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
        let mut diffs = Vec::new();
        self.compare_values(rust, java, "root", &mut diffs, &mut HashMap::new());
        diffs
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
                        Self::recurse(self, rust, jv, &field_path, diffs, java_seen);
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
                    let field_path = if path == "root" {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    diffs.push(format!("{}: missing in Java", field_path));
                }
                for key in java_unmatched {
                    let field_path = if path == "root" {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
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
                    diffs.push(format!(
                        "{}: type differs (Rust={:?} Java={:?})",
                        path, rust, java
                    ));
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
