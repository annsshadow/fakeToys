import json,re,os
from collections import defaultdict
ROOT='D:/WORKSPACE/fakeToys'
backend=json.load(open(f'{ROOT}/docs/audits/three-ends-2026-09-20/backend_routes.json',encoding='utf-8'))

def url_slots(path):
    return len(re.findall(r'\{[^}]*\}', path))

# cache file text
_cache={}
def gettext(fp):
    if fp not in _cache:
        p=os.path.join(ROOT,'oa4rust',fp) if not os.path.isabs(fp) else fp
        # backend file paths are like crates/xxx/src/routes.rs relative to oa4rust
        try: _cache[fp]=open(p,encoding='utf-8',errors='replace').read()
        except: _cache[fp]=None
    return _cache[fp]

def find_sig(text, handler):
    # find 'fn handler(' possibly 'async fn' / 'pub'
    m=re.search(r'\bfn\s+'+re.escape(handler)+r'\s*(?:<[^>]*>)?\s*\(', text)
    if not m: return None
    i=m.end()-1  # at '('
    depth=0; j=i
    while j<len(text):
        c=text[j]
        if c=='(': depth+=1
        elif c==')':
            depth-=1
            if depth==0: break
        j+=1
    return text[i:j+1]  # includes parens

def path_arity(sig):
    # find Path<...> in the signature type position
    # patterns: Path<( T1 , T2 )>  or Path<T>
    ms=list(re.finditer(r'Path\s*<', sig))
    if not ms: return 0
    # take the type after Path< ... matching >
    total=None
    ar=0
    for m in ms:
        k=m.end(); depth=1; s=k
        while k<len(sig) and depth>0:
            if sig[k]=='<': depth+=1
            elif sig[k]=='>': depth-=1
            k+=1
        inner=sig[s:k-1].strip()
        if inner.startswith('('):
            # tuple: count top-level commas+1
            depth=0; cnt=1; empty=(inner.strip()=='()')
            for ch in inner[1:-1]:
                if ch in '(<[': depth+=1
                elif ch in ')>]': depth-=1
                elif ch==',' and depth==0: cnt+=1
            ar += 0 if empty else cnt
        else:
            ar += 1
    return ar

rows=[]
macro=[]
nofn=[]
for crate,routes in backend.items():
    for r in routes:
        if 'mock' in r['path']: continue
        slots=url_slots(r['path'])
        fp=r.get('file'); h=r.get('handler')
        txt=gettext(fp) if fp else None
        if txt is None:
            nofn.append((crate,r['method'],r['path'],h,fp,'NOFILE')); continue
        sig=find_sig(txt,h)
        if sig is None:
            # handler likely macro-generated
            macro.append((crate,r['method'],r['path'],h,slots)); continue
        ar=path_arity(sig)
        if ar!=slots:
            rows.append((crate,r['method'],r['path'],h,slots,ar,fp))

print("=== ARITY MISMATCH (url_slots != handler Path arity) ===")
print("total mismatches:",len(rows))
bycrate=defaultdict(int)
for c,m,p,h,s,a,fp in rows: bycrate[c]+=1
for c,n in sorted(bycrate.items(),key=lambda x:-x[1]):
    print(f"  {c:38}{n}")
print("\nmacro-handler (no fn found, likely macro):",len(macro))
mc=defaultdict(int)
for c,m,p,h,s in macro: mc[c]+=1
for c,n in sorted(mc.items(),key=lambda x:-x[1])[:15]: print(f"  {c:38}{n}")
print("\nNOFILE:",len(nofn))
json.dump({'mismatch':rows,'macro':macro,'nofile':nofn},open(f'{ROOT}/docs/audits/three-ends-2026-09-20/_arity_audit.json','w',encoding='utf-8'),ensure_ascii=False,indent=0)
