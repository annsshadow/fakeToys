# -*- coding: utf-8 -*-
import json,sys,io
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8",errors="replace")
v=json.load(open(sys.argv[1],encoding="utf-8"))
while isinstance(v,str): v=json.loads(v)
print("KEYS=",sorted(v.keys()))
rows=v.get("rows") or []
print("ROW_COUNT=",len(rows))
print("HEADERS=",json.dumps(v.get("headers"),ensure_ascii=False))
print("ROW0=",json.dumps(rows[0] if rows else None,ensure_ascii=False))
print("ROW20=",json.dumps(rows[20] if len(rows)>20 else None,ensure_ascii=False))
print("ROW99=",json.dumps(rows[-1] if rows else None,ensure_ascii=False))
print("AI=",repr(v.get("ai")))