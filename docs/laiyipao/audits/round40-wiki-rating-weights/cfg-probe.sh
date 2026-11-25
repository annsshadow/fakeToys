#!/bin/bash
cd "$(dirname "$0")" || exit 1
python - <<'PY'
import json,hashlib
p="data/config.json"
b=open(p,"rb").read()
d=json.loads(b)
print("CFG_SHA256="+hashlib.sha256(b).hexdigest())
root=d.get("data") if isinstance(d.get("data"),dict) else d
rw=root.get("rating_weights")
print("RATING_WEIGHTS="+json.dumps(rw,ensure_ascii=False,sort_keys=True))
for k in ("skills","enemies","levels","reactions","composite_skills","recipes","skins","drop_rates","mastery_families","gems","gem_qualities"):
 v=root.get(k)
 if isinstance(v,dict): print(k,"dict",len(v))
 elif isinstance(v,list): print(k,"list",len(v))
PY
