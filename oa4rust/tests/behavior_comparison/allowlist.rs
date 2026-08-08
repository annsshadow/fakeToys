use std::collections::{HashMap, HashSet};

/// Diff allowlist entry: a pair of equivalent field names (Rust name, Java name).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AllowlistEntry {
    pub rust_field: String,
    pub java_field: String,
    pub reason: Option<String>,
}

/// Diff allowlist loaded from YAML.
#[derive(Debug, Clone, Default)]
pub struct DiffAllowlist {
    /// Maps Java field name → set of equivalent Rust field names.
    pub rust_equivalents: HashMap<String, HashSet<String>>,
    /// Raw entries for reporting.
    pub entries: Vec<AllowlistEntry>,
}

impl DiffAllowlist {
    /// Load allowlist from a YAML file at the given path.
    pub fn from_yaml(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let entries: Vec<AllowlistEntry> = serde_yaml::from_str(&content)?;
        let mut allowlist = Self::default();
        for entry in &entries {
            allowlist
                .rust_equivalents
                .entry(entry.java_field.clone())
                .or_default()
                .insert(entry.rust_field.clone());
            allowlist.entries.push(entry.clone());
        }
        Ok(allowlist)
    }

    /// Create an empty allowlist.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Check whether a (rust_field, java_field) pair is allowlisted.
    pub fn is_allowed(&self, rust_field: &str, java_field: &str) -> bool {
        self.rust_equivalents
            .get(java_field)
            .map(|set| set.contains(rust_field))
            .unwrap_or(false)
            || self
                .rust_equivalents
                .get(rust_field)
                .map(|set| set.contains(java_field))
                .unwrap_or(false)
    }

    /// Given a Java field name, return the preferred Rust field name if allowlisted, else None.
    pub fn rust_equivalent(&self, java_field: &str) -> Option<&str> {
        self.rust_equivalents.get(java_field).and_then(|set| set.iter().next()).map(|s| s.as_str())
    }

    /// Given a Rust field name, return the preferred Java field name if allowlisted, else None.
    pub fn java_equivalent(&self, rust_field: &str) -> Option<&str> {
        self.rust_equivalents
            .iter()
            .find(|(_, set)| set.contains(rust_field))
            .map(|(k, _)| k.as_str())
    }
}
