import os, re

java_dir = os.path.abspath('../oa/o2server')

test_file = os.path.join(java_dir, 'x_processplatform_assemble_designer/src/main/java/com/x/processplatform/assemble/designer/jaxrs/applicationdict/ApplicationDictAction.java')

with open(test_file, 'r', encoding='utf-8') as fh:
    lines = fh.readlines()

print(f'Total lines: {len(lines)}')
for i, line in enumerate(lines):
    if '@Path' in line:
        stripped = line.lstrip()
        indent = len(line) - len(stripped)
        print(f'Line {i+1} (indent={indent}): {line.rstrip()}')
        m = re.match(r'^(\s*)@Path\(\"([^\"]+)\"\)', line)
        if m:
            print(f'  -> MATCH: class_path={m.group(2)}, indent={len(m.group(1))}')
        else:
            print(f'  -> NO MATCH')
