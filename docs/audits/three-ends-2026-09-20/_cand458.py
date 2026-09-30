import json,re,os,glob
BASE='D:/WORKSPACE/fakeToys/oa4rust'
rec=json.load(open('api_reconcile.json'))
be=json.load(open('backend_routes.json'))
consumed=set()
for end,v in rec.items():
    if not isinstance(v,dict): continue
    for k in ('exact','param-ok'):
        for it in v.get(k,[]):
            m=it.get('method'); p=it.get('hit')
            if m and p: consumed.add((m.upper(),p))

crate='processplatform_assemble_surface'
routes=be[crate]
# gather handler sources for this crate
srcdir=os.path.join(BASE,'crates',crate,'src')
src=''
for f in glob.glob(srcdir+'/**/*.rs',recursive=True):
    src+=open(f,encoding='utf-8',errors='ignore').read()+'\n'

def sig(name):
    m=re.search(r'pub async fn '+re.escape(name)+r'\s*\(([^{]*?)\)\s*->', src)
    return m.group(1) if m else None

def classify(name):
    s=sig(name)
    if s is None: return ('NO-SIG',None)
    haspath='Path<' in s or 'Path <' in s
    hasjson='Json<' in s
    # path arity
    ar=0
    pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',s)
    if pm:
        inner=pm.group(1)
        ar=inner.count(',')+1 if inner.strip() else 0
    return ('PATH' if haspath else ('JSON' if hasjson else 'POOL'), ar)

cands=[]
for r in routes:
    m=r['method'].upper(); p=r['path']; h=r['handler']
    if (m,p) in consumed: continue
    cls,ar=classify(h)
    pc=p.count('{')
    term=p.rstrip('/').split('/')[-1]
    distinctive = not (term.startswith('{'))  # ends with literal
    # clean = POOL or JSON (no path), OR PATH with arity matching param count
    ok=False
    if cls=='POOL' and pc==0: ok=True
    elif cls=='JSON' and pc==0: ok=True
    elif cls=='PATH' and ar==pc and pc>0: ok=True
    cands.append((ok,m,p,h,cls,ar,pc,distinctive))

clean=[c for c in cands if c[0] and c[7]]
print('total unconsumed in surface:',len(cands))
print('clean+distinctive:',len(clean))
for c in clean[:60]:
    print(c[1],c[2],'|',c[4],'ar',c[5],'pc',c[6])

print('=== breakdown of unconsumed ===')
from collections import Counter
c=Counter((x[4],'ar%s'%x[5],'pc%s'%x[6],'dist' if x[7] else 'param') for x in cands)
for k,v in c.most_common(20): print(v,k)
print('=== clean regardless of distinctive ===')
cl2=[x for x in cands if x[0]]
print('clean(any term):',len(cl2))
for c in cl2[:40]:
    print(c[1],c[2],'| term-dist' if c[7] else '| term-param',c[4],'ar',c[5])

print('=== FILTERED (no mock*, no applicationdict deep-fan, pc<=2) ===')
for c in cands:
    if not c[0]: continue
    p=c[2]
    if 'mock' in p: continue
    if 'applicationdict' in p: continue
    if '{path' in p: continue
    if c[6]>2: continue
    print(c[1],p,'|',c[4],'ar',c[5])
