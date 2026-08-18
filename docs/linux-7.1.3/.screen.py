import os, re, sys

def strip_md(text):
    # remove fenced code blocks
    lines = text.split('\n')
    out = []
    in_fence = False
    for ln in lines:
        if ln.lstrip().startswith('```'):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        out.append(ln)
    body = '\n'.join(out)
    # remove inline code spans
    body = re.sub(r'`[^`]*`', ' ', body)
    # remove heading lines (lines starting with #)
    body_lines = []
    for ln in body.split('\n'):
        s = ln.strip()
        if s.startswith('#'):
            continue
        # remove link-only lines: a line that is essentially just a markdown link
        if re.fullmatch(r'\s*\[[^\]]*\]\([^)]*\)\s*', ln):
            continue
        body_lines.append(ln)
    prose = '\n'.join(body_lines)
    return prose

def metrics(prose):
    # CJK chars
    cjk = len(re.findall(r'[\u4e00-\u9fff\u3400-\u4dbf]', prose))
    total = len(prose)
    cjk_ratio = (cjk / total) if total else 0.0
    # english words: runs of ascii letters length>=2
    words = re.findall(r'[A-Za-z]{2,}', prose)
    nwords = len(words)
    return cjk, total, cjk_ratio, nwords

paths = [l.strip() for l in open('.chunk0_paths.txt', encoding='utf-8') if l.strip()]
for p in paths:
    try:
        sz = os.path.getsize(p)
        text = open(p, encoding='utf-8', errors='ignore').read()
    except Exception as e:
        print(f"FAILREAD {sz if 'sz' in dir() else -1} {p} :: {e}")
        continue
    prose = strip_md(text)
    cjk, total, ratio, nwords = metrics(prose)
    # decision
    if total < 20 or nwords < 8:
        dec = 'SKIP(empty/short)'
    elif ratio >= 0.10 and nwords < 20:
        dec = 'SKIP(alreadytx)'
    else:
        dec = 'TRANSLATE'
    print(f"{dec:18} sz={sz:8} cjk={cjk:5} ratio={ratio:.3f} enwords={nwords:4} {p.split('系统文档')[-1]}")
