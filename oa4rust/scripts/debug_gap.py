import os, re

java_dir = os.path.abspath('../oa/o2server')
rust_dir = os.path.abspath('crates')

# 1. 提取所有 Rust 路由（按 crate 分组）
rust_routes_by_crate = {}
for root, dirs, files in os.walk(rust_dir):
    crate_name = os.path.basename(root)
    if crate_name == 'crates' or not os.path.exists(os.path.join(root, 'Cargo.toml')):
        continue
    for f in files:
        if f.endswith('.rs'):
            fpath = os.path.join(root, f)
            with open(fpath, 'r', encoding='utf-8') as fh:
                for line in fh:
                    m = re.search(r'\.route\("([^"]+)"', line)
                    if m:
                        r = m.group(1)
                        if not r.endswith('/health') and r != '/hello/world' and r.startswith('/jaxrs/'):
                            if crate_name not in rust_routes_by_crate:
                                rust_routes_by_crate[crate_name] = set()
                            rust_routes_by_crate[crate_name].add(r)

print('Rust routes by crate (sample):')
for crate, routes in list(rust_routes_by_crate.items())[:5]:
    print(f'  {crate}: {len(routes)} routes')
    for r in list(routes)[:5]:
        print(f'    {r}')

# 2. 测试一个 Java 模块
java_mod = 'x_processplatform_core_entity'
prefix = '/jaxrs/processplatform/core/entity'
java_paths = set()
for root, dirs, files in os.walk(java_dir):
    for f in files:
        if not f.endswith('.java'):
            continue
        jf = os.path.join(root, f)
        if java_mod not in jf.replace('\\', '/'):
            continue
        try:
            with open(jf, 'r', encoding='utf-8') as fh:
                lines = fh.readlines()
        except:
            continue
        cls = None
        for line in lines:
            m = re.match(r'^\s*@Path\(\"([^\"]+)\"\)', line)
            if m:
                val = m.group(1)
                indent = len(line) - len(line.lstrip())
                if indent == 0:
                    cls = val
                else:
                    if cls:
                        full = f'{cls}/{val}'
                    else:
                        full = val
                    if full.startswith(prefix):
                        java_paths.add(full)

print(f'\nJava paths for {java_mod}: {len(java_paths)}')
for p in sorted(java_paths)[:10]:
    print(f'  {p}')

# 3. 检查 Rust crate
rust_crate_name = 'processplatform_core_entity'
rust_routes = rust_routes_by_crate.get(rust_crate_name, set())
print(f'\nRust routes for {rust_crate_name}: {len(rust_routes)}')
for r in sorted(rust_routes)[:10]:
    print(f'  {r}')

# 4. 计算缺口
missing = java_paths - rust_routes
print(f'\nMissing: {len(missing)}')
for p in sorted(missing)[:10]:
    print(f'  {p}')
