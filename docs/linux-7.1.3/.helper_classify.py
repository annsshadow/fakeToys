import os, re, sys, json

def cjk_ratio(s):
    cjk = sum(1 for ch in s if '\u4e00' <= ch <= '\u9fff')
    return cjk / max(1, len(s))

def count_english_words(s):
    words = re.findall(r"[A-Za-z][A-Za-z'\-]+", s)
    return len(words)

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

def needs_translation(path):
    try:
        raw = open(path, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        return ('ERR', str(e), 0, 0, 0)
    size = os.path.getsize(path)
    prose = strip_code_and_struct(raw)
    prose_lines = []
    for ln in prose.split('\n'):
        if re.match(r'^\s{0,3}#{1,6}\s', ln):
            continue
        prose_lines.append(ln)
    prose = '\n'.join(prose_lines).strip()
    cjk = cjk_ratio(prose)
    nwords = count_english_words(prose)
    if len(prose) < 30 or nwords < 8:
        return ('SKIP', 'near-empty/short', size, cjk, nwords)
    if cjk >= 0.10 and nwords < 20:
        return ('SKIP', 'already-chinese', size, cjk, nwords)
    return ('TRANS', 'needs-translation', size, cjk, nwords)

files = [l.strip() for l in open('/tmp/chunks1.txt', encoding='utf-8') if l.strip()]
results = []
for f in files:
    if not os.path.exists(f):
        results.append((f,'MISSING',0,0,0,'no file'))
        continue
    dec, reason, size, cjk, nwords = needs_translation(f)
    results.append((f, dec, size, round(cjk,3), nwords, reason))

for f, dec, size, cjk, nwords, reason in results:
    print(f"{dec:6} size={size:8} cjk={cjk:.3f} words={nwords:4} {reason:20} {f.split('系统文档')[-1]}")
