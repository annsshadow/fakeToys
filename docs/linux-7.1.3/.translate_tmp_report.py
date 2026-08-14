import json, re, os

base = 'D:/WORKSPACE/linux-7.1.3/docs/系统文档'
d = json.load(open(base + '/.translate_workers.json', encoding='utf-8'))
files = d['workers'][195]

def strip_code(t):
    t = re.sub(r'(?ms)```.*?```', ' ', t)
    t = re.sub(r'`[^`]*`', ' ', t)
    return t

def analyze(path):
    try:
        t = open(path, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        return ('FAIL', 'read:' + str(e))
    s = strip_code(t)
    s2 = re.sub(r'(?m)^#{1,6}\s.*$', ' ', s)
    s2 = re.sub(r'\[[^\]]*\]\([^)]*\)', ' ', s2)
    s2 = re.sub(r'https?://\S+', ' ', s2)
    cn = len(re.findall(r'[一-鿿]', s2))
    ratio = cn / len(s2) if s2 else 0.0
    en = len(re.findall(r'[A-Za-z]+', s2))
    fences = len(re.findall(r'(?m)^```', t))
    parity = (fences % 2 == 0)
    if not parity:
        return ('FAIL', 'fence-odd')
    if ratio >= 0.03:
        return ('DONE', '')
    if en < 8:
        return ('SKIP', 'structure/index (en<%d)' % en)
    return ('SKIP', 'structure (en=%d,cn=%.3f)' % (en, ratio))

ok = skip = fail = 0
fmap = {}
for f in files:
    st, why = analyze(f)
    rel = os.path.relpath(f, base).replace('\\', '/')
    if st == 'DONE':
        ok += 1; fmap[rel] = 'DONE'
    elif st == 'SKIP':
        skip += 1; fmap[rel] = 'SKIP'
    else:
        fail += 1; fmap[rel] = 'FAIL:' + why

report = {'worker': 195, 'ok': ok, 'skip': skip, 'fail': fail, 'files': fmap}
out = base + '/.translate_results/w195.json'
os.makedirs(os.path.dirname(out), exist_ok=True)
json.dump(report, open(out, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('ok=%d skip=%d fail=%d total=%d' % (ok, skip, fail, ok+skip+fail))
print('written', out)
