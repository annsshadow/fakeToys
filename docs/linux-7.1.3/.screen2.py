import re, json
paths = [l.strip() for l in open('.chunk0_paths.txt', encoding='utf-8') if l.strip()]
def analyze(p):
    try:
        t = open(p, encoding='utf-8').read()
    except Exception as e:
        return ('ERR', str(e), 0, 0, 0)
    lines = t.split('\n')
    out = []
    infence = False
    fcount = 0
    for ln in lines:
        s = ln.strip()
        if s.startswith('```'):
            if not infence:
                infence = True
            else:
                infence = False
            fcount += 1
            continue
        if infence:
            continue
        if s.startswith('#') or s.startswith('>'):
            continue
        # strip inline code
        s2 = re.sub(r'`[^`]*`', ' ', s)
        # strip links: keep only display text
        s2 = re.sub(r'\[([^\]]*)\]\([^)]*\)', r'\1', s2)
        # remove pure url / image lines
        if re.match(r'^\s*(https?://|!\[|<|\|)', s2):
            continue
        out.append(s2)
    prose = '\n'.join(out)
    cjk = len(re.findall(r'[\u4e00-\u9fff]', prose))
    total = len(re.findall(r'\S', prose))
    enwords = len(re.findall(r'[A-Za-z]+', prose))
    ratio = cjk/total if total else 0
    return ('OK', fcount, cjk, enwords, round(ratio,3))

for i,p in enumerate(paths,1):
    st,fc,cjk,en,ratio = analyze(p)
    short = p.split('\\')[-1]
    print(f"{i:2d} {short:35s} {st} fence={fc} cjk={cjk} en={en} ratio={ratio} {'<<SKIP/translated' if (ratio>=0.03) else ''}")
