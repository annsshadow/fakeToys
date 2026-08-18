import os, re, sys, json

def cjk_ratio(s):
    cjk = sum(1 for ch in s if '\u4e00' <= ch <= '\u9fff')
    return cjk / max(1, len(s))

def count_english_words(s):
    return len(re.findall(r"[A-Za-z][A-Za-z'\-]+", s))

def strip_code_and_struct(md):
    lines = md.split('\n')
    out = []
    in_fence = False
    for ln in lines:
        if re.match(r'^\s*```', ln):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        ln2 = re.sub(r'`[^`]*`', '', ln)
        if re.search(r'^\s*\[[^\]]*\]\([^)]*\)\s*$', ln2):
            continue
        if re.match(r'^\s*!\[', ln2):
            continue
        out.append(ln2)
    return '\n'.join(out)

def fence_parity(raw):
    depth = 0
    for ln in raw.split('\n'):
        if re.match(r'^\s*```', ln):
            depth += 1
    return depth % 2 == 0

def analyze(path):
    raw = open(path, encoding='utf-8', errors='ignore').read()
    prose = strip_code_and_struct(raw)
    pl = [ln for ln in prose.split('\n') if not re.match(r'^\s{0,3}#{1,6}\s', ln)]
    prose = '\n'.join(pl).strip()
    cjk = cjk_ratio(prose)
    nw = count_english_words(prose)
    return fence_parity(raw), cjk, nw, len(prose)

if __name__ == '__main__':
    files = [l.strip() for l in open('/tmp/chunks1.txt', encoding='utf-8') if l.strip()]
    results = []
    for f in files:
        if not os.path.exists(f):
            results.append((f, 'MISSING', None, None, None))
            continue
        fp, cjk, nw, plen = analyze(f)
        results.append((f, 'OK' if fp else 'ODD_FENCE', round(cjk,3), nw, plen))
    for f, status, cjk, nw, plen in results:
        print(f"{status:10} cjk={cjk:.3f} w={nw:4} len={plen:5} {f.split('系统文档')[-1]}")
