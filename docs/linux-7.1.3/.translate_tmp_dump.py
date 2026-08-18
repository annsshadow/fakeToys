import os, sys

base = 'D:/WORKSPACE/linux-7.1.3/docs/系统文档'
lines = [x.strip() for x in open(base + '/.translate_need.txt', encoding='utf-8').read().splitlines() if x.strip()]
start = int(sys.argv[1]) if len(sys.argv) > 1 else 0
n = int(sys.argv[2]) if len(sys.argv) > 2 else 20
batch = lines[start:start+n]
print("TOTAL", len(lines), "BATCH", start, "TO", start+len(batch))
for f in batch:
    print("=====FILE " + f + " =====")
    print(open(f, encoding='utf-8', errors='ignore').read())
    print("=====END " + f + " =====")
