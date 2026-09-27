#!/usr/bin/env python
# -*- coding: utf-8 -*-
# General arity-trap classifier across ALL crates.
# For each all-literal trap route, find its crate's routes.rs registration + handler arity,
# match it to an o2server endpoint template by SUFFIX alignment (position structure), and classify:
#   REMOVE_DUP : an existing {param} route in the SAME crate already covers the o2 structure
#   CONVERT    : no existing sibling; convert literal segs at o2 {} positions to {pN} (needs P==H)
#   defer buckets otherwise.
import json, re, glob, os, sys

TRAPS = "docs/audits/three-ends-2026-09-20/_arity_traps_baseline.txt"
INV   = "docs/audits/o2server-endpoint-inventory.json"
ONLY_CRATE = None
for a in sys.argv[1:]:
    if a.startswith("--crate="): ONLY_CRATE=a.split("=",1)[1]

d = json.load(open(INV, encoding="utf-8"))
mods = d.get("modules", d)
# o2 templates: list of (method, segs_with_None_for_wildcard)
o2_templates = []
for k in mods:
    for e in mods[k].get("endpoints", []):
        p=e.get("path",""); m=e.get("method","GET")
        segs=[s for s in p.split("/") if s!=""]
        tmpl=[None if s=="{}" else s for s in segs]
        o2_templates.append((m, tmpl, k))

# --- scan every crate ---
CR = "oa4rust/crates"
route_re = re.compile(r'\.route\(\s*"([^"]+)"\s*,\s*((?:get|post|put|delete|patch)\([^)]*\)(?:\.(?:get|post|put|delete|patch)\([^)]*\))*)\)')
# capture first handler per method chain
methcall_re = re.compile(r'(get|post|put|delete|patch)\(\s*([A-Za-z0-9_:]+)')

def struct(p):
    return tuple("*" if s.startswith("{") else s for s in p.split("/") if s)

crate_of = {}          # (method,path) -> crate
route_handler = {}     # (method,path) -> handler
from collections import defaultdict as _dd
handler_paths = _dd(set)   # handler -> set of registered paths (any method)
crate_paramstruct = {} # crate -> set(structure) for {param} routes
crate_routesfile = {}  # crate -> routes.rs path
crate_arity = {}       # crate -> {handler: arity}

def parse_arity_file(txt, out):
    for m in re.finditer(r'pub async fn (\w+)\s*\(', txt):
        name=m.group(1); i=m.end(); depth=1; buf=[]
        while i<len(txt) and depth>0:
            c=txt[i]
            if c=='(':depth+=1
            elif c==')':depth-=1
            if depth>0:buf.append(c)
            i+=1
        sig="".join(buf); total=0; found=False
        for pm in re.finditer(r'Path\(\s*(\(?)', sig):
            found=True
            if pm.group(1)=='(':
                j=pm.end()-1; dep=1; el=""
                while j<len(sig) and dep>0:
                    cc=sig[j]
                    if cc=='(':dep+=1
                    elif cc==')':dep-=1
                    if dep>0:el+=cc
                    j+=1
                total+=len([x for x in el.split(",") if x.strip()])
            else: total+=1
        out[name]= total if found else 0

import os as _os
crate_dirs = sorted({p.replace("\\","/").split("/")[2] for p in glob.glob(CR+"/*/src")})
# .route( "path" , ...method(handler)... )  — path and method may span newlines
route_span = re.compile(r'\.route\(\s*"([^"]+)"\s*,(.*?)\)\s*(?=\.route\(|;|\}|\.with_state|\.layer|\.fallback|\.nest|\.merge)', re.S)
for crate in crate_dirs:
    # prefer routes.rs as the file to EDIT, but scan all .rs for registrations
    rfile = CR+"/"+crate+"/src/routes.rs"
    crate_routesfile[crate] = rfile if _os.path.exists(rfile) else (CR+"/"+crate+"/src/lib.rs")
    ps=set(); ar={}
    for f in glob.glob(CR+"/"+crate+"/src/**/*.rs", recursive=True):
        f=f.replace("\\","/")
        try: txt=open(f,encoding="utf-8",newline="").read()
        except Exception: continue
        parse_arity_file(txt, ar)
        # scan registrations: simple single-line first
        for pm in re.finditer(r'\.route\(\s*"([^"]+)"\s*,\s*(.*)', txt):
            path=pm.group(1); rest=pm.group(2)
            if "{" in path: ps.add(struct(path))
            for mm in methcall_re.finditer(rest[:200]):
                crate_of[(mm.group(1).upper(),path)]=crate
                route_handler[(mm.group(1).upper(),path)]=mm.group(2).split("::")[-1]
                handler_paths[mm.group(2).split("::")[-1]].add(path)
        # multi-line: path on its own line, method on next
        for pm in re.finditer(r'\.route\(\s*"([^"]+)"\s*,\s*\n\s*((?:get|post|put|delete|patch)\([^\n]*)', txt):
            path=pm.group(1); rest=pm.group(2)
            if "{" in path: ps.add(struct(path))
            for mm in methcall_re.finditer(rest):
                crate_of[(mm.group(1).upper(),path)]=crate
                route_handler[(mm.group(1).upper(),path)]=mm.group(2).split("::")[-1]
                handler_paths[mm.group(2).split("::")[-1]].add(path)
    crate_paramstruct[crate]=ps
    crate_arity[crate]=ar


# GLOBAL param-route structures (merged router is what matchit sees -> cross-crate dup collides)
global_paramstruct=set()
for ps in crate_paramstruct.values(): global_paramstruct|=ps

def match_o2_suffix(meth, path):
    segs=[s for s in path.split("/") if s!=""]
    assert segs[0]=="api", path
    body=segs[1:]  # drop 'api'
    best=None
    for (m,tmpl,mod) in o2_templates:
        if m!=meth: continue
        L=len(tmpl)
        if L>len(body): continue
        tail=body[-L:]
        ok=True
        for a,b in zip(tmpl,tail):
            if a is None: continue
            if a!=b: ok=False;break
        if not ok: continue
        pc=sum(1 for s in tmpl if s is None)
        # prefer longer template, then more params
        key=(L,pc)
        if best is None or key>best[0]:
            prefix=body[:len(body)-L]
            newtail=[]; n=0
            for a,b in zip(tmpl,tail):
                if a is None: newtail.append("{p%d}"%n); n+=1
                else: newtail.append(b)
            newpath="/api/"+"/".join(prefix+newtail)
            best=(key,newpath,pc,mod)
    return best  # (key,newpath,pc,mod) or None

traps=[ln.strip() for ln in open(TRAPS,encoding="utf-8")]
from collections import defaultdict
buckets=defaultdict(list)
for t in traps:
    meth,path=t.split(" ",1)
    crate=crate_of.get((meth,path))
    if crate is None: buckets["NO_CRATE"].append((meth,path,None,None,None,None)); continue
    if ONLY_CRATE and crate!=ONLY_CRATE: continue
    h=route_handler.get((meth,path)); H=crate_arity.get(crate,{}).get(h)
    # PRIORITY: same handler also registered on an arity-CORRECT path (slots==H) elsewhere?
    # Then THIS trap path is a redundant codegen-garbled duplicate -> safe to remove; the
    # correct-arity registration serves. Probe unreachable check confirms nothing lost.
    if H is not None and H>0:
        sib=[p for p in handler_paths.get(h,()) if p!=path and p.count("{")==H]
        if sib:
            buckets["DUP_SIBLING"].append((meth,path,crate,h,sib[0],H)); continue
    mo=match_o2_suffix(meth,path)
    if mo is None: buckets["NOMATCH"].append((meth,path,crate,h,None,H)); continue
    _,newpath,P,mod=mo
    if P==0: buckets["O2_0PARAM"].append((meth,path,crate,h,newpath,H)); continue
    rec=(meth,path,crate,h,newpath,H,P,mod)
    same_crate = struct(newpath) in crate_paramstruct.get(crate,set())
    in_global  = struct(newpath) in global_paramstruct
    if same_crate:
        # sibling in the SAME crate's router -> removing the literal falls back safely,
        # crate-local reachability test still passes.
        buckets["REMOVE_DUP"].append(rec)
    elif in_global:
        # sibling only in ANOTHER crate -> merged router routes fine after removal, but this
        # crate's own unit test would 404. Needs per-endpoint judgement -> defer.
        buckets["CROSS_DUP"].append(rec)
    elif H is None: buckets["NO_HANDLER_ARITY"].append(rec)
    elif P==H: buckets["CONVERT"].append(rec)
    elif P>H: buckets["CONVERT_WIDEN"].append(rec)
    else: buckets["CONVERT_NARROW"].append(rec)

for name in ["DUP_SIBLING","REMOVE_DUP","CONVERT","CROSS_DUP","CONVERT_WIDEN","CONVERT_NARROW","O2_0PARAM","NOMATCH","NO_HANDLER_ARITY","NO_CRATE"]:
    lst=buckets.get(name,[])
    # per-crate count
    from collections import Counter
    cc=Counter(r[2] for r in lst if len(r)>2 and r[2])
    print("== %s : %d  %s"%(name,len(lst),dict(cc)))

if "--dup-apply" in sys.argv and ONLY_CRATE:
    methfn={"GET":"get","POST":"post","PUT":"put","DELETE":"delete","PATCH":"patch"}
    targets=[(r[0],r[1]) for r in buckets.get("DUP_SIBLING",[]) if r[2]==ONLY_CRATE]
    files=sorted(set(glob.glob(CR+"/"+ONLY_CRATE+"/src/**/*.rs", recursive=True)))
    def remove_block(txt, meth, path):
        """remove a `.route("path", meth(...))` call (single/multi-line). Returns (txt, n)."""
        n=0; needle='"%s"'%path; mfn=methfn[meth]
        while True:
            i=txt.find(needle)
            found=False
            while i>=0:
                rs=txt.rfind(".route(",0,i)
                if rs<0: break
                popen=txt.find("(",rs); depth=1; k=popen+1
                while k<len(txt) and depth>0:
                    c=txt[k]
                    if c=='(':depth+=1
                    elif c==')':depth-=1
                    k+=1
                block=txt[rs:k]
                # this block must contain our needle and the method call
                if needle in block and re.search(r'\b%s\('%mfn, block):
                    ls=txt.rfind("\n",0,rs)+1
                    e=k
                    while e<len(txt) and txt[e] in " \t": e+=1
                    if e<len(txt) and txt[e]=="\n": e+=1
                    txt=txt[:ls]+txt[e:]; n+=1; found=True; break
                else:
                    i=txt.find(needle,i+1)
            if not found: break
        return txt,n
    total=0
    for f in files:
        f=f.replace("\\","/")
        txt=open(f,"rb").read().decode("utf-8"); fn=0
        for (m,p) in targets:
            txt,c=remove_block(txt,m,p); fn+=c
        if fn: open(f,"wb").write(txt.encode("utf-8")); total+=fn
    print("DUP-APPLIED crate=%s removed=%d (targets=%d)"%(ONLY_CRATE,total,len(targets)))

if "--dump" in sys.argv:
    tgt=sys.argv[sys.argv.index("--dump")+1]
    for r in buckets.get(tgt,[]):
        print(r)

if "--apply" in sys.argv and ONLY_CRATE:
    methfn={"GET":"get","POST":"post","PUT":"put","DELETE":"delete","PATCH":"patch"}
    rp=crate_routesfile[ONLY_CRATE]
    raw=open(rp,"rb").read(); txt=raw.decode("utf-8")
    EXCL=set()
    ef="docs/audits/three-ends-2026-09-20/_arity_exclude.txt"
    if os.path.exists(ef):
        for ln in open(ef,encoding="utf-8"):
            ln=ln.strip()
            if ln and " " in ln: EXCL.add(tuple(ln.split(" ",1)))
    # REMOVE_DUP: drop the .route line matching (meth, path)
    lines=txt.split("\n"); keep=[]; removed=0
    rmset=set((r[0],r[1]) for r in buckets.get("REMOVE_DUP",[]) if r[2]==ONLY_CRATE and (r[0],r[1]) not in EXCL)
    def matches(line,meth,path):
        return '.route(' in line and ('"%s"'%path) in line and ('%s('%methfn[meth]) in line
    for ln in lines:
        drop=any(matches(ln,m,p) for (m,p) in rmset)
        if drop: removed+=1
        else: keep.append(ln)
    txt="\n".join(keep)
    # CONVERT: replace literal path with param path. Only safe when this literal path has a
    # SINGLE method registration (a static sibling under another verb would shadow the new param route).
    conv=0; conv_skip=[]
    for r in buckets.get("CONVERT",[]):
        if r[2]!=ONLY_CRATE: continue
        meth,path,crate,h,newpath,H,P,mod=r
        if (meth,path) in EXCL: continue
        methods_here=set(m for (m,p) in route_handler if p==path)
        if len(methods_here)>1:
            conv_skip.append((meth,path,"multi-method-static-shadow")); continue
        newlines=[]; done=False
        for ln in txt.split("\n"):
            if matches(ln,meth,path) and not done:
                ln=ln.replace('"%s"'%path,'"%s"'%newpath); conv+=1; done=True
            newlines.append(ln)
        txt="\n".join(newlines)
    open(rp,"wb").write(txt.encode("utf-8"))
    print("APPLIED crate=%s removed=%d converted=%d skip=%d"%(ONLY_CRATE,removed,conv,len(conv_skip)))
    for s in conv_skip: print("  SKIP-CONVERT",s)


