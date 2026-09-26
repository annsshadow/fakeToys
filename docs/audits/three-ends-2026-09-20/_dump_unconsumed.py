import json, os, sys
HERE=os.path.dirname(os.path.abspath(__file__))
backend=json.load(open(os.path.join(HERE,"backend_routes.json"),encoding="utf-8"))
rec=json.load(open(os.path.join(HERE,"api_reconcile.json"),encoding="utf-8"))
consumed=set()
for end in rec:
    for cat in ("exact","param-ok"):
        for c in rec[end][cat]:
            h=c.get("hit")
            if h: consumed.add((({"UPLOAD":"POST","DOWNLOAD":"GET"}).get(c["method"],c["method"]),h))
prefix=sys.argv[1] if len(sys.argv)>1 else "/api/bbs"
rows=[]
for _crate,routes in backend.items():
    for r in routes:
        p=r["path"]; m=r["method"]
        if not p.startswith(prefix) or "mock" in p: continue
        if (m,p) not in consumed: rows.append((m,p))
rows=sorted(set(rows))
print(f"# unconsumed under {prefix}: {len(rows)}")
for m,p in rows: print(m,p)
