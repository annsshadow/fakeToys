import json, os
base="docs/laiyipao/audits/round31-postdeploy/data"
cfg=json.load(open(base+"/config.json",encoding="utf-8"))
rx=cfg["reactions"]
print("VERSION="+str(cfg.get("version")))
print("REACTION_COUNT="+str(len(rx) if isinstance(rx,list) else len(rx.keys())))
items = rx if isinstance(rx,list) else [dict(v,id=k) for k,v in rx.items()]
for r in items:
  print("RX="+json.dumps({k:r.get(k) for k in ("id","key","name","spread_stacks","armor_shred_permille","knockback")},ensure_ascii=False))
