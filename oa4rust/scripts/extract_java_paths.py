import os, re

java_dir = os.path.abspath('../oa/o2server')

# 提取单个 Java 文件的所有完整 @Path（类级别 + 方法级别拼接）
def extract_java_paths(jf):
    paths = []
    try:
        with open(jf, 'r', encoding='utf-8') as fh:
            lines = fh.readlines()
    except:
        return paths
    
    cls_path = None
    for i, line in enumerate(lines):
        # 匹配 @Path("...")
        m = re.match(r'^(\s*)@Path\(\"([^\"]+)\"\)', line)
        if m:
            indent = len(m.group(1))
            val = m.group(2)
            # 类级别：缩进较少（通常在 0-2 个空格）
            if indent <= 2:
                cls_path = val
            else:
                # 方法级别
                if cls_path:
                    full = f'{cls_path}/{val}'
                else:
                    full = val
                paths.append(full)
    return paths

# 测试：读取一个 Java 文件
test_file = os.path.join(java_dir, 'x_processplatform_assemble_designer/src/main/java/com/x/processplatform/assemble/designer/jaxrs/applicationdict/ApplicationDictAction.java')
if os.path.exists(test_file):
    paths = extract_java_paths(test_file)
    print(f'Test file: {test_file}')
    print(f'Paths found: {len(paths)}')
    for p in paths[:10]:
        print(f'  {p}')
else:
    print('Test file not found')

# 统计所有 Java 模块的 @Path 数量
print('\n=== Java module @Path counts ===')
from collections import Counter
module_counts = Counter()
for root, dirs, files in os.walk(java_dir):
    for f in files:
        if not f.endswith('.java'):
            continue
        jf = os.path.join(root, f)
        # 识别模块
        rel = jf.replace('\\', '/').replace(java_dir + '/', '')
        parts = rel.split('/')
        mod = None
        for p in parts:
            if p.startswith('x_'):
                mod = p
                break
        if not mod:
            continue
        paths = extract_java_paths(jf)
        module_counts[mod] += len(paths)

for mod, cnt in module_counts.most_common(30):
    print(f'{cnt:4d}  {mod}')
