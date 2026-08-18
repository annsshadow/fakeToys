//! Parity comparison module (Phase 4 takeover-regression scaffold).
//!
//! RECORD mode (see `tests/parity_runner.rs`) captures oa4rust endpoint
//! responses into `tests/parity/corpus/oa4rust-<name>.json`.  This module
//! provides the field-by-field `diff` used to compare an oa4rust corpus
//! against a (future) o2server corpus.
//!
//! The o2server side is currently a TODO stub: when no o2server corpus is
//! supplied, `diff` returns `["o2server corpus not provided"]`.  The
//! comparison logic itself is real and compilable, so once an o2server
//! corpus exists the same function performs a genuine structural diff
//! (keys, array lengths, scalars, and types).

use std::path::{Path, PathBuf};

use serde_json::Value;

/// Sentinel written/expected when the o2server corpus has not been supplied.
pub const O2SERVER_CORPUS_MISSING: &str = "o2server corpus not provided";

/// Compare two corpus records field-by-field.
///
/// `a` is the oa4rust corpus (`{path, status, body}`); `b` is the o2server
/// corpus in the same shape.  Returns a list of human-readable differences.
///
/// Current stub behaviour: if `b` is the missing sentinel, returns a single
/// entry explaining the o2server corpus is not provided.  Otherwise a real
/// recursive structural comparison is performed over status, path, and body.
pub fn diff(a: &Value, b: &Value) -> Vec<String> {
    // TODO(stub): o2server corpus not yet captured in this environment.
    if is_missing_o2server(b) {
        return vec![O2SERVER_CORPUS_MISSING.to_string()];
    }

    let mut out = Vec::new();

    let a_status = a.get("status").and_then(|v| v.as_u64());
    let b_status = b.get("status").and_then(|v| v.as_u64());
    if a_status != b_status {
        out.push(format!(
            "status mismatch: oa4rust={:?} o2server={:?}",
            a_status, b_status
        ));
    }

    let a_path = a.get("path").and_then(|v| v.as_str());
    let b_path = b.get("path").and_then(|v| v.as_str());
    if a_path != b_path {
        out.push(format!(
            "path mismatch: oa4rust={:?} o2server={:?}",
            a_path, b_path
        ));
    }

    let a_body = a.get("body").unwrap_or(&Value::Null);
    let b_body = b.get("body").unwrap_or(&Value::Null);
    compare_values("body", a_body, b_body, &mut out);

    out
}

fn is_missing_o2server(b: &Value) -> bool {
    b.is_null() || (b.is_string() && b.as_str() == Some(O2SERVER_CORPUS_MISSING))
}

/// Recursive structural comparison between two JSON values.
fn compare_values(prefix: &str, a: &Value, b: &Value, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(ao), Value::Object(bo)) => {
            for (k, av) in ao {
                let child = format!("{}.{}", prefix, k);
                match bo.get(k) {
                    Some(bv) => compare_values(&child, av, bv, out),
                    None => out.push(format!("missing in o2server: {}", child)),
                }
            }
            for (k, _) in bo {
                if !ao.contains_key(k) {
                    out.push(format!("missing in oa4rust: {}.{}", prefix, k));
                }
            }
        }
        (Value::Array(aa), Value::Array(ba)) => {
            if aa.len() != ba.len() {
                out.push(format!(
                    "array length mismatch at {}: oa4rust={} o2server={}",
                    prefix,
                    aa.len(),
                    ba.len()
                ));
            }
            for (i, (av, bv)) in aa.iter().zip(ba.iter()).enumerate() {
                compare_values(&format!("{}[{}]", prefix, i), av, bv, out);
            }
        }
        (Value::String(as_), Value::String(bs)) => {
            if as_ != bs {
                out.push(format!(
                    "string mismatch at {}: oa4rust={:?} o2server={:?}",
                    prefix, as_, bs
                ));
            }
        }
        (Value::Number(an), Value::Number(bn)) => {
            if an != bn {
                out.push(format!(
                    "number mismatch at {}: oa4rust={} o2server={}",
                    prefix, an, bn
                ));
            }
        }
        (Value::Bool(ab), Value::Bool(bb)) => {
            if ab != bb {
                out.push(format!(
                    "bool mismatch at {}: oa4rust={} o2server={}",
                    prefix, ab, bb
                ));
            }
        }
        (Value::Null, Value::Null) => {}
        _ => {
            out.push(format!(
                "type mismatch at {}: oa4rust={:?} o2server={:?}",
                prefix, a, b
            ));
        }
    }
}

/// Load a recorded corpus record from `tests/parity/corpus/<prefix>-<name>.json`.
/// Returns `None` if the file does not exist.
pub fn load_corpus(prefix: &str, name: &str) -> Option<Value> {
    let path = corpus_path(prefix, name);
    let text = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

/// Compare an oa4rust corpus record against an o2server corpus record by name.
/// When the o2server corpus is absent, `diff` returns the stub message.
pub fn compare_corpora(name: &str) -> Vec<String> {
    let a = match load_corpus("oa4rust", name) {
        Some(v) => v,
        None => return vec![format!("oa4rust corpus not recorded: {}", name)],
    };
    let b = load_corpus("o2server", name)
        .unwrap_or_else(|| Value::String(O2SERVER_CORPUS_MISSING.to_string()));
    diff(&a, &b)
}

fn corpus_path(prefix: &str, name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/parity/corpus")
        .join(format!("{}-{}.json", prefix, name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stub_when_o2server_missing() {
        let a = json!({ "path": "/x", "status": 200, "body": {"ok": true} });
        let b = Value::String(O2SERVER_CORPUS_MISSING.to_string());
        let d = diff(&a, &b);
        assert_eq!(d, vec![O2SERVER_CORPUS_MISSING.to_string()]);
    }

    #[test]
    fn real_diff_detects_key_and_scalar_mismatch() {
        let a = json!({ "status": 200, "path": "/x", "body": { "id": 1, "name": "a" } });
        let b = json!({ "status": 200, "path": "/x", "body": { "id": 2, "extra": true } });
        let d = diff(&a, &b);
        let joined = d.join("\n");
        assert!(joined.contains("body.id"), "expected body.id number mismatch, got:\n{}", joined);
        assert!(joined.contains("missing in o2server: body.name"), "got:\n{}", joined);
        assert!(joined.contains("missing in oa4rust: body.extra"), "got:\n{}", joined);
    }

    #[test]
    fn real_diff_passes_when_equal() {
        let v = json!({ "status": 200, "path": "/x", "body": { "id": 1, "list": [1, 2] } });
        assert!(diff(&v, &v).is_empty());
    }
}
