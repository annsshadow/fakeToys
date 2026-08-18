import sys, re, os

def strip_code(text):
    # remove fenced code blocks
    out=[]
    infence=False
    for ln in text.split('\n'):
        if ln.lstrip().startswith('```'):
            infence = not infence
            continue
        if infence:
            continue
        out.append(ln)
    return '\n'.join(out)

def is_link_line(ln):
    s=ln.strip()
    return bool(re.match(r'^\[.*\]\(.*\)\s*$', s)) or '](' in s and s.startswith('[')

def analyze(path):
    try:
        text=open(path,encoding='utf-8',errors='ignore').read()
    except Exception as e:
        return ('FAIL', str(e), 0,0,0,0)
    size=len(text.encode('utf-8'))
    # fence count
    fence=sum(1 for ln in text.split('\n') if ln.lstrip().startswith('```'))
    # strip code
    nos=strip_code(text)
    lines=nos.split('\n')
    prose_lines=[]
    for ln in lines:
        s=ln.strip()
        if not s: continue
        if s.startswith('#'): continue
        if is_link_line(ln): continue
        prose_lines.append(ln)
    prose='\n'.join(prose_lines)
    # count chinese chars
    cjk=len(re.findall(r'[\u4e00-\u9fff]', prose))
    # english words
    words=re.findall(r'[A-Za-z]+', prose)
    nw=len(words)
    total=len(prose)
    ratio = (cjk/(cjk+nw)) if (cjk+nw)>0 else 0
    return ('OK', None, size, fence, cjk, nw, round(ratio,3))

paths=open('.w190_list.txt',encoding='utf-8').read().split('\n')
paths=[p for p in paths if p]
print('idx  cjk   nw   ratio  fence  size  verdict  path')
for idx,p in enumerate(paths):
    r=analyze(p)
    if r[0]=='FAIL':
        print(f'{idx:3}  FAIL {r[1]}')
        continue
    _,_,size,fence,cjk,nw,ratio=r
    # verdict
    verdict='TRANSLATE'
    if nw<8 or cjk+nw<8:
        verdict='SKIP(lowprose)'
    elif ratio>=0.10 and nw<20:
        verdict='SKIP(translated)'
    print(f'{idx:3} {cjk:5} {nw:5} {ratio:5} {fence:5} {size:7}  {verdict:16} {os.path.basename(p)}')
