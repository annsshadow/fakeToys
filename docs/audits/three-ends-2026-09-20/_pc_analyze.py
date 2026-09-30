#!/usr/bin/env python
# -*- coding: utf-8 -*-
import json, re

CRATE = "program_center"
ROUTES = "oa4rust/crates/%s/src/routes.rs" % CRATE
LIB    = "oa4rust/crates/%s/src/lib.rs" % CRATE
TRAPS  = "docs/audits/three-ends-2026-09-20/_arity_traps_baseline.txt"
INV    = "docs/audits/o2server-endpoint-inventory.json"

d = json.load(open(INV, encoding="utf-8"))
mods = d.get("modules", d)
o2 = mods.get("x_program_center", {}).get("endpoints", [])

# build o2 templates: (method, tuple_of_segments_with_None_for_wildcard)
o2_templates = []  # (method, segs_with_None, paramcount)
for e in o2:
    p = e.get("path",""); m = e.get("method","GET")
    segs = [s for s in p.split("/") if s != ""]
    tmpl = [None if s=="{}" else s for s in segs]
    pc = sum(1 for s in tmpl if s is None)
    o2_templates.append((m, tmpl, pc))

def match_o2(meth, path):
    """path = full registered path like /api/program_center/a/b/c (all literal).
    Return (newpath_with_params, paramcount) best o2 template match or None."""
    segs = [s for s in path.split("/") if s != ""]
    # strip leading api/program_center
    assert segs[0]=="api" and segs[1]=="program_center", path
    body = segs[2:]
    cands = []
    for (m, tmpl, pc) in o2_templates:
        if m != meth: continue
        if len(tmpl) != len(body): continue
        ok = True
        for a,b in zip(tmpl, body):
            if a is None: continue
            if a != b: ok=False; break
        if ok: cands.append((tmpl, pc))
    if not cands: return None
    # prefer the template with MOST wildcards that still matches (real param route),
    # but if a pure-literal (pc==0) exact match exists prefer it as ground truth 0-param
    # We return the max-wildcard candidate for conversion; caller handles pc==0.
    cands.sort(key=lambda c: -c[1])
    tmpl, pc = cands[0]
    out=[]; n=0
    for a,b in zip(tmpl, body):
        if a is None:
            out.append("{p%d}"%n); n+=1
        else:
            out.append(b)
    return "/api/program_center/" + "/".join(out), pc

rtxt = open(ROUTES, encoding="utf-8", newline="").read()
route_re = re.compile(r'\.route\(\s*"([^"]+)"\s*,\s*(get|post|put|delete|patch)\(([A-Za-z0-9_:]+)\)')
route_by = {}
existing_param_struct = set()   # structure-tuples (method-agnostic: matchit tree conflicts ignore method)
def _struct(p):
    return tuple("*" if s.startswith("{") else s for s in p.split("/") if s)
# capture EVERY registered path literal (method-agnostic) incl. chained .method() forms
for pm in re.finditer(r'\.route\(\s*"([^"]+)"', rtxt):
    path = pm.group(1)
    if "{" in path:
        existing_param_struct.add(_struct(path))
for m in route_re.finditer(rtxt):
    path, meth, handler = m.group(1), m.group(2).upper(), m.group(3).split("::")[-1]
    route_by[(meth, path)] = handler

ltxt = open(LIB, encoding="utf-8", newline="").read()
def arity_of(name):
    m = re.search(r'pub async fn %s\s*\(' % re.escape(name), ltxt)
    if not m: return None
    i = m.end(); depth = 1; buf = []
    while i < len(ltxt) and depth > 0:
        c = ltxt[i]
        if c == '(': depth += 1
        elif c == ')': depth -= 1
        if depth > 0: buf.append(c)
        i += 1
    sig = "".join(buf)
    total = 0; found = False
    for pm in re.finditer(r'Path\(\s*(\(?)', sig):
        found = True
        if pm.group(1) == '(':
            j = pm.end()-1; dep=1; el=""
            while j < len(sig) and dep>0:
                cc=sig[j]
                if cc=='(':dep+=1
                elif cc==')':dep-=1
                if dep>0: el+=cc
                j+=1
            total += len([x for x in el.split(",") if x.strip()])
        else:
            total += 1
    return total if found else 0

traps = [ln.strip() for ln in open(TRAPS, encoding="utf-8") if 'program_center' in ln]
# EXCLUDE: static-literal sibling shadows the param route for the exact path, so removing
# the trap makes the other verb fall to the static route -> 405 (breaks u2_method_chains).
EXCLUDE = {("PUT","/api/program_center/module/write/flag")}
convert_only=[]; remove_dup=[]; convert_widen=[]; convert_narrow=[]; nomatch=[]
def newstruct(np):
    return tuple("*" if s.startswith("{") else s for s in np.split("/") if s)
for t in traps:
    meth, path = t.split(" ",1)
    if (meth,path) in EXCLUDE: nomatch.append((meth,path,route_by.get((meth,path)),"excluded-shadow")); continue
    o2m = match_o2(meth,path); handler = route_by.get((meth,path))
    if o2m is None: nomatch.append((meth,path,handler,"no-o2-match")); continue
    newpath, P = o2m
    H = arity_of(handler) if handler else None
    if H is None: nomatch.append((meth,path,handler,"handler-not-found")); continue
    if P==0: nomatch.append((meth,path,handler,"o2-0-param")); continue
    rec=(meth,path,newpath,handler,P,H)
    # if a param route with SAME structure already exists -> converting collides; remove trap instead
    if newstruct(newpath) in existing_param_struct:
        remove_dup.append(rec); continue
    if P==H: convert_only.append(rec)
    elif P>H: convert_widen.append(rec)
    else: convert_narrow.append(rec)

def dump(name,lst,lim=80):
    print("\n=== %s: %d ===" % (name,len(lst)))
    for r in lst[:lim]:
        if len(r)==6:
            meth,path,newpath,handler,P,H=r
            print("  %-6s %s -> %s [P=%d H=%d fn=%s]"%(meth,path,newpath,P,H,handler))
        else:
            print("  %-6s %s (%s) %s"%(r[0],r[1],r[2] or '?',r[3]))

dump("CONVERT_ONLY",convert_only)
dump("REMOVE_DUP",remove_dup)
dump("CONVERT_WIDEN",convert_widen)
dump("CONVERT_NARROW",convert_narrow)
dump("NOMATCH",nomatch,120)
print("\nSUMMARY only=%d removedup=%d widen=%d narrow=%d nomatch=%d total=%d"%(
    len(convert_only),len(remove_dup),len(convert_widen),len(convert_narrow),len(nomatch),len(traps)))

import sys as _sys
if "--apply" in _sys.argv:
    raw = open(ROUTES,"rb").read(); txt = raw.decode("utf-8")
    lines = txt.split("\n")
    # 1) REMOVE_DUP: drop the whole .route line matching (meth, oldpath)
    methfn={"GET":"get","POST":"post","PUT":"put","DELETE":"delete","PATCH":"patch"}
    removed=0
    def line_matches(line, meth, oldp):
        return ('.route(' in line and ('"%s"'%oldp) in line
                and ('%s('%methfn[meth]) in line)
    keep=[]
    rm_targets=[(meth,oldp) for (meth,oldp,_,_,_,_) in remove_dup]
    for ln in lines:
        drop=False
        for (meth,oldp) in rm_targets:
            if line_matches(ln,meth,oldp): drop=True; break
        if drop: removed+=1
        else: keep.append(ln)
    txt="\n".join(keep)
    # 2) CONVERT_ONLY: replace path literal (global, all methods -> same newpath)
    applied=0; problems=[]
    seen=set()
    for (meth,oldp,newp,handler,P,H) in convert_only:
        if oldp in seen: continue
        seen.add(oldp)
        needle='"%s"'%oldp; repl='"%s"'%newp
        c=txt.count(needle)
        if c==0: problems.append((oldp,"count=0")); continue
        txt=txt.replace(needle,repl); applied+=c
    open(ROUTES,"wb").write(txt.encode("utf-8"))
    print("\nAPPLIED convert=%d removed=%d problems=%d"%(applied,removed,len(problems)))
    for p in problems: print("  PROBLEM",p)


