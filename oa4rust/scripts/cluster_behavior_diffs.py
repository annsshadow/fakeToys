#!/usr/bin/env python3
"""
cluster_behavior_diffs.py — 行为对比报告 FAIL 差异聚类工具（U1 / R9 / R-infrastructure）

从 behavior-report.md 提取 FAIL 端点差异，聚类为"候选改名对"，输出机读 TSV
和人审 Markdown，纳入版本控制。

用法:
    python oa4rust/scripts/cluster_behavior_diffs.py \
        --report target/debug/behavior-report.md \
        --out-dir target/

输出:
    target/diff_candidates.tsv   — 机器可读（pair、频次、示例端点）
    target/diff_candidates.md    — 人审报告（按频次排序、附证据摘录）
"""

import argparse
import os
import re
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path


# ── 正则表达式 ──────────────────────────────────────────────────────────────────

# FAIL 区表格行：| GET | /jaxrs/... | diff1<br>diff2 |
FAIL_ROW_RE = re.compile(
    r'^\|\s*(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\s*'
    r'\|\s*(/[^|]+?)\s*'
    r'\|\s*(.+?)\s*\|?\s*$'
)

# 差异字段：field_name: missing in Rust / missing in Java / type differs / array length differs
MISSING_IN_JAVA_RE = re.compile(r'^(.+?):\s*missing in Java\s*$', re.IGNORECASE)
MISSING_IN_RUST_RE = re.compile(r'^(.+?):\s*missing in Rust\s*$', re.IGNORECASE)
TYPE_DIFFERS_RE = re.compile(r'^(.+?):\s*(?:type differs|array length differs)\s*$', re.IGNORECASE)

# 类型差异子类
TYPE_MISMATCH_RE = re.compile(r'^(.+?):\s*type differs\s*$', re.IGNORECASE)
ARRAY_LEN_RE = re.compile(r'^(.+?):\s*array length differs\s*$', re.IGNORECASE)

#  crate 区标题
CRATE_HEADER_RE = re.compile(r'^### Crate: `(.+)`$')


def parse_failures(report_path: str) -> list[dict]:
    """Parse behavior-report.md, extract FAIL rows with field-level diffs.

    Returns list of:
        {"crate": str, "method": str, "endpoint": str, "diffs": [(field, category), ...]}
    """
    failures = []
    current_crate = "unknown"

    with open(report_path, encoding="utf-8") as f:
        lines = f.readlines()

    # State machine: look for "## Failures" section
    in_failures = False
    in_full_results = False

    for line in lines:
        stripped = line.strip()

        # Track section boundaries
        if stripped == "## Failures":
            in_failures = True
            in_full_results = False
            continue
        if stripped == "## Skipped (Java unreachable)":
            in_failures = False
            continue
        if stripped == "## Full Results by Crate":
            in_failures = False
            in_full_results = True
            continue
        if stripped.startswith("#") and in_full_results:
            # New top-level heading ends full results
            if stripped.startswith("# ") or stripped.startswith("## ") and not stripped.startswith("## Full"):
                in_full_results = False

        # Track crate headers
        crate_match = CRATE_HEADER_RE.match(stripped)
        if crate_match:
            current_crate = crate_match.group(1)
            continue

        # Parse FAIL rows in Failures section
        if in_failures:
            row_match = FAIL_ROW_RE.match(stripped)
            if not row_match:
                continue
            method = row_match.group(1)
            endpoint = row_match.group(2).strip()
            diffs_raw = row_match.group(3)

            # Parse individual diffs (separated by <br>)
            diffs = []
            for diff_str in diffs_raw.split("<br>"):
                diff_str = diff_str.strip()
                if not diff_str:
                    continue
                # Classify diff type
                if MISSING_IN_JAVA_RE.match(diff_str):
                    field = MISSING_IN_JAVA_RE.match(diff_str).group(1).strip()
                    diffs.append((field, "MISSING_IN_JAVA"))
                elif MISSING_IN_RUST_RE.match(diff_str):
                    field = MISSING_IN_RUST_RE.match(diff_str).group(1).strip()
                    diffs.append((field, "MISSING_IN_RUST"))
                elif TYPE_DIFFERS_RE.match(diff_str):
                    field = TYPE_DIFFERS_RE.match(diff_str).group(1).strip()
                    if ARRAY_LEN_RE.match(diff_str):
                        diffs.append((field, "ARRAY_LEN_DIFFERS"))
                    else:
                        diffs.append((field, "TYPE_DIFFERS"))

            if diffs:
                failures.append({
                    "crate": current_crate,
                    "method": method,
                    "endpoint": endpoint,
                    "diffs": diffs,
                })

        # Also parse FAIL rows in Full Results section (with rust_status/java_status columns)
        elif in_full_results:
            # Full results table: | # | Method | Endpoint | Status | Rust | Java | Differences |
            full_row_re = re.compile(
                r'^\|\s*\d+\s*'
                r'\|\s*(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\s*'
                r'\|\s*(/[^|]+?)\s*'
                r'\|\s*(PASS|FAIL|SKIP)\s*'
                r'\|.*?\|.*?\|\s*(.+?)\s*\|?\s*$'
            )
            row_match = full_row_re.match(stripped)
            if not row_match:
                continue
            method = row_match.group(1)
            endpoint = row_match.group(2).strip()
            status = row_match.group(3)
            diffs_raw = row_match.group(4)

            if status != "FAIL":
                continue

            diffs = []
            for diff_str in diffs_raw.split("<br>"):
                diff_str = diff_str.strip()
                if not diff_str or diff_str == "—":
                    continue
                if MISSING_IN_JAVA_RE.match(diff_str):
                    field = MISSING_IN_JAVA_RE.match(diff_str).group(1).strip()
                    diffs.append((field, "MISSING_IN_JAVA"))
                elif MISSING_IN_RUST_RE.match(diff_str):
                    field = MISSING_IN_RUST_RE.match(diff_str).group(1).strip()
                    diffs.append((field, "MISSING_IN_RUST"))
                elif TYPE_DIFFERS_RE.match(diff_str):
                    field = TYPE_DIFFERS_RE.match(diff_str).group(1).strip()
                    if ARRAY_LEN_RE.match(diff_str):
                        diffs.append((field, "ARRAY_LEN_DIFFERS"))
                    else:
                        diffs.append((field, "TYPE_DIFFERS"))

            if diffs:
                failures.append({
                    "crate": current_crate,
                    "method": method,
                    "endpoint": endpoint,
                    "diffs": diffs,
                })

    return failures


def cluster_diffs(failures: list[dict]) -> dict:
    """Cluster field diffs into candidate rename pairs.

    Strategy:
    1. For each endpoint, pair MISSING_IN_JAVA fields with MISSING_IN_RUST fields
       → candidate rename pairs (same endpoint, both sides missing something)
    2. Aggregate pairs across all endpoints → count frequency + collect example endpoints
    3. TYPE_DIFFERS entries are NOT paired (structural differences)

    Returns:
        {
            "rename_pairs": [(field_a, field_b, count, [endpoint_examples]), ...],
            "single_missing_java": [(field, count, [endpoint_examples]), ...],
            "single_missing_rust": [(field, count, [endpoint_examples]), ...],
            "type_differs": [(field, count, [endpoint_examples]), ...],
            "total_failures": int,
            "total_diffs": int,
        }
    """
    # Track: pair_key → (field_a, field_b, Counter(endpoints), set of examples)
    pair_counter: dict[str, tuple[str, str, Counter, list[str]]] = {}
    single_java: dict[str, tuple[Counter, list[str]]] = {}
    single_rust: dict[str, tuple[Counter, list[str]]] = {}
    type_diff: dict[str, tuple[Counter, list[str]]] = {}

    total_diffs = 0

    for entry in failures:
        endpoint = f"{entry['method']} {entry['endpoint']}"
        java_fields = []
        rust_fields = []
        type_fields = []

        for field, category in entry["diffs"]:
            total_diffs += 1
            if category == "MISSING_IN_JAVA":
                java_fields.append(field)
            elif category == "MISSING_IN_RUST":
                rust_fields.append(field)
            else:
                type_fields.append((field, category))

        # Pair Java-missing with Rust-missing (same endpoint → candidate rename)
        for jf in java_fields:
            for rf in rust_fields:
                # Sort pair key for dedup (A,B) == (B,A)
                pair_key = "|".join(sorted([jf, rf]))
                if pair_key not in pair_counter:
                    pair_counter[pair_key] = (jf, rf, Counter(), [])
                field_a, field_b, counter, examples = pair_counter[pair_key]
                counter[endpoint] += 1
                if len(examples) < 3 and endpoint not in examples:
                    examples.append(endpoint)
                pair_counter[pair_key] = (field_a, field_b, counter, examples)

        # Track single missing fields
        for jf in java_fields:
            if jf not in single_java:
                single_java[jf] = (Counter(), [])
            counter, examples = single_java[jf]
            counter[endpoint] += 1
            if len(examples) < 3 and endpoint not in examples:
                examples.append(endpoint)
            single_java[jf] = (counter, examples)

        for rf in rust_fields:
            if rf not in single_rust:
                single_rust[rf] = (Counter(), [])
            counter, examples = single_rust[rf]
            counter[endpoint] += 1
            if len(examples) < 3 and endpoint not in examples:
                examples.append(endpoint)
            single_rust[rf] = (counter, examples)

        for field, category in type_fields:
            if field not in type_diff:
                type_diff[field] = (Counter(), [])
            counter, examples = type_diff[field]
            counter[endpoint] += 1
            if len(examples) < 3 and endpoint not in examples:
                examples.append(endpoint)
            type_diff[field] = (counter, examples)

    # Build sorted results
    def sort_key(item):
        field, (counter, examples) = item
        return -sum(counter.values())

    rename_pairs = []
    for pair_key, (field_a, field_b, counter, examples) in pair_counter.items():
        total = sum(counter.values())
        rename_pairs.append((field_a, field_b, total, examples))

    rename_pairs.sort(key=lambda x: -x[2])

    single_java_list = [(f, sum(c.values()), e) for f, (c, e) in sorted(single_java.items(), key=sort_key)]
    single_rust_list = [(f, sum(c.values()), e) for f, (c, e) in sorted(single_rust.items(), key=sort_key)]
    type_diff_list = [(f, sum(c.values()), e) for f, (c, e) in sorted(type_diff.items(), key=sort_key)]

    return {
        "rename_pairs": rename_pairs,
        "single_missing_java": single_java_list,
        "single_missing_rust": single_rust_list,
        "type_differs": type_diff_list,
        "total_failures": len(failures),
        "total_diffs": total_diffs,
    }


def write_tsv(clustered: dict, out_path: str) -> None:
    """Write machine-readable TSV output."""
    lines = []
    lines.append("# category\tfield_a\tfield_b\tfreq\texample_endpoints")
    lines.append("# type: rename_pair")
    for field_a, field_b, freq, examples in clustered["rename_pairs"]:
        lines.append(f"rename_pair\t{field_a}\t{field_b}\t{freq}\t{' ; '.join(examples[:3])}")

    lines.append("# type: single_missing_java")
    for field, freq, examples in clustered["single_missing_java"]:
        lines.append(f"single_missing_java\t{field}\t\t{freq}\t{' ; '.join(examples[:3])}")

    lines.append("# type: single_missing_rust")
    for field, freq, examples in clustered["single_missing_rust"]:
        lines.append(f"single_missing_rust\t\t{field}\t{freq}\t{' ; '.join(examples[:3])}")

    lines.append("# type: type_differs")
    for field, freq, examples in clustered["type_differs"]:
        lines.append(f"type_differs\t{field}\t\t{freq}\t{' ; '.join(examples[:3])}")

    with open(out_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def write_markdown(clustered: dict, out_path: str, report_path: str) -> None:
    """Write human-reviewable Markdown output."""
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
    total_fail = clustered["total_failures"]
    total_diffs = clustered["total_diffs"]
    pair_count = len(clustered["rename_pairs"])

    md = []
    md.append(f"# Behavior Diff Clustering Report")
    md.append(f"")
    md.append(f"Generated by: `oa4rust/scripts/cluster_behavior_diffs.py`")
    md.append(f"")
    md.append(f"**Source:** `{report_path}`")
    md.append(f"**Generated at:** {now}")
    md.append(f"")
    md.append(f"## Summary")
    md.append(f"")
    md.append(f"- **Total FAIL endpoints:** {total_fail}")
    md.append(f"- **Total field-level diffs:** {total_diffs}")
    md.append(f"- **Candidate rename pairs:** {pair_count}")
    md.append(f"")
    md.append(f"---")
    md.append(f"")

    # Candidate Rename Pairs
    md.append(f"## Candidate Rename Pairs ({pair_count} unique)")
    md.append(f"")
    md.append(f"Pairs of fields that co-occur as missing on the same endpoint —")
    md.append(f"potential equivalent field names across implementations.")
    md.append(f"")
    md.append(f"| Rank | Field A (missing in Java) | Field B (missing in Rust) | Freq | Example Endpoints |")
    md.append(f"|------|--------------------------|--------------------------|------|-------------------|")

    for i, (field_a, field_b, freq, examples) in enumerate(clustered["rename_pairs"], 1):
        ex_str = "; ".join(examples[:3])
        md.append(f"| {i} | `{field_a}` | `{field_b}` | {freq} | {ex_str} |")

    md.append(f"")

    # Single Missing in Java
    md.append(f"## Single Missing in Java ({len(clustered['single_missing_java'])} unique)")
    md.append(f"")
    md.append(f"Fields present in Rust response but absent in Java.")
    md.append(f"")
    md.append(f"| Rank | Field (missing in Java) | Freq | Example Endpoints |")
    md.append(f"|------|------------------------|------|-------------------|")

    for i, (field, freq, examples) in enumerate(clustered["single_missing_java"][:100], 1):
        ex_str = "; ".join(examples[:3])
        md.append(f"| {i} | `{field}` | {freq} | {ex_str} |")

    md.append(f"")

    # Single Missing in Rust
    md.append(f"## Single Missing in Rust ({len(clustered['single_missing_rust'])} unique)")
    md.append(f"")
    md.append(f"Fields present in Java response but absent in Rust.")
    md.append(f"")
    md.append(f"| Rank | Field (missing in Rust) | Freq | Example Endpoints |")
    md.append(f"|------|------------------------|------|-------------------|")

    for i, (field, freq, examples) in enumerate(clustered["single_missing_rust"][:100], 1):
        ex_str = "; ".join(examples[:3])
        md.append(f"| {i} | `{field}` | {freq} | {ex_str} |")

    md.append(f"")

    # Type Differs
    md.append(f"## Type/Shape Differs ({len(clustered['type_differs'])} unique)")
    md.append(f"")
    md.append(f"Fields with differing types or array lengths between implementations.")
    md.append(f"")
    md.append(f"| Rank | Field | Freq | Example Endpoints |")
    md.append(f"|------|-------|------|-------------------|")

    for i, (field, freq, examples) in enumerate(clustered["type_differs"][:50], 1):
        ex_str = "; ".join(examples[:3])
        md.append(f"| {i} | `{field}` | {freq} | {ex_str} |")

    md.append(f"")
    md.append(f"---")
    md.append(f"")
    md.append(f"*Note: Rename pairs are ranked by total endpoint frequency (cross-endpoint aggregation).*")
    md.append(f"*Type differs entries are listed separately and NOT included in rename pair clustering.*")

    with open(out_path, "w", encoding="utf-8") as f:
        f.write("\n".join(md) + "\n")


def self_test() -> bool:
    """Validate script against synthetic data."""
    synthetic_report = """\
# Behavior Comparison Report

**Generated:** 2026-08-28T00:00:00Z
**Java service:** http://localhost:8080
**Allowlist entries:** 0

## Summary

- **Total endpoints:** 10
- **Passed:** 4
- **Failed:** 3
- **Skipped:** 3

## Failures

### Crate: `processplatform_assemble_surface`

| Method | Endpoint | Differences |
|--------|----------|-------------|
| GET | /jaxrs/processplatform/assemble/surface/work/list | data: missing in Java<br>prompt: missing in Rust |
| POST | /jaxrs/processplatform/assemble/surface/attachment/update/{id} | data: missing in Java<br>status: missing in Rust<br>url: missing in Rust |

### Crate: `cms_assemble_control`

| Method | Endpoint | Differences |
|--------|----------|-------------|
| GET | /jaxrs/cms/assemble/control/document/{id} | count: type differs<br>name: missing in Java |

## Skipped (Java unreachable)

| Crate | Method | Endpoint |
|-------|--------|----------|
| ... | ... | ... |
"""
    import tempfile
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False, encoding="utf-8") as f:
        f.write(synthetic_report)
        tmp_path = f.name

    failures = parse_failures(tmp_path)
    os.unlink(tmp_path)

    # Should find 2 endpoints in processplatform + 1 in cms = 3 failures
    assert len(failures) == 3, f"Expected 3 failures, got {len(failures)}"

    # Check rename pair: data↔prompt should appear on work/list
    clustered = cluster_diffs(failures)
    pair_fields = {(a, b) for a, b, _, _ in clustered["rename_pairs"]}
    assert ("data", "prompt") in pair_fields or ("prompt", "data") in pair_fields, \
        f"Expected data↔prompt pair, got {pair_fields}"

    # Check data↔status pair
    assert ("data", "status") in pair_fields or ("status", "data") in pair_fields, \
        f"Expected data↔status pair, got {pair_fields}"

    # Check type differs: count should be in type_differs
    type_fields = {f for f, _, _ in clustered["type_differs"]}
    assert "count" in type_fields, f"Expected count in type_differs, got {type_fields}"

    # Check total counts
    assert clustered["total_failures"] == 3
    assert clustered["total_diffs"] >= 5  # 2+3 diffs across 3 endpoints

    print("[self_test] PASS: all assertions OK")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Cluster behavior comparison FAIL diffs into candidate rename pairs"
    )
    parser.add_argument(
        "--report", "-r",
        default="target/debug/behavior-report.md",
        help="Path to behavior-report.md (default: target/debug/behavior-report.md)"
    )
    parser.add_argument(
        "--out-dir", "-o",
        default="target",
        help="Output directory (default: target)"
    )
    parser.add_argument(
        "--format", "-f",
        default="tsv,md",
        help="Output formats: tsv, md, or both (default: tsv,md)"
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run internal self-test and exit"
    )
    args = parser.parse_args()

    if args.self_test:
        ok = self_test()
        return 0 if ok else 1

    report_path = Path(args.report)
    if not report_path.exists():
        print(f"ERROR: report file not found: {report_path}", file=sys.stderr)
        return 1

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    formats = [f.strip() for f in args.format.split(",")]

    print(f"[cluster] Reading {report_path} ...")
    failures = parse_failures(str(report_path))
    print(f"[cluster] Found {len(failures)} FAIL endpoints")

    if not failures:
        print("[cluster] No FAIL entries found — writing empty output and exiting 0")
        if "tsv" in formats:
            write_tsv(
                {"rename_pairs": [], "single_missing_java": [], "single_missing_rust": [], "type_differs": [], "total_failures": 0, "total_diffs": 0},
                str(out_dir / "diff_candidates.tsv"),
            )
        if "md" in formats:
            write_markdown(
                {"rename_pairs": [], "single_missing_java": [], "single_missing_rust": [], "type_differs": [], "total_failures": 0, "total_diffs": 0},
                str(out_dir / "diff_candidates.md"),
                str(report_path),
            )
        return 0

    print(f"[cluster] Clustering diffs ...")
    clustered = cluster_diffs(failures)
    print(f"[cluster] {clustered['total_diffs']} total diffs → "
          f"{len(clustered['rename_pairs'])} rename pairs, "
          f"{len(clustered['single_missing_java'])} single-missing-Java, "
          f"{len(clustered['single_missing_rust'])} single-missing-Rust, "
          f"{len(clustered['type_differs'])} type-differs")

    if "tsv" in formats:
        tsv_path = out_dir / "diff_candidates.tsv"
        write_tsv(clustered, str(tsv_path))
        print(f"[cluster] TSV written to {tsv_path}")

    if "md" in formats:
        md_path = out_dir / "diff_candidates.md"
        write_markdown(clustered, str(md_path), str(report_path))
        print(f"[cluster] Markdown written to {md_path}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
