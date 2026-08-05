import os, re

java_dir = os.path.abspath('../oa/o2server')

# 测试单个文件
test_file = os.path.join(java_dir, 'x_processplatform_assemble_designer/src/main/java/com/x/processplatform/assemble/designer/jaxrs/applicationdict/ApplicationDictAction.java')

paths = []
with open(test_file, 'r', encoding='utf-8') as fh:
    lines = fh.readlines()

cls_path = None
for i, line in enumerate(lines):
    m = re.match(r'^(\s*)@Path\(\"([^\"]+)\"\)', line)
    if m:
        indent = len(m.group(1))
        val = m.group(2)
        print(f'Line {i+1}: indent={indent}, val={val}')
        if indent <= 2:
            cls_path = val
            print(f'  -> class_path = {cls_path}')
        else:
            if cls_path:
                full = f'{cls_path}/{val}'
            else:
                full = val
            paths.append(full)
            print(f'  -> full = {full}')

print(f'\nTotal paths: {len(paths)}')
for p in paths:
    print(f'  {p}')
