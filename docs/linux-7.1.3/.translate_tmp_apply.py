import json, os, re, sys

base = 'D:/WORKSPACE/linux-7.1.3/docs/系统文档'
batchfile = sys.argv[1] if len(sys.argv) > 1 else '.translate_tmp_batch.json'
data = json.load(open(batchfile, encoding='utf-8'))
results = {}
for path, content in data.items():
    try:
        tmp = path + '.tmp'
        with open(tmp, 'w', encoding='utf-8') as fh:
            fh.write(content)
        os.replace(tmp, path)
        # self-check: fence parity + chinese ratio
        t = open(path, encoding='utf-8', errors='ignore').read()
        fences = len(re.findall(r'(?m)^```', t))
        parity = fences % 2 == 0
        s = re.sub(r'(?ms)```.*?```', ' ', t)
        s = re.sub(r'`[^`]*`', ' ', s)
        s = re.sub(r'(?m)^#{1,6}\s.*$', ' ', s)
        s = re.sub(r'\[[^\]]*\]\([^)]*\)', ' ', s)
        s = re.sub(r'https?://\S+', ' ', s)
        cn = len(re.findall(r'[一-鿿]', s))
        ratio = cn / len(s) if s else 1.0
        if not parity:
            results[path] = 'FAIL:fence-odd'
        elif ratio < 0.03:
            results[path] = 'FAIL:cn-low(%s)' % round(ratio,3)
        else:
            results[path] = 'DONE'
    except Exception as e:
        results[path] = 'FAIL:' + str(e)
ok = sum(1 for v in results.values() if v=='DONE')
fail = sum(1 for v in results.values() if v.startswith('FAIL'))
print("batch ok=%d fail=%d" % (ok, fail))
for k,v in results.items():
    print(v, k)
json.dump(results, open(batchfile + '.result', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
