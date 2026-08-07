/// 端点对比结果
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub endpoint: String,
    pub method: String,
    pub rust_response: Option<serde_json::Value>,
    pub java_response: Option<serde_json::Value>,
    pub is_equivalent: bool,
    pub differences: Vec<String>,
}

/// 端点对比器：并行调用 Rust 和 Java 端点并对比响应
pub struct EndpointComparator {
    rust_base_url: String,
    java_base_url: String,
    client: reqwest::Client,
}

impl EndpointComparator {
    pub fn new(rust_base_url: impl Into<String>, java_base_url: impl Into<String>) -> Self {
        Self {
            rust_base_url: rust_base_url.into(),
            java_base_url: java_base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    /// 对比单个端点的响应
    pub async fn compare_endpoint(
        &self,
        method: &str,
        path: &str,
        rust_headers: Option<std::collections::HashMap<String, String>>,
        java_headers: Option<std::collections::HashMap<String, String>>,
    ) -> ComparisonResult {
        let rust_url = format!("{}{}", self.rust_base_url, path);
        let java_url = format!("{}{}", self.java_base_url, path);

        let rust_response = self.call_endpoint(&rust_url, method, rust_headers).await;
        let java_response = self.call_endpoint(&java_url, method, java_headers).await;

        let (is_equivalent, differences) = match (&rust_response, &java_response) {
            (Some(r), Some(j)) => {
                let diffs = self.find_differences(r, j);
                (diffs.is_empty(), diffs)
            }
            (None, None) => (true, vec![]),
            _ => (false, vec!["One or both responses are None".to_string()]),
        };

        ComparisonResult {
            endpoint: path.to_string(),
            method: method.to_string(),
            rust_response,
            java_response,
            is_equivalent,
            differences,
        }
    }

    /// 调用端点并返回 JSON 响应
    async fn call_endpoint(
        &self,
        url: &str,
        method: &str,
        headers: Option<std::collections::HashMap<String, String>>,
    ) -> Option<serde_json::Value> {
        let mut request = match method.to_uppercase().as_str() {
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => self.client.get(url),
        };

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(&key, &value);
            }
        }

        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    response.json::<serde_json::Value>().await.ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    /// 查找两个 JSON 值之间的差异
    fn find_differences(
        &self,
        rust: &serde_json::Value,
        java: &serde_json::Value,
    ) -> Vec<String> {
        let mut diffs = Vec::new();
        self.compare_values(rust, java, "", &mut diffs);
        diffs
    }

    /// 递归比较 JSON 值
    fn compare_values(
        &self,
        rust: &serde_json::Value,
        java: &serde_json::Value,
        path: &str,
        diffs: &mut Vec<String>,
    ) {
        match (rust, java) {
            (serde_json::Value::Object(rust_map), serde_json::Value::Object(java_map)) => {
                let all_keys: std::collections::HashSet<_> = rust_map
                    .keys()
                    .chain(java_map.keys())
                    .collect();
                for key in all_keys {
                    let field_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    match (rust_map.get(key), java_map.get(key)) {
                        (Some(r), Some(j)) => self.compare_values(r, j, &field_path, diffs),
                        (None, Some(_)) => {
                            diffs.push(format!("{}: missing in Rust", field_path));
                        }
                        (Some(_), None) => {
                            diffs.push(format!("{}: missing in Java", field_path));
                        }
                        _ => {}
                    }
                }
            }
            (serde_json::Value::Array(rust_arr), serde_json::Value::Array(java_arr)) => {
                if rust_arr.len() != java_arr.len() {
                    diffs.push(format!(
                        "{}: array length differs (Rust: {}, Java: {})",
                        path,
                        rust_arr.len(),
                        java_arr.len()
                    ));
                } else {
                    for (i, (r, j)) in rust_arr.iter().zip(java_arr.iter()).enumerate() {
                        let item_path = format!("{}[{}]", path, i);
                        self.compare_values(r, j, &item_path, diffs);
                    }
                }
            }
            _ => {
                if rust != java {
                    diffs.push(format!(
                        "{}: Rust={:?} vs Java={:?}",
                        path, rust, java
                    ));
                }
            }
        }
    }
}
