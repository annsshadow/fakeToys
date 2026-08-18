import json,re,os
d=json.load(open('.translate_dispatch_cold.json',encoding='utf-8'))
files=d['chunks'][1]

def classify(path):
    try:
        t=open(path,encoding='utf-8').read()
    except Exception as e:
        return ('READERR',str(e)[:50],0,0,0)
    # remove fenced code blocks
    t2=re.sub(r'```.*?```','',t,flags=re.S)
    # remove inline code
    t2=re.sub(r'`[^`]*`','',t2)
    # remove heading lines
    t2=re.sub(r'(?m)^#.*$','',t2)
    # remove link-only lines (line that is essentially just a markdown link or list of links)
    lines=t2.split('\n')
    out=[]
    for ln in lines:
        s=ln.strip()
        # link line: mostly bracket/paren link syntax
        if re.fullmatch(r'(-?\s*\[[^\]]*\]\([^)]*\)\s*)+', s):
            continue
        out.append(ln)
    prose='\n'.join(out)
    cjk=len(re.findall(r'[\u4e00-\u9fff]',prose))
    plen=max(1,len(prose))
    eng=len(re.findall(r'[A-Za-z]{3,}',prose))
    ratio=cjk/plen
    # decision
    if plen<30:
        dec='SKIP_EMPTY'
    elif eng<8:
        dec='SKIP_FEWENG'
    elif ratio>=0.10 and eng<20:
        dec='SKIP_CHINESE'
    else:
        dec='TRANSLATE'
    return (dec,f'{ratio:.3f}',eng,plen)

for i,p in enumerate(files):
    dec,ratio,eng,plen=classify(p)
    flag='' if dec=='TRANSLATE' else '  '
    print(f'{i:2d} {dec:14s} r={ratio:>6} eng={eng:4d} len={plen:5d} {flag}{os.path.basename(p)}')
