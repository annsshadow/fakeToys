import json, re, os

base = 'D:/WORKSPACE/linux-7.1.3/docs/系统文档'
d = json.load(open(base + '/.translate_workers.json', encoding='utf-8'))
files = d['workers'][195]

def strip_code(t):
    t = re.sub(r'(?ms)```.*?```', ' ', t)
    t = re.sub(r'`[^`]*`', ' ', t)
    return t

def needs(path):
    try:
        t = open(path, encoding='utf-8', errors='ignore').read()
    except Exception:
        return False
    s = strip_code(t)
    s = re.sub(r'(?m)^#{1,6}\s.*$', ' ', s)
    s = re.sub(r'\[[^\]]*\]\([^)]*\)', ' ', s)
    s = re.sub(r'https?://\S+', ' ', s)
    cn = len(re.findall(r'[一-鿿]', s))
    ratio = cn / len(s) if s else 0.0
    en = len(re.findall(r'[A-Za-z]+', s))
    if ratio >= 0.03:
        return False
    if en < 8:
        return False
    return True

need = [f for f in files if needs(f)]
with open(base + '/.translate_need.txt', 'w', encoding='utf-8') as fh:
    for f in need:
        fh.write(f + '\n')
print("remaining need:", len(need))
