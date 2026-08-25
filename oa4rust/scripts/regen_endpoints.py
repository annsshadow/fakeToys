#!/usr/bin/env python3
"""Regenerate endpoints.rs from all crate route registrations.

Scans every `crates/*/src/**/*.rs` (not just routes.rs/lib.rs), so routes
registered in dedicated router files (e.g. u2_router.rs, auth submodules,
shared/router.rs, empower/router.rs) are captured. Path literals are parsed
escape-aware ({\"param\"} no longer truncates the match) and raw strings
r#"..."# are supported. Method detection accepts fully-qualified
`axum::routing::get(...)` forms as well as plain/chained get(a).put(b).

Exclusions (triaged in docs/audits/final-coverage-sweep.md §六):
- parity crate: test harness; its `.route(` occurrences are doc-comment examples
- mcp_server crate: standalone binary (/mcp has no Java counterpart)
- files named tests*/testing*: #[cfg(test)] modules and test helpers
"""
import re
from pathlib import Path

BASE = Path(r'D:\WORKSPACE\fakeToys\oa4rust')
ENDPOINTS_FILE = BASE / 'tests' / 'behavior_comparison' / 'endpoints.rs'
CRATES_DIR = BASE / 'crates'

SKIP_CRATES = {'parity', 'mcp_server'}

# r#"raw"# literal | escape-aware normal literal (two capture groups)
PATH_RE = r'r#*"([^"]*)"#|"((?:[^"\\]|\\.)*)"'
# a method call, optionally fully qualified: get(h) / axum::routing::get(h)
METHOD_CALL = r'(?:[A-Za-z_][A-Za-z0-9_]*::)*(?:get|post|put|delete)\([^)]*\)'
ROUTE_RE = re.compile(
    r'\.route\(\s*' + PATH_RE + r'\s*,\s*('
    + METHOD_CALL + r'(?:\s*\.\s*' + METHOD_CALL + r')*)',
    re.DOTALL,
)
# chained/plain methods: start-of-segment, ".method(" or "::method("
METHOD_SPLIT_RE = re.compile(r'(?:^|\.\s*|::)(get|post|put|delete)\(')
HANDLER_RE = re.compile(r'(?:get|post|put|delete)\(\s*(?:crate::)?(\w+)')


def is_test_file(name: str) -> bool:
    return name.startswith('tests') or name.startswith('testing')


def route_files(crate: Path):
    """All candidate .rs files, with historical precedence (routes.rs, lib.rs first)."""
    src = crate / 'src'
    if not src.is_dir():
        return []
    files = [p for p in src.rglob('*.rs') if not is_test_file(p.name)]
    ordered = []
    for pri in ('routes.rs', 'lib.rs'):
        p = src / pri
        if p in files:
            ordered.append(p)
    ordered.extend(sorted(p for p in files if p.name not in ('routes.rs', 'lib.rs')))
    return ordered


endpoints = []

for crate in sorted(CRATES_DIR.iterdir()):
    if not crate.is_dir() or crate.name in SKIP_CRATES:
        continue
    seen = set()
    for rs_file in route_files(crate):
        content = rs_file.read_text(encoding='utf-8')
        # Find all .route("path", method(handler)) patterns (single & multi-line),
        # including chained multi-method registrations like get(a).put(b).delete(c):
        # emit one endpoint per (path, method) pair.
        for m in ROUTE_RE.finditer(content):
            if m.group(1) is not None:  # raw string: no escape decoding
                path = m.group(1)
            else:  # normal string: decode escapes (\" -> ")
                path = re.sub(r'\\(.)', r'\1', m.group(2))
            handler_m = HANDLER_RE.search(m.group(3))
            handler = handler_m.group(1) if handler_m else ''
            for mm in METHOD_SPLIT_RE.finditer(m.group(3)):
                method = mm.group(1).upper()
                key = (path, method)
                if key in seen:
                    continue
                seen.add(key)
                endpoints.append({
                    'crate': crate.name,
                    'method': method,
                    'rust_path': path,
                    'handler': handler,
                })

# Sort by crate, then method, then path
endpoints.sort(key=lambda e: (e['crate'], e['method'], e['rust_path']))


def rs_str(s: str) -> str:
    """Escape a value for emission inside a Rust string literal."""
    return s.replace('\\', '\\\\').replace('"', '\\"')

# Write endpoints.rs
lines = ['/// 行为对比测试端点列表（自动生成）\n']
lines.append('/// 生成时间: ' + __import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
lines.append('use super::EndpointDef;\n')
lines.append('pub const ENDPOINTS: &[EndpointDef] = &[\n')

for e in endpoints:
    lines.append(f'    EndpointDef {{')
    lines.append(f'        crate_name: "{e["crate"]}",')
    lines.append(f'        method: "{e["method"]}",')
    lines.append(f'        rust_path: "{rs_str(e["rust_path"])}",')
    lines.append(f'        java_war: "",')
    lines.append(f'        java_action: "",')
    lines.append(f'        body: None,')
    lines.append(f'        requires_auth: false,')
    lines.append(f'    }},\n')

lines.append('];\n')

ENDPOINTS_FILE.write_text('\n'.join(lines), encoding='utf-8')
print(f'Wrote {len(endpoints)} endpoints to {ENDPOINTS_FILE}')

# Count unique paths
unique_paths = set(e['rust_path'] for e in endpoints)
print(f'Unique paths: {len(unique_paths)}')

# Count by crate
from collections import Counter
crate_counts = Counter(e['crate'] for e in endpoints)
print(f'Crates with endpoints: {len(crate_counts)}')
for c, n in crate_counts.most_common(10):
    print(f'  {c}: {n}')
