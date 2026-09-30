import json,re,glob,sys
crate=sys.argv[1]
prefixfilter=sys.argv[2] if len(sys.argv)>2 else ''
rec=json.load(open('api_reconcile.json'))
be=json.load(open('backend_routes.json'))
consumed=set()
for end,v in rec.items():
    if not isinstance(v,dict):continue
    for k in ('exact','param-ok'):
        for it in v.get(k,[]): consumed.add((it.get('method'),it.get('hit')))
src=''
for f in glob.glob('D:/WORKSPACE/fakeToys/oa4rust/crates/'+crate+'/src/**/*.rs',recursive=True):
    src+=open(f,encoding='utf-8',errors='ignore').read()+'\n'
def sig(n):
    m=re.search(r'pub async fn '+re.escape(n)+r'\s*\(([^{]*?)\)\s*->',src);return m.group(1) if m else None
rows=[]
for r in be[crate]:
    m=r['method'];p=r['path'];h=r['handler']
    if prefixfilter and prefixfilter not in p: continue
    if (m,p) in consumed: continue
    if 'mock' in p or '{path' in p: continue
    s=sig(h)
    if s is None: continue
    haspath='Path<' in s; hasjson='Json<' in s
    pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',s); ar=(pm.group(1).count(',')+1 if pm and pm.group(1).strip() else 0)
    pc=p.count('{')
    clean=(haspath and ar==pc and pc>0) or (not haspath and pc==0)
    if not clean: continue
    term=p.rstrip('/').split('/')[-1]
    dist=not term.startswith('{')
    rows.append((m,p,h,'POOL' if not haspath and not hasjson else('JSON' if hasjson else 'PATH'),ar,pc,dist))
# prefer GET reads, distinctive terminal
rows.sort(key=lambda x:(x[0]!='GET', not x[6], x[5]))
for m,p,h,cls,ar,pc,dist in rows[:50]:
    print(m,p,'|',cls,'ar',ar,'pc',pc,'D' if dist else 'p')
print('total clean:',len(rows))
