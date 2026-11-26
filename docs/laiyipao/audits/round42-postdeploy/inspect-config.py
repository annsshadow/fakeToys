# -*- coding: utf-8 -*-
import json,sys,io
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8",errors="replace")
d=json.load(open(sys.argv[1],encoding="utf-8"))
print("KEY_TOP=",sorted(d.keys()))
print("VERSION=",d.get("version"))
print("REQUIRE_LOGIN=",d.get("require_login"))
print("CHAPTERS=",json.dumps(d.get("chapters"),ensure_ascii=False))
lv=d.get("levels")
print("LEVELS_TYPE=",type(lv).__name__)
if isinstance(lv,list):
    print("LEVELS_LEN=",len(lv))
    print("LEVEL0=",json.dumps(lv[0],ensure_ascii=False) if lv else "EMPTY")
    print("LEVEL_LAST=",json.dumps(lv[-1],ensure_ascii=False) if lv else "EMPTY")
elif isinstance(lv,dict):
    print("LEVELS_KEYS=",list(lv.keys())[:10])
    k0=next(iter(lv),None)
    print("LEVEL0=",json.dumps(lv[k0],ensure_ascii=False) if k0 else "EMPTY")