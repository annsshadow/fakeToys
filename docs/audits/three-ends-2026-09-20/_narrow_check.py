#!/usr/bin/env python
# -*- coding: utf-8 -*-
# For O2_0PARAM traps in a crate: determine if the handler's Path binding(s) are USED in the body.
# unused -> NARROW_SAFE (drop the Path extractor). used -> needs query/body rework (defer).
import re, glob, sys, json
CRATE=sys.argv[1] if len(sys.argv)>1 else "program_center"
CR="oa4rust/crates/"+CRATE+"/src"
TRAPS="docs/audits/three-ends-2026-09-20/_arity_traps_baseline.txt"

# gather all handler names registered as literal traps for this crate
rtxt=open(CR+"/routes.rs",encoding="utf-8",newline="").read()
methcall=re.compile(r'(get|post|put|delete|patch)\(\s*([A-Za-z0-9_:]+)')
route_lines=re.findall(r'\.route\(\s*"([^"]+)"\s*,\s*(.*)', rtxt)
handler_of={}  # path -> [handlers]
for path,rest in route_lines:
    for m in methcall.finditer(rest):
        handler_of.setdefault(path,[]).append(m.group(2).split("::")[-1])

# load all lib text
alltxt=""
for f in glob.glob(CR+"/**/*.rs",recursive=True):
    alltxt+=open(f,encoding="utf-8",newline="").read()+"\n"

def sig_and_body(name):
    m=re.search(r'pub async fn %s\s*\('%re.escape(name),alltxt)
    if not m: return None,None
    i=m.end(); depth=1; sig=[]
    while i<len(alltxt) and depth>0:
        c=alltxt[i]
        if c=='(':depth+=1
        elif c==')':depth-=1
        if depth>0: sig.append(c)
        i+=1
    # body: from next { to matching }
    j=alltxt.find("{",i); depth=1; k=j+1
    while k<len(alltxt) and depth>0:
        c=alltxt[k]
        if c=='{':depth+=1
        elif c=='}':depth-=1
        k+=1
    return "".join(sig), alltxt[j+1:k-1]

def path_binds(sig):
    binds=[]
    for pm in re.finditer(r'Path\(\s*(\(?)([^)]*?)\)\s*:', sig):
        inner=pm.group(2)
        for x in inner.replace("(","").replace(")","").split(","):
            x=x.strip()
            if x and x!="_": binds.append(x)
    return binds

# read O2_0PARAM handlers for this crate from a fresh classify? simpler: iterate literal-trap routes
traps=[ln.strip() for ln in open(TRAPS,encoding="utf-8") if "/"+CRATE.split("_")[0] in ln]
safe=[]; used=[]; noinfo=[]
seen=set()
# handler -> min slot count across its registered paths
hpaths={}
for path,hs in handler_of.items():
    for h in hs: hpaths.setdefault(h,[]).append(path)
for path,hs in handler_of.items():
    for h in hs:
        if h in seen: continue
        seen.add(h)
        # only arity-trap handlers: EVERY registered path has fewer {slots} than Path binds
        sig,body=sig_and_body(h)
        if sig is None: continue
        binds=path_binds(sig)
        if not binds: continue
        maxslots=max(p.count("{") for p in hpaths[h])
        if maxslots>=len(binds): continue  # a matching-arity route exists -> not a trap
        usedb=[b for b in binds if re.search(r'\b%s\b'%re.escape(b),body)]
        if not usedb: safe.append((h,binds))
        else: used.append((h,binds,usedb))

print("NARROW_SAFE (Path binding unused in body):",len(safe))
for h,b in safe: print("  ",h,b)
print("\nUSED (Path binding referenced -> defer):",len(used))
for h,b,u in used[:40]: print("  ",h,"used=",u)
