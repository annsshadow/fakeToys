import os, re

java_dir = os.path.abspath('../oa/o2server')

# 找一个包含 @Path("applicationdict") 的 Java 文件
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
        for line in lines:
            if '@Path("applicationdict")' in line:
                print(f'Found in: {jf}')
                for i, l in enumerate(lines[:50]):
                    print(f'{i+1}: {l.rstrip()}')
                raise SystemExit

print('Not found')
