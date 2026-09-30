#!/usr/bin/env python3
"""Full arity audit: find each route-handler fn ANYWHERE in its crate, compare
URL {param} slot count to the handler's axum Path extractor arity. Runtime 500
'Wrong number of path arguments' occurs when they differ."""
import json,re,os,glob
from collections import defaultdict
ROOT='D:/WORKSPACE/fakeToys/oa4rust'
backend=json.load(open('D:/WORKSPACE/fakeToys/docs/audits/three-ends-2026-09-20/backend_routes.json',encoding='utf-8'))
PATH=r'(?:axum::)?(?:extract::)?Path'
def split_top(s):
    out=[];d=0;cur=''
    for ch in s:
        if ch in '(<[':d+=1
        elif ch in ')>]':d-=1
        if ch==',' and d==0: out.append(cur);cur=''
        else: cur+=ch
    if cur.strip():out.append(cur)
    return [x.strip() for x in out if x.strip()]
# index: crate -> {handler -> arity}
crate_files=defaultdict(list)
def crate_of(fp):  # fp like crates/xxx/src/...
    m=re.match(r'crates/([^/]+)/',fp.replace(chr(92),'/'))
    return m.group(1) if m else None
# preload all crate fn arities lazily
fn_re_tpl=r'\bfn\s+%s\s*(?:<[^>]*>)?\s*\('
_idx={}
def build_crate_index(crate):
    if crate in _idx: return _idx[crate]
    idx={}
    for f in glob.glob(f'{ROOT}/crates/{crate}/src/**/*.rs',recursive=True):
        txt=open(f,encoding='utf-8',errors='replace').read()
        for m in re.finditer(r'\bfn\s+([A-Za-z0-9_]+)\s*(?:<[^>]*>)?\s*\(',txt):
            name=m.group(1)
            # scan param list until matching ) at depth0
            i=m.end()-1;depth=0;j=i
            while j<len(txt):
                c=txt[j]
                if c=='(':depth+=1
                elif c==')':
                    depth-=1
                    if depth==0:break
                j+=1
            params=txt[i:j+1]
            # find Path<...> arity
            bm=re.search(PATH+r'\(\s*(?:\([^)]*\)|[A-Za-z0-9_]+)\s*\)\s*:\s*'+PATH+r'<',params)
            if not bm: ar=0
            else:
                k=bm.end();d=1;q=k
                while q<len(params) and d>0:
                    if params[q]=='<':d+=1
                    elif params[q]=='>':d-=1
                    q+=1
                inner=params[k:q-1].strip()
                if inner.startswith('('):
                    ar=len(split_top(inner[1:-1]))
                else: ar=1
            idx.setdefault(name,ar)  # first def wins
    _idx[crate]=idx
    return idx
rows=[];nofn=[]
for crate,routes in backend.items():
    for r in routes:
        if 'mock' in r['path']: continue
        slots=len(re.findall(r'\{[^}]*\}',r['path']))
        c=crate_of(r['file']) or crate
        idx=build_crate_index(c)
        h=r['handler']
        if h not in idx:
            nofn.append((c,r['method'],r['path'],h)); continue
        ar=idx[h]
        if ar!=slots:
            rows.append((c,r['method'],r['path'],h,slots,ar))
print("TRUE arity mismatches:",len(rows))
bc=defaultdict(int)
for c,m,p,h,s,a in rows: bc[c]+=1
for c,n in sorted(bc.items(),key=lambda x:-x[1]): print(f"  {c:40}{n}")
print("handler fn NOT found in crate (still macro/other):",len(nofn))
bn=defaultdict(int)
for c,m,p,h in nofn: bn[c]+=1
for c,n in sorted(bn.items(),key=lambda x:-x[1])[:12]: print(f"  [nofn] {c:36}{n}")
json.dump({'mismatch':rows,'nofn':nofn},open('D:/WORKSPACE/fakeToys/docs/audits/three-ends-2026-09-20/_arity_audit2.json','w',encoding='utf-8'))
