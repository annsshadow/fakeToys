# -*- coding: utf-8 -*-
import json,sys,io
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8",errors="replace")
D="docs/laiyipao/audits/round42-postdeploy/data"
cfg=json.load(open(D+"/config.json",encoding="utf-8"))
all_ids={l["id"] for l in cfg.get("levels",[])}
observed=set()
for vp in ["390","768","1280"]:
    parts=[]
    for i in range(1,7):
        v=json.load(open("%s/pd-level-%s-ch%d.json"%(D,vp,i),encoding="utf-8"))
        while isinstance(v,str): v=json.loads(v)
        rows=v.get("rows") or []
        ids=[int(r[0]) for r in rows if r and str(r[0]).strip().isdigit()]
        observed.update(ids)
        parts.append("%d:%d[%s..%s]"%(i,len(rows),ids[0] if ids else "-",ids[-1] if ids else "-"))
    print(vp," ".join(parts))
print("OBSERVED=",len(observed))
print("MISSING=",sorted(all_ids-observed))