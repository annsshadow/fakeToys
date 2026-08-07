import os, re
from collections import Counter

java_dir = os.path.abspath('../oa/o2server')

# 统计每个 Java 模块下的 @Path 数量
module_counts = Counter()
for root, dirs, files in os.walk(java_dir):
    for f in files:
        if not f.endswith('.java'):
            continue
        jf = os.path.join(root, f)
        try:
            with open(jf, 'r', encoding='utf-8') as fh:
                content = fh.read()
        except:
            continue
        parts = jf.replace('\\', '/').split('/')
        mod = None
        for p in parts:
            if p.startswith('x_') and p != 'x_base_core_project':
                mod = p
                break
        if not mod:
            continue
        paths = re.findall(r'@Path\("([^"]+)"\)', content)
        module_counts[mod] += len(paths)

print('Java module endpoint counts:')
for mod, cnt in module_counts.most_common(50):
    print(f'{cnt:4d}  {mod}')
