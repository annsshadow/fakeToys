import json,re,glob,sys
from consumption_gap import DOMAINS, belongs
ONMETRIC=[k for _,ks in DOMAINS for k in ks]
be=json.load(open('backend_routes.json',encoding='utf-8'))
rec=json.load(open('api_reconcile.json',encoding='utf-8'))
consumed=set()
for app,v in rec.items():
    if not isinstance(v,dict):continue
    for k in ('exact','param-ok'):
        for it in v.get(k,[]):
            if it.get('hit'): consumed.add((it.get('method'),it.get('hit')))
def norm(p): return re.sub(r'\{[^}]+\}','{}',p)
out=[]
for crate,routes in be.items():
    src=''
    for f in glob.glob('D:/WORKSPACE/fakeToys/oa4rust/crates/'+crate+'/src/**/*.rs',recursive=True):
        src+=open(f,encoding='utf-8',errors='ignore').read()+'\n'
    def sig(n):
        m=re.search(r'pub async fn '+re.escape(n)+r'\s*\(([^{]*?)\)\s*->',src);return m.group(1) if m else None
    for r in routes:
        m=r['method'];p=r['path'];h=r['handler']
        if (m,p) in consumed: continue
        if 'mock' in p or '{path' in p: continue
        if not belongs(p,ONMETRIC): continue
        s=sig(h)
        if s is None: continue
        haspath='Path<' in s; hasjson='Json<' in s
        pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',s); ar=(pm.group(1).count(',')+1 if pm and pm.group(1).strip() else 0)
        pc=p.count('{')
        # arity-consistent only
        ok=(haspath and ar==pc) or (not haspath and pc==0)
        if not ok: continue
        b=re.search(r'pub async fn '+re.escape(h)+r'\s*\([^{]*?\{(.*?)\n\}',src,re.S)
        body=b.group(1) if b else ''
        readonly = ('SELECT' in body or 'query' in body.lower()) and not re.search(r'\b(INSERT|UPDATE|DELETE)\b',body)
        stub='capability_unavailable' in body
        if not readonly or stub: continue
        out.append((crate,m,p,ar))
for c,m,p,ar in sorted(out):
    print(c,'|',m,p,'| ar',ar)
print('TOTAL read-candidates:',len(out))
