use std::collections::HashMap;

use super::comparator::{ComparisonResult, ComparisonStatus};

/// Report format.
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Markdown,
    Json,
}

/// Comparison report grouped by crate.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComparisonReport {
    pub timestamp: String,
    pub java_service_url: String,
    pub total_endpoints: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub allowlist_entries: usize,
    /// Grouped results: crate_name → list of entries.
    pub by_crate: HashMap<String, Vec<EndpointReportEntry>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EndpointReportEntry {
    pub endpoint: String,
    pub method: String,
    pub status: String,
    pub rust_status: Option<u16>,
    pub java_status: Option<u16>,
    pub differences: Vec<String>,
}

impl ComparisonReport {
    pub fn new(java_service_url: impl Into<String>) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            java_service_url: java_service_url.into(),
            total_endpoints: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            allowlist_entries: 0,
            by_crate: HashMap::new(),
        }
    }

    pub fn with_allowlist_count(mut self, count: usize) -> Self {
        self.allowlist_entries = count;
        self
    }

    pub fn add_result(&mut self, result: ComparisonResult) {
        self.total_endpoints += 1;
        let status_str = match result.status {
            ComparisonStatus::Pass => {
                self.passed += 1;
                "PASS"
            }
            ComparisonStatus::Fail => {
                self.failed += 1;
                "FAIL"
            }
            ComparisonStatus::Skip => {
                self.skipped += 1;
                "SKIP"
            }
        };

        let entry = EndpointReportEntry {
            endpoint: result.endpoint,
            method: result.method,
            status: status_str.to_string(),
            rust_status: result.rust_status,
            java_status: result.java_status,
            differences: result.differences,
        };

        self.by_crate
            .entry(result.crate_name)
            .or_default()
            .push(entry);
    }

    /// Generate Markdown report grouped by crate.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Behavior Comparison Report\n\n");
        md.push_str(&format!("**Generated:** {}\n\n", self.timestamp));
        md.push_str(&format!("**Java service:** {}\n\n", self.java_service_url));
        md.push_str(&format!(
            "**Allowlist entries:** {}\n\n",
            self.allowlist_entries
        ));
        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "- **Total endpoints:** {}\n",
            self.total_endpoints
        ));
        md.push_str(&format!("- **Passed:** {}\n", self.passed));
        md.push_str(&format!("- **Failed:** {}\n", self.failed));
        md.push_str(&format!("- **Skipped:** {}\n", self.skipped));
        md.push('\n');

        if self.failed > 0 {
            md.push_str("## Failures\n\n");
            for (crate_name, entries) in &self.by_crate {
                let failures: Vec<_> = entries.iter().filter(|e| e.status == "FAIL").collect();
                if failures.is_empty() {
                    continue;
                }
                md.push_str(&format!("### Crate: `{}`\n\n", crate_name));
                md.push_str("| Method | Endpoint | Differences |\n");
                md.push_str("|--------|----------|-------------|\n");
                for entry in failures {
                    let diffs = entry
                        .differences
                        .iter()
                        .map(|d| d.replace('|', "\\|"))
                        .collect::<Vec<_>>()
                        .join("<br>");
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        entry.method, entry.endpoint, diffs
                    ));
                }
                md.push('\n');
            }
        }

        if self.skipped > 0 {
            md.push_str("## Skipped (Java unreachable)\n\n");
            md.push_str("| Crate | Method | Endpoint |\n");
            md.push_str("|-------|--------|----------|\n");
            for (crate_name, entries) in &self.by_crate {
                for entry in entries.iter().filter(|e| e.status == "SKIP") {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        crate_name, entry.method, entry.endpoint
                    ));
                }
            }
            md.push('\n');
        }

        md.push_str("## Full Results by Crate\n\n");
        let mut crate_names: Vec<_> = self.by_crate.keys().collect();
        crate_names.sort();

        for crate_name in crate_names {
            if let Some(entries) = self.by_crate.get(crate_name) {
                let (pass, fail, skip): (usize, usize, usize) = (
                    entries.iter().filter(|e| e.status == "PASS").count(),
                    entries.iter().filter(|e| e.status == "FAIL").count(),
                    entries.iter().filter(|e| e.status == "SKIP").count(),
                );
                md.push_str(&format!("### `{}` — {} pass, {} fail, {} skip\n\n", crate_name, pass, fail, skip));
                md.push_str("| # | Method | Endpoint | Status | Rust | Java | Differences |\n");
                md.push_str("|---|--------|----------|--------|-------|-------|-------------|\n");
                for (i, entry) in entries.iter().enumerate() {
                    let diffs = if entry.differences.is_empty() {
                        "—".to_string()
                    } else {
                        entry.differences.iter().map(|d| d.replace('|', "\\|")).collect::<Vec<_>>().join("<br>")
                    };
                    md.push_str(&format!(
                        "| {} | {} | {} | {} | {:?} | {:?} | {} |\n",
                        i + 1,
                        entry.method,
                        entry.endpoint,
                        entry.status,
                        entry.rust_status.unwrap_or(0),
                        entry.java_status.unwrap_or(0),
                        diffs
                    ));
                }
                md.push('\n');
            }
        }

        md
    }

    /// Generate JSON report.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
