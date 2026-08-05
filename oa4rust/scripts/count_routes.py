import os, re

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')
print(f'Java dir: {java_dir}')
print(f'Rust dir: {rust_dir}')
print(f'Java exists: {os.path.exists(java_dir)}')

# 1. 提取 Rust 路由
rust_routes = set()
for root, dirs, files in os.walk(rust_dir):
    for f in files:
        if f.endswith('.rs'):
            path = os.path.join(root, f)
            with open(path, 'r', encoding='utf-8') as fh:
                for line in fh:
                    m = re.search(r'\.route\("([^"]+)"', line)
                    if m:
                        r = m.group(1)
                        if not r.endswith('/health') and r != '/hello/world':
                            rust_routes.add(r)

# 2. 提取 Java 完整路径（类级别 + 方法级别拼接）
java_paths = []
for root, dirs, files in os.walk(java_dir):
    for f in files:
        if not f.endswith('.java'):
            continue
        jf = os.path.join(root, f)
        try:
            with open(jf, 'r', encoding='utf-8') as fh:
                lines = fh.readlines()
        except:
            continue
        cls = None
        for line in lines:
            m = re.match(r'\s*@Path\("([^"]+)"\)', line)
            if m:
                val = m.group(1)
                stripped = line.lstrip()
                indent_level = len(line) - len(stripped)
                if indent_level == 0 and stripped.startswith('@Path'):
                    cls = val
                elif indent_level > 0 and stripped.startswith('@Path'):
                    if cls:
                        java_paths.append(f'{cls}/{val}')
                    else:
                        java_paths.append(val)

java_unique = sorted(set(java_paths))
rust_sorted = sorted(rust_routes)

print(f'\nRust routes (business): {len(rust_sorted)}')
print(f'Java unique paths: {len(java_unique)}')

# 3. 模块前缀对比（前3段）
rust_prefixes = set()
for r in rust_sorted:
    parts = r.split('/')
    if len(parts) >= 3:
        rust_prefixes.add('/'.join(parts[:3]))

java_prefixes = set()
for j in java_unique:
    parts = j.split('/')
    if len(parts) >= 3:
        java_prefixes.add('/'.join(parts[:3]))

missing_prefixes = sorted(java_prefixes - rust_prefixes)
print(f'\nRust prefixes: {len(rust_prefixes)}')
print(f'Java prefixes: {len(java_prefixes)}')
print(f'Missing prefixes: {len(missing_prefixes)}')
for p in missing_prefixes:
    print(f'  {p}')

# 4. 按模块分组统计
print('\n=== Java modules with most endpoints ===')
from collections import Counter
java_mod_count = Counter()
for j in java_unique:
    parts = j.split('/')
    if len(parts) >= 3:
        java_mod_count['/'.join(parts[:3])] += 1

for mod, cnt in java_mod_count.most_common(30):
    status = 'OK' if mod in rust_prefixes else 'MISSING'
    print(f'  [{status}] {mod}: {cnt} endpoints')
