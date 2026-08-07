use serde::{Deserialize, Serialize};

/// 报告格式
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Markdown,
    Json,
    Html,
}

/// 对比报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub timestamp: String,
    pub total_endpoints: usize,
    pub equivalent_count: usize,
    pub different_count: usize,
    pub error_count: usize,
    pub results: Vec<EndpointReportEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointReportEntry {
    pub endpoint: String,
    pub method: String,
    pub status: String,
    pub differences: Vec<String>,
}

impl ComparisonReport {
    pub fn new() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_endpoints: 0,
            equivalent_count: 0,
            different_count: 0,
            error_count: 0,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: super::ComparisonResult) {
        self.total_endpoints += 1;
        let status = if result.rust_response.is_none() || result.java_response.is_none() {
            self.error_count += 1;
            "error"
        } else if result.is_equivalent {
            self.equivalent_count += 1;
            "equivalent"
        } else {
            self.different_count += 1;
            "different"
        };

        self.results.push(EndpointReportEntry {
            endpoint: result.endpoint,
            method: result.method,
            status: status.to_string(),
            differences: result.differences,
        });
    }

    /// 生成 Markdown 格式报告
    pub fn to_markdown(&self) -> String {
        let mut md = format!(
            "# Behavior Comparison Report\n\n**Generated:** {}\n\n",
            self.timestamp
        );
        md.push_str(&format!(
            "**Summary:** {} endpoints tested, {} equivalent, {} different, {} errors\n\n",
            self.total_endpoints, self.equivalent_count, self.different_count, self.error_count
        ));

        if self.different_count > 0 {
            md.push_str("## Differences\n\n");
            for entry in &self.results {
                if entry.status == "different" {
                    md.push_str(&format!(
                        "### {} {}\n\n",
                        entry.method, entry.endpoint
                    ));
                    for diff in &entry.differences {
                        md.push_str(&format!("- {}\n", diff));
                    }
                    md.push('\n');
                }
            }
        }

        if self.error_count > 0 {
            md.push_str("## Errors\n\n");
            for entry in &self.results {
                if entry.status == "error" {
                    md.push_str(&format!("- {} {}\n", entry.method, entry.endpoint));
                }
            }
            md.push('\n');
        }

        md
    }

    /// 生成 JSON 格式报告
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for ComparisonReport {
    fn default() -> Self {
        Self::new()
    }
}
