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
sigcache={}
def sigs(crate):
    if crate in sigcache: return sigcache[crate]
    out={}
    for f in glob.glob(ROOT+'/'+crate+'/src/**/*.rs',recursive=True):
        s=open(f,encoding='utf-8',errors='ignore').read()
        for mm in re.finditer(r'pub async fn (\w+)\s*\(([^{]*?)\)\s*->',s):
            out.setdefault(mm.group(1),mm.group(2))
    sigcache[crate]=out
    return out
macro_arity={}
for f in glob.glob(ROOT+'/*/src/**/*.rs',recursive=True):
    crate=f.split('/')[3]
    s=open(f,encoding='utf-8',errors='ignore').read()
    for m in re.finditer(r'(\w+)!\((\w+)',s):
        name,h=m.group(1),m.group(2)
        j=s.find('(',m.end()-2)
        if j<0 or j>m.end()+2: continue
        i=j; depth=0; j=i
        while j<len(s):
            if s[j]=='(':depth+=1
            elif s[j]==')':
                depth-=1
                if depth==0: break
            j+=1
        body=s[i:j]
        pmm=re.search(r'\(([^()]*)\)\s*$',body)
        if pmm and pmm.group(1).strip() and ':' in pmm.group(1):
            n=pmm.group(1).count(',')+1
            heads=re.findall(r'^\s*(\w+),\s*(\w+)\s*\(',body,re.M)
            for a,b in heads:
                macro_arity[(crate,a)]=n; macro_arity[(crate,b)]=n
    for m in re.finditer(r'(\w+)!\((\w+)\s*,\s*(\d+)\)',s):
        macro_arity[(crate,m.group(2))]=int(m.group(3))
    for m in re.finditer(r'data_path_write_handlers!\(\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\d+)\)',s):
        for g in (1,2,3):
            macro_arity[(crate,m.group(g))]=int(m.group(4))
    for m in re.finditer(r'topic_toggle_endpoints!\((\w+)',s):
        macro_arity[(crate,m.group(1))]=1
CRED=['login','logout','regist','password','captcha','/oauth','/sso','/token','/code/','validate','two_factor','dingding','qywx','qiyeweixin','zhengwudingding','authentication','/empower']
PLACE=['/test','/foo','/bar','mass/from/count','/collect','mpweixin','/%20',' /']
rows=[]
for crate,routes in be.items():
    S=sigs(crate)
    for r in routes:
        m,p,h=r['method'],r['path'],r['handler']
        if (m,p) in consumed or 'mock' in p or not belongs(p,ONMETRIC): continue
        if p.startswith('/ws/') or any(c in p for c in CRED) or any(c in p for c in PLACE): continue
        if p.endswith('/object') or any(x in p for x in ['/upload','multipart','base64']) or ('download' in p and 'stream' in p): continue
        s=S.get(h,'')
        pm=re.search(r'Path\s*<\s*\[String;\s*(\d+)\]',s)
        if pm:
            ar=int(pm.group(1))
        else:
            pm=re.search(r'Path\s*<\s*\(?([^>]*?)\)?\s*>',s)
            ar=(pm.group(1).count(',')+1) if pm and pm.group(1) is not None and pm.group(1).strip() else 0
        if ar==0:
            ar=macro_arity.get((crate,h),0)
        if ar!=p.count('{'): continue
        src=''
        for f in glob.glob(ROOT+'/'+crate+'/src/**/*.rs',recursive=True):
            src+=open(f,encoding='utf-8',errors='ignore').read()+'\n'
        b=re.search(r'pub async fn '+re.escape(h)+r'\s*\([^{]*?\{(.*?)\n\}',src,re.S)
        body=b.group(1) if b else ''
        if 'capability_unavailable' in body: continue
        dml=bool(re.search(r'\b(INSERT|UPDATE|DELETE)\b',body))
        mt=[mm for mm in ('GET','POST','PUT','DELETE') if mm!=m and (mm,p) in consumed]
        rows.append((crate,m,p,h,'DML' if dml else 'READ','TWIN' if mt else 'SOLO'))
for x in sorted(rows):
    print(x[4],x[5],x[0],'|',x[1],x[2],'|',x[3])
print('RECOVERED ARITY-MATCHED UNCONSUMED:',len(rows))
