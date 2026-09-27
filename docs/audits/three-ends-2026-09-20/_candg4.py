import json,re,glob,sys
from consumption_gap import DOMAINS, belongs
ONMETRIC=[k for _,ks in DOMAINS for k in ks]
crates=sys.argv[1:]
rec=json.load(open('api_reconcile.json'))
be=json.load(open('backend_routes.json'))
consumed=set()
for end,v in rec.items():
    if not isinstance(v,dict):continue
    for k in ('exact','param-ok'):
        for it in v.get(k,[]): consumed.add((it.get('method'),it.get('hit')))
consumed_paths=[p for (_m,p) in consumed]
CRED=['login','logout','regist','password','captcha','authentication','/oauth','/sso','anonymous','/token','/code/','validate']
EXT=['collect','dingding','qywx','qiyeweixin','mpweixin','zhengwudingding','/sms','oauth','/send','/sync','download','upload','install','deploy','/foo','/bar','/test']
WRITE=['/create','/delete','/save','/update','/edit','/set','/add','/remove','/import','/reset','/merge','/flush','/refresh','/wipe','/execute','/force','/append','/generate','/publish','/bind','/unbind','changeTitle','/photo']
def norm(p): return re.sub(r'\{[^}]+\}','{}',p)
for crate in crates:
    src=''
    for f in glob.glob('D:/WORKSPACE/fakeToys/oa4rust/crates/'+crate+'/src/**/*.rs',recursive=True):
        src+=open(f,encoding='utf-8',errors='ignore').read()+'\n'
    def sig(n):
        m=re.search(r'pub async fn '+re.escape(n)+r'\s*\(([^{]*?)\)\s*->',src);return m.group(1) if m else None
    bypath={}
    for r in be[crate]: bypath.setdefault(r['path'],set()).add(r['method'])
    out=[]
    for r in be[crate]:
        m=r['method'];p=r['path'];h=r['handler']
        if (m,p) in consumed: continue
        if 'mock' in p or '{path' in p: continue
        if not belongs(p,ONMETRIC): continue   # on-metric only
        if any(c in p for c in CRED): continue
        if any(e in p for e in EXT): continue
        if any(w in p for w in WRITE): continue
        if any((mm,p) in consumed for mm in bypath[p] if mm!=m): continue
        if p.endswith('/object'):
            stem=p[:-len('/object')]
            if any(cp.startswith(stem) for cp in consumed_paths): continue
        s=sig(h)
        if s is None: continue
        haspath='Path<' in s; hasjson='Json<' in s
        pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',s); ar=(pm.group(1).count(',')+1 if pm and pm.group(1).strip() else 0)
        pc=p.count('{')
        clean=(haspath and ar==pc and pc>0) or (not haspath and pc==0)
        if not clean: continue
        # confirm handler body has SELECT and no INSERT/UPDATE/DELETE (read-only)
        b=re.search(r'pub async fn '+re.escape(h)+r'\s*\([^{]*?\{(.*?)\n\}',src,re.S)
        body=b.group(1) if b else ''
        readonly = ('SELECT' in body or 'query' in body.lower()) and not re.search(r'\b(INSERT|UPDATE|DELETE)\b',body)
        stub = 'capability_unavailable' in body
        coll=sum(1 for rr in be[crate] if norm(rr['path'])==norm(p) and rr['method']==m)
        if stub: continue
        out.append((m,p,'POOL' if not haspath and not hasjson else('JSON' if hasjson else 'PATH'),ar,dist:=(not p.rstrip('/').split('/')[-1].startswith('{')),coll,readonly))
    out.sort(key=lambda x:(not x[6],x[1]!='GET',not x[4]))
    print('##### %s : clean=%d'%(crate,len(out)))
    for m,p,cls,ar,dist,coll,ro in out[:25]:
        print(' ',('RO' if ro else 'w?'),m,p,'|',cls,'ar',ar,'D' if dist else 'p','coll',coll)
