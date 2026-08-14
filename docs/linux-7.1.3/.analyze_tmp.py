import json, os, re
d = json.load(open('.translate_dispatch_cold2.json', encoding='utf-8'))
chunk = d['chunks'][0]

def analyze(path):
    path = path.replace('\\', '/')
    try:
        txt = open(path, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        return ('ERR', str(e), 0, 0, 0)
    lines = txt.split('\n')
    fences = sum(1 for ln in lines if ln.lstrip().startswith('```'))
    out = []
    fencedepth = 0
    for ln in lines:
        if ln.lstrip().startswith('```'):
            fencedepth += 1
            continue
        if (fencedepth % 2) == 0:
            out.append(ln)
    prose = '\n'.join(out)
    prose = re.sub(r'`[^`]*`', '', prose)
    prose = re.sub(r'^#+\s*', '', prose, flags=re.M)
    zh = len(re.findall(r'[一-鿿]', prose))
    en = len(re.findall(r'[A-Za-z]{2,}', prose))
    total = zh + en
    ratio = (zh / total) if total > 0 else 0
    return ('OK', None, fences, zh, en, round(ratio, 3))

for p in chunk:
    path = p.replace('\\', '/')
    name = os.path.basename(path)
    try:
        st, _, fences, zh, en, ratio = analyze(path)
    except Exception as e:
        st, fences = 'ERR', str(e)
    if st == 'ERR':
        print('ERR  ', name, fences)
    else:
        decision = 'SKIP' if (ratio >= 0.10 and en < 20) or en < 8 else 'TR'
        print(f'{decision:5} fences={fences:3} zh={zh:5} en={en:4} ratio={ratio}  {name}')
