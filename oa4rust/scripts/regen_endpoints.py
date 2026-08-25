#!/usr/bin/env python3
"""Regenerate endpoints.rs from all crate routes.rs files."""
import re
from pathlib import Path

BASE = Path(r'D:\WORKSPACE\fakeToys\oa4rust')
ENDPOINTS_FILE = BASE / 'tests' / 'behavior_comparison' / 'endpoints.rs'
CRATES_DIR = BASE / 'crates'

endpoints = []

for crate in sorted(CRATES_DIR.iterdir()):
    if not crate.is_dir():
        continue
    seen = set()
    for rs_name in ('routes.rs', 'lib.rs'):
        routes_file = crate / 'src' / rs_name
        if not routes_file.exists():
            continue
        content = routes_file.read_text(encoding='utf-8')
        # Find all .route("path", method(handler)) patterns (single & multi-line),
        # including chained multi-method registrations like get(a).put(b).delete(c):
        # emit one endpoint per (path, method) pair.
        for m in re.finditer(r'\.route\(\s*"([^"]+)"\s*,\s*((?:get|post|put|delete)\([^)]*\)(?:\s*\.\s*(?:get|post|put|delete)\([^)]*\))*)', content, re.DOTALL):
            path = m.group(1)
            handler_m = re.search(r'(?:get|post|put|delete)\(\s*(?:crate::)?(\w+)', m.group(2))
            handler = handler_m.group(1) if handler_m else ''
            for mm in re.finditer(r'(?:^|\.\s*)(get|post|put|delete)\(', m.group(2)):
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

# Write endpoints.rs
lines = ['/// 行为对比测试端点列表（自动生成）\n']
lines.append('/// 生成时间: ' + __import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M:%S'))
lines.append('use super::EndpointDef;\n')
lines.append('pub const ENDPOINTS: &[EndpointDef] = &[\n')

for e in endpoints:
    lines.append(f'    EndpointDef {{')
    lines.append(f'        crate_name: "{e["crate"]}",')
    lines.append(f'        method: "{e["method"]}",')
    lines.append(f'        rust_path: "{e["rust_path"]}",')
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
