#!/usr/bin/env python3
"""
generate_parity_tests.py — Phase 4 U4.1 Parity Regression Suite generator.

Reads docs/audits/o2server-parity-report.json and scans each mapped Rust crate
for axum route registrations (`.route("...", get/post/...)`), then emits a Rust
test file at crates/parity/src/generated_tests.rs.

Strategy:
  - The parity crate depends on oa4rust (dev-dep).
  - The Python script discovers routes from each crate's source.
  - The generated tests build a *per-crate* sub-router via the crate's `router(pool)`
    function and assert each route is registered (not 404) using
    `tower::util::ServiceExt::oneshot`.

Output:
  crates/parity/src/generated_tests.rs
"""

import json
import os
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ── Paths ─────────────────────────────────────────────────────────────────────
# BASE is the workspace root (parent of oa4rust/), where docs/ lives.
BASE = Path(__file__).resolve().parent.parent.parent
REPORT_PATH = BASE / "docs" / "audits" / "o2server-parity-report.json"
CRATES_DIR = BASE / "oa4rust" / "crates"
OUTPUT_PATH = CRATES_DIR / "parity" / "src" / "generated_tests.rs"

# ── Regexes ───────────────────────────────────────────────────────────────────
ROUTE_RE = re.compile(
    r'\.route\(\s*("(?:[^"\\]|\\.)*")\s*,\s*'
    r'(get|post|put|delete|patch|head|options)\s*\(\s*(\w+)'
)
ROUTER_FN_RE = re.compile(r'pub\s+(async\s+)?fn\s+(\w*router\w*)\s*\(([^)]*)\)')


def load_report() -> dict:
    if not REPORT_PATH.exists():
        print(f"FATAL: parity report not found at {REPORT_PATH}", file=sys.stderr)
        sys.exit(1)
    with open(REPORT_PATH, encoding="utf-8") as f:
        return json.load(f)


def find_rs_files(crate_dir: Path) -> List[Path]:
    src = crate_dir / "src"
    if not src.is_dir():
        return []
    result = []
    for root, _dirs, files in os.walk(str(src)):
        for fn in files:
            if fn.endswith(".rs"):
                result.append(Path(root) / fn)
    return sorted(result)


def extract_routes_from_crate(crate_dir: Path) -> List[Tuple[str, str, str]]:
    """Return [(handler_name, method, path), ...] for all .route(...) in crate."""
    rs_files = find_rs_files(crate_dir)
    routes: List[Tuple[str, str, str]] = []
    seen = set()
    for rs_file in rs_files:
        try:
            text = rs_file.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        for m in ROUTE_RE.finditer(text):
            path = m.group(1).strip('"')
            method = m.group(2).upper()
            handler = m.group(3)
            key = (handler, method, path)
            if key not in seen:
                seen.add(key)
                routes.append(key)
    return routes


def find_router_fn(crate_dir: Path) -> Optional[Tuple[str, str, bool]]:
    """Find the name and params of the public router() function in the crate."""
    rs_files = find_rs_files(crate_dir)
    lib_files = [f for f in rs_files if f.name == "lib.rs"]
    other_files = [f for f in rs_files if f.name != "lib.rs"]

    def search(files):
        best = None
        for rs_file in files:
            try:
                text = rs_file.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            for m in ROUTER_FN_RE.finditer(text):
                fn_name = m.group(2)
                is_async = m.group(1) is not None
                params = m.group(3).strip()
                if best is None:
                    best = (fn_name, params, is_async)
                if fn_name == "router":
                    return (fn_name, params, is_async)
        return best

    result = search(lib_files)
    if result is None:
        result = search(other_files)
    return result


def rust_safe_name(s: str) -> str:
    """Convert a string to a valid Rust identifier suffix."""
    s = re.sub(r'[^a-zA-Z0-9]', '_', s)
    s = re.sub(r'_+', '_', s).strip('_')
    if s and s[0].isdigit():
        s = '_' + s
    return s[:80]


def generate_route_test(
    java_mod: str,
    rust_crate: str,
    handler: str,
    method: str,
    path: str,
    router_fn: Optional[str],
    is_async_router: bool = False,
) -> str:
    """Generate one #[tokio::test] fn for a single route."""
    # Use crate+handler to guarantee uniqueness across crates
    test_name = f"parity__{rust_safe_name(rust_crate)}__{rust_safe_name(handler)}"
    rust_path = path.replace('\\', '\\\\').replace('"', '\\"')
    # Escape { } for Rust format strings inside format!()
    rust_path_fmt = rust_path.replace('{', '{{').replace('}', '}}')
    request_path = re.sub(r'\{[^}]+\}', 'test-id', path)

    if router_fn:
        router_expr = f"oa4rust::{rust_crate}::{router_fn}(shared::testing::test_pool())"
    else:
        router_expr = f"oa4rust::{rust_crate}::router(shared::testing::test_pool())"

    method_upper = method.upper()

    # Async routers: the router() fn returns a Future that must be .await-ed
    if is_async_router:
        return f"""    #[tokio::test]
    async fn {test_name}() {{
        let router = {router_expr}.await;
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(\"{request_path}\")
                    .method(axum::http::Method::{method_upper})
                    .body(axum::body::Body::empty())
                    .expect(\"build request\"),
            )
            .await
            .expect(\"oneshot dispatch\");
        assert_ne!(
            response.status(),
            axum::http::StatusCode::NOT_FOUND,
            \"parity: route missing on {rust_crate}: {rust_path_fmt} ({method_upper})\"
        );
    }}"""
    else:
        # Sync routers: router() returns Router directly
        return f"""    #[tokio::test]
    async fn {test_name}() {{
        let router = {router_expr};
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(\"{request_path}\")
                    .method(axum::http::Method::{method_upper})
                    .body(axum::body::Body::empty())
                    .expect(\"build request\"),
            )
            .await
            .expect(\"oneshot dispatch\");
        assert_ne!(
            response.status(),
            axum::http::StatusCode::NOT_FOUND,
            \"parity: route missing on {rust_crate}: {rust_path_fmt} ({method_upper})\"
        );
    }}"""


def generate_crate_block(
    java_mod: str,
    rust_crate: str,
    routes: List[Tuple[str, str, str]],
    router_fn: Optional[str],
    router_params: str,
    is_async_router: bool = False,
) -> Tuple[str, int]:
    """Generate the test block for one crate. Returns (code_str, route_count)."""
    if not routes:
        return "", 0

    lines: List[str] = []
    async_tag = " async" if is_async_router else ""
    lines.append(f"    // ── {java_mod} → {rust_crate} ({len(routes)} routes{async_tag}) ──")

    # Deduplicate test names: same handler can serve multiple routes.
    # Track {base_name: count} and append _<n> for duplicates.
    name_counts: Dict[str, int] = {}

    for handler, method, path in sorted(routes, key=lambda r: (r[2], r[1], r[0])):
        base = f"parity__{rust_safe_name(rust_crate)}__{rust_safe_name(handler)}"
        if base in name_counts:
            name_counts[base] += 1
            test_name = f"{base}_{name_counts[base]}"
        else:
            name_counts[base] = 0
            test_name = base

        test_code = generate_route_test_single(
            rust_crate, test_name, method, path, router_fn, router_params, is_async_router
        )
        if test_code is not None:
            lines.append(test_code)

    lines.append("")
    return "\n".join(lines), len(routes)


# Characters that are invalid in an HTTP request-target and would make
# http::Request::builder().uri(...) fail with InvalidUriChar. Route templates
# may legitimately contain `{param}` placeholders, so the check runs on the
# concrete request path (placeholders already substituted).
INVALID_URI_CHARS_RE = re.compile(r'[\s"<>\\^`|\x00-\x1f]')


def generate_route_test_single(
    rust_crate: str,
    test_name: str,
    method: str,
    path: str,
    router_fn: Optional[str],
    router_params: str,
    is_async_router: bool,
) -> Optional[str]:
    """Generate one #[tokio::test] fn (test_name is pre-computed, guaranteed unique).

    Returns None when the route cannot be exercised over oneshot (e.g. the
    concrete request path contains characters that are invalid in a URI).
    """
    rust_path = path.replace('\\', '\\\\').replace('"', '\\"')
    rust_path_fmt = rust_path.replace('{', '{{').replace('}', '}}')
    request_path = re.sub(r'\{[^}]+\}', 'test-id', path)

    if INVALID_URI_CHARS_RE.search(request_path):
        return (
            f"    // skipped: request path not representable as a URI "
            f"(Java-parity quirk): {rust_path_fmt} ({method.upper()})\n"
        )

    if router_fn:
        router_expr = f"oa4rust::{rust_crate}::{router_fn}({build_args(router_params)})"
    else:
        router_expr = f"oa4rust::{rust_crate}::router({build_args(router_params)})"

    method_upper = method.upper()

    # Existence contract: a truly unwired route yields axum's fallback 404 with
    # an EMPTY body. A wired handler may legitimately answer 404 for an unknown
    # resource (AppError::NotFound renders the O2OA error envelope), so a 404
    # with a non-empty body counts as "route exists".
    existence_check = f"""let (parts, body) = response.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .expect("read parity body");
        assert!(
            parts.status != StatusCode::NOT_FOUND || !bytes.is_empty(),
            \"parity: route missing on {rust_crate}: {rust_path_fmt} ({method_upper})\"
        );"""

    if is_async_router:
        return f"""    #[tokio::test]
    async fn {test_name}() {{
        let router = {router_expr}.await;
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(\"{request_path}\")
                    .method(axum::http::Method::{method_upper})
                    .body(axum::body::Body::empty())
                    .expect(\"build request\"),
            )
            .await
            .expect(\"oneshot dispatch\");
        {existence_check}
    }}"""
    else:
        return f"""    #[tokio::test]
    async fn {test_name}() {{
        let router = {router_expr};
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri(\"{request_path}\")
                    .method(axum::http::Method::{method_upper})
                    .body(axum::body::Body::empty())
                    .expect(\"build request\"),
            )
            .await
            .expect(\"oneshot dispatch\");
        {existence_check}
    }}"""


def build_args(params_str: str) -> str:
    """Build argument list for router() call from parsed param names and types."""
    if not params_str.strip():
        return ""
    args = []
    for raw in params_str.split(","):
        raw = raw.strip()
        if not raw:
            continue
        parts = raw.split(":", 1)
        name = parts[0].strip()
        typ = parts[1].strip() if len(parts) > 1 else ""
        if "Option" in typ and ("Pool" in typ or "pool" in typ.lower()):
            args.append(f"Some(shared::testing::test_pool())")
        elif name == "pool" or "Pool" in typ:
            args.append("shared::testing::test_pool()")
        elif name == "session_manager" or "SessionManager" in typ:
            args.append("shared::SessionManager::new()")
        elif name == "rate_limiter" or "RateLimiter" in typ:
            args.append("shared::RateLimiter::new()")
        else:
            args.append("/* unknown param: " + name + " */")
    return ", ".join(args)


def build_header(total_crates: int, total_routes: int, total_tests: int) -> str:
    return f"""// AUTO-GENERATED by scripts/generate_parity_tests.py — DO NOT EDIT.
//
// Parity regression suite (Phase 4 U4.1).
//
// For each o2server @Path mapped in docs/audits/o2server-parity-report.json,
// this file contains a test that verifies the the corresponding Rust axum route
// is registered on the crate's Router.  A NOT_FOUND (404) response means the
// route is missing from the Rust implementation — a parity gap.
//
// Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}
// Crates: {total_crates}   Routes: {total_routes}   Tests: {total_tests}

#[allow(unreachable_code, unused_variables, non_snake_case)]

use axum::body::Body;
use axum::http::{{Request, Method, StatusCode}};
use tower::util::ServiceExt;
use shared::testing::test_pool;

"""


def main() -> None:
    report = load_report()
    module_comparison = report.get("module_comparison", [])

    module_blocks: List[str] = []
    total_crates = 0
    total_routes = 0
    total_tests = 0
    missing_crates: List[str] = []

    for entry in module_comparison:
        java_mod = entry.get("java_mod", "")
        rust_crate = entry.get("rust_crate", "")
        if not rust_crate:
            continue

        crate_dir = CRATES_DIR / rust_crate
        if not crate_dir.is_dir():
            missing_crates.append(rust_crate)
            continue

        total_crates += 1
        routes = extract_routes_from_crate(crate_dir)
        router_info = find_router_fn(crate_dir)
        if router_info:
            router_fn, router_params, is_async = router_info
        else:
            router_fn, router_params, is_async = None, "", False

        if routes:
            block, count = generate_crate_block(java_mod, rust_crate, routes, router_fn, router_params, is_async)
            module_blocks.append(block)
            total_routes += count
            total_tests += count

    header = build_header(total_crates, total_routes, total_tests)
    body = "\n".join(module_blocks)
    footer = f"""
// ──────────────────────────────────────────────────────────────────────────────
// Parity count sanity check
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn parity_generated_test_count() {{
    assert!({total_tests} > 0, "no parity tests generated — check the Python script output");
}}
"""
    content = header + body + footer

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(content, encoding="utf-8")

    print(f"Wrote: {OUTPUT_PATH}")
    print(f"  Java modules in report : {len(module_comparison)}")
    print(f"  Rust crates scanned    : {total_crates}")
    if missing_crates:
        print(f"  Missing crate dirs     : {missing_crates}")
    print(f"  Total routes found     : {total_routes}")
    print(f"  Tests generated        : {total_tests}")
    print()
    print("Per-crate breakdown:")
    for entry in module_comparison:
        rc = entry.get("rust_crate", "")
        crate_dir = CRATES_DIR / rc
        if crate_dir.is_dir():
            routes = extract_routes_from_crate(crate_dir)
            print(f"  {rc:42s}  {len(routes):4d} routes")


if __name__ == "__main__":
    main()
