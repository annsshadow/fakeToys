import json,re,glob
from consumption_gap import DOMAINS, belongs
ONMETRIC=[k for _,ks in DOMAINS for k in ks]
ROOT='D:/WORKSPACE/fakeToys/oa4rust/crates'
be=json.load(open('backend_routes.json',encoding='utf-8'))
rec=json.load(open('api_reconcile.json',encoding='utf-8'))
consumed=set()
for app,v in rec.items():
    if not isinstance(v,dict):continue
    for k in ('exact','param-ok'):
        for c in v.get(k,[]):
            if c.get('hit'): consumed.add((c['method'],c['hit']))
macro_arity={}
for f in glob.glob(ROOT+'/*/src/**/*.rs',recursive=True):
    crate=f.split('/')[3]
    s=open(f,encoding='utf-8',errors='ignore').read()
    for m in re.finditer(r'(\w+)!\((\w+)',s):
        j=s.find('(',m.end()-2)
        if j<0 or j>m.end()+2: continue
        i=j; depth=0; k2=i
        while k2<len(s):
            if s[k2]=='(':depth+=1
            elif s[k2]==')':
                depth-=1
                if depth==0:break
            k2+=1
        body=s[i:k2]
        pmm=re.search(r'\(([^()]*)\)\s*$',body)
        if pmm and pmm.group(1).strip() and ':' in pmm.group(1):
            n=pmm.group(1).count(',')+1
            for a,b2 in re.findall(r'^\s*(\w+),\s*(\w+)\s*\(',body,re.M):
                macro_arity[(crate,a)]=n; macro_arity[(crate,b2)]=n
    for m in re.finditer(r'(\w+)!\((\w+)\s*,\s*(\d+)\)',s):
        macro_arity[(crate,m.group(2))]=int(m.group(3))
    for m in re.finditer(r'data_path_write_handlers!\(\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\d+)\)',s):
        for g in (1,2,3): macro_arity[(crate,m.group(g))]=int(m.group(4))
    for m in re.finditer(r'topic_toggle_endpoints!\((\w+)',s):
        macro_arity[(crate,m.group(1))]=1
CRED=['login','logout','regist','password','captcha','/oauth','/sso','/token','/code/','validate','two_factor','dingding','qywx','qiyeweixin','zhengwudingding','authentication','/empower']
PLACE=['/test','/foo','/bar','mass/from/count','/collect','mpweixin','/%20',' /']
out=[]
for crate,routes in be.items():
    src=''
    S={}
    for f in glob.glob(ROOT+'/'+crate+'/src/**/*.rs',recursive=True):
        s=open(f,encoding='utf-8',errors='ignore').read(); src+=s+'\n'
        for mm in re.finditer(r'pub async fn (\w+)\s*\(([^{]*?)\)\s*->',s): S.setdefault(mm.group(1),mm.group(2))
    for r in routes:
        m,p,h=r['method'],r['path'],r['handler']
        if (m,p) in consumed or 'mock' in p: continue
        if belongs(p,ONMETRIC): continue
        if p.startswith('/ws/') or any(c in p for c in CRED) or any(c in p for c in PLACE): continue
        sig=S.get(h,'')
        pm=re.search(r'Path\s*<\s*\[String;\s*(\d+)\]',sig)
        if pm: ar=int(pm.group(1))
        else:
            pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',sig)
            ar=(pm.group(1).count(',')+1) if pm and pm.group(1) is not None and pm.group(1).strip() else 0
        if ar==0: ar=macro_arity.get((crate,h),0)
        if ar!=p.count('{'): continue
        b=re.search(r'pub async fn '+re.escape(h)+r'\s*\([^{]*?\{(.*?)\n\}',src,re.S)
        body=b.group(1) if b else ''
        if 'capability_unavailable' in body: continue
        dml=bool(re.search(r'\b(INSERT|UPDATE|DELETE)\b',body))
        out.append((crate,m,p,h,'DML' if dml else 'READ'))
out.sort()
for c,m,p,h,t in out:
    print(t,m,p,'|',c,'|',h)
print('OFFMETRIC WIREABLE:',len(out))
json.dump([list(x) for x in out],open('_offmetric_wireable.json','w',encoding='utf-8'),ensure_ascii=False)
