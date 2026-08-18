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
        return ('FAIL', str(e))
    s = strip_code(t)
    s = re.sub(r'(?m)^#{1,6}\s.*$', ' ', s)
    s = re.sub(r'\[[^\]]*\]\([^)]*\)', ' ', s)
    s = re.sub(r'https?://\S+', ' ', s)
    cn = len(re.findall(r'[一-鿿]', s))
    total_chars = len(s)
    ratio = cn / total_chars if total_chars else 0.0
    en_words = re.findall(r'[A-Za-z]+', s)
    en_count = len(en_words)
    sz = os.path.getsize(path)
    return ('DATA', ratio, en_count, sz, cn)

need, skip_trans, skip_struct, fails = [], [], [], []
for f in files:
    r = analyze(f)
    if r[0] == 'FAIL':
        fails.append((f, r[1])); continue
    _, ratio, en_count, sz, cn = r
    if ratio >= 0.03:
        skip_trans.append(f)
    elif en_count < 8:
        skip_struct.append(f)
    else:
        need.append((f, ratio, en_count, sz))

print("TOTAL:", len(files))
print("SKIP already-translated (>=3% cn):", len(skip_trans))
print("SKIP structure/index (<8 en words):", len(skip_struct))
print("NEED translation:", len(need))
print("FAIL read:", len(fails))
print("---- NEED files (sorted by size) ----")
for f, ratio, en, sz in sorted(need, key=lambda x: -x[3]):
    print(f"{sz:>9}  cn%={ratio*100:4.1f}  en={en:3d}  {os.path.basename(f)}")
