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
# Java reference source tree (monorepo sibling); when absent the generator
# degrades gracefully and leaves java_war/java_action empty (all comparisons SKIP).
O2SERVER_DIR = BASE.parent / 'oa' / 'o2server'

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


# ──────────────────────────────────────────────────────────────────────────────
# Java side mapping: extract JAXRS endpoints from the o2server source tree and
# match Rust endpoints to their Java counterparts (war + action path).
#
# Matching semantics (case-insensitive, path params normalized to {}):
# - candidate rust paths: strip 0..3 leading module segments after /jaxrs
# - candidate java paths: full class@Path/method@Path, plus variants with the
#   first/second segment (class-level prefix like "surface/") dropped
# - mock variant suffixes translate methods (O2OA MockServletFilter):
#   DELETE .../mockdeletetoget is served as GET; PUT .../mockputtopost as POST
# - strict suffix fallback: one side fully consumed, >=3 segments, equal param
#   count (guards against shape mismatches)
# Unmatched endpoints keep java_war empty and are SKIPped by behavior_compare
# (Rust-only extensions or generator artifacts have no Java counterpart).
# ──────────────────────────────────────────────────────────────────────────────

JAVA_HTTP_RE = re.compile(r'@(GET|POST|PUT|DELETE)\s*\n(?:\s*@Path\s*\(\s*"([^"]*)"\s*\)\s*\n)?')
JAVA_CLASS_PATH_RE = re.compile(
    r'@Path\s*\(\s*"([^"]*)"\s*\)[^{};]*?public\s+class\s+\w+', re.S)

# Rust crates reorganized from differently-named Java wars
WAR_ALIAS = {
    'personal': 'x_organization_assemble_personal',
    'auth': 'x_organization_assemble_authentication',
    'control': 'x_organization_assemble_control',
    'program_center': 'x_program_center',
    'program_init': 'x_program_init',
    'portal': 'x_portal_assemble_surface',
    'message': 'x_message_assemble_communicate',
}
# crates without a deployable JAXRS war (entity jars, Rust-only modules)
NO_WAR_CRATES = {
    'base', 'console', 'realtime', 'shared', 'signature', 'preview',
    'personal_extend', 'empower', 'express', 'message', 'query_service',
    'query_express', 'cms_control', 'cms_express', 'process_bam',
    'process_designer', 'process_express', 'process_surface',
}


def seg_norm(p):
    p = re.sub(r'\{[^}]*\}', '{}', p)
    return tuple(s for s in p.strip('/').split('/') if s)


def build_java_index(o2_root):
    """(war, method) -> {casefold_seg_tuple: original action string}"""
    index = {}
    for mod in sorted(o2_root.iterdir()):
        if not (mod.is_dir() and mod.name.startswith('x_')):
            continue
        for jf in mod.rglob('*.java'):
            try:
                text = jf.read_text(encoding='utf-8', errors='replace')
            except OSError:
                continue
            cm = JAVA_CLASS_PATH_RE.search(text)
            if not cm:
                continue
            cls_path = cm.group(1).strip('/')
            war = mod.name
            body = text[cm.end():]
            for mm in JAVA_HTTP_RE.finditer(body):
                segs = seg_norm('/'.join(x for x in (cls_path, (mm.group(2) or '').strip('/')) if x))
                if not segs:
                    continue
                bucket = index.setdefault((war, mm.group(1)), {})
                cf = tuple(s.casefold() for s in segs)
                action = '/' + '/'.join(segs)
                # prefer shortest original for a casefolded key (fewer prefixes)
                if cf not in bucket or len(action) < len(bucket[cf]):
                    bucket[cf] = action
                for v in ({segs, segs[1:], segs[2:]} - {segs}):
                    vcf = tuple(s.casefold() for s in v)
                    vaction = '/' + '/'.join(v)
                    if vcf not in bucket:
                        bucket[vcf] = vaction
    return index


def resolve_war(crate_name, o2_dirs):
    if crate_name.endswith(('_core_entity', '_core_express')) or crate_name in NO_WAR_CRATES:
        return ''
    if crate_name in WAR_ALIAS:
        return WAR_ALIAS[crate_name]
    for cand in (f'x_{crate_name}', f'x_{crate_name}_assemble_control',
                 f'x_{crate_name}_service_processing'):
        if cand in o2_dirs:
            return cand
    return ''


def match_java_action(java_index, war, method, rust_path):
    """Return the Java action path for a Rust endpoint, or '' when unmatched."""
    after = seg_norm(re.sub(r'^/jaxrs', '', rust_path))
    tail = after[-1] if after else ''
    j_method = {'mockdeletetoget': 'GET', 'mockputtopost': 'POST'}.get(tail, method)
    bucket = java_index.get((war, j_method)) or java_index.get((war, method))
    if not bucket:
        return ''
    for n in range(4):
        rc = after[n:] if len(after) > n else after
        rcf = tuple(s.casefold() for s in rc)
        if rcf in bucket:
            return bucket[rcf]
    # strict suffix fallback: one side fully consumed, >=3 segs, equal params
    params_r = sum(1 for s in after if s == '{}')
    best, bestk = '', 0
    for jcf, jo in bucket.items():
        k = 0
        while k < min(len(after), len(jcf)) and after[-1-k].casefold() == jcf[-1-k]:
            k += 1
        if (k == min(len(after), len(jcf)) and k >= 3 and k > bestk
                and sum(1 for s in jcf if s == '{}') == params_r):
            best, bestk = jo, k
    return best


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

# Java mapping (degrades to empty war/action when o2server source is absent)
java_index, o2_dirs = {}, set()
if O2SERVER_DIR.is_dir():
    print(f'Scanning Java endpoints under {O2SERVER_DIR} ...')
    o2_dirs = {d.name for d in O2SERVER_DIR.iterdir()
               if d.is_dir() and d.name.startswith('x_')}
    java_index = build_java_index(O2SERVER_DIR)
    n_java = sum(len(b) for b in java_index.values())
    print(f'Java index: {n_java} action variants across {len(java_index)} (war, method) buckets')
else:
    print(f'WARNING: {O2SERVER_DIR} not found; java_war/java_action left empty')

for e in endpoints:
    war = resolve_war(e['crate'], o2_dirs) if java_index else ''
    action = match_java_action(java_index, war, e['method'], e['rust_path']) if war else ''
    e['java_war'] = war if action else ''
    e['java_action'] = action


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
    lines.append(f'        java_war: "{rs_str(e["java_war"])}",')
    lines.append(f'        java_action: "{rs_str(e["java_action"])}",')
    lines.append(f'        body: None,')
    lines.append(f'        requires_auth: false,')
    lines.append(f'    }},\n')

lines.append('];\n')

ENDPOINTS_FILE.write_text('\n'.join(lines), encoding='utf-8')
print(f'Wrote {len(endpoints)} endpoints to {ENDPOINTS_FILE}')

mapped = sum(1 for e in endpoints if e['java_war'])
print(f'Mapped to Java: {mapped} (SKIP when unreachable: {len(endpoints) - mapped})')

# Count unique paths
unique_paths = set(e['rust_path'] for e in endpoints)
print(f'Unique paths: {len(unique_paths)}')

# Count by crate
from collections import Counter
crate_counts = Counter(e['crate'] for e in endpoints)
print(f'Crates with endpoints: {len(crate_counts)}')
for c, n in crate_counts.most_common(10):
    print(f'  {c}: {n}')
