import json, os, sys

jp = sys.argv[1]
cfg = json.load(open(jp, encoding='utf-8'))
target = cfg['target'].replace('\\', '/')
pairs = cfg['pairs']
content = open(target, encoding='utf-8', errors='ignore').read()
applied = 0
for i, (old, new) in enumerate(pairs):
    cnt = content.count(old)
    if cnt == 0:
        print(f"  WARN pair {i} NOT FOUND (len={len(old)}) :: {old[:60]!r}")
    elif cnt > 1:
        print(f"  WARN pair {i} AMBIGUOUS x{cnt} :: {old[:60]!r}")
    content = content.replace(old, new)
    applied += 1
tmp = target + '.tmp'
open(tmp, 'w', encoding='utf-8').write(content)
print(f"OK {os.path.basename(target)} applied={applied}/{len(pairs)} -> {tmp}")
