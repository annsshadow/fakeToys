# -*- coding: utf-8 -*-
import json, os, sys
base="docs/laiyipao/audits/round31-postdeploy/data"
cfg=json.load(open(base+"/config.json",encoding="utf-8"))
rx=cfg["reactions"]
items = rx if isinstance(rx,list) else [dict(v,key=k) for k,v in rx.items()]
CN={"steam_burst":"蒸汽爆发","overheat":"过热","burn_cloud":"燃烧云","superconduct":"超导","flash_freeze":"急速冻结","corrosion_spread":"腐蚀扩散","armor_break":"破甲击退"}
def fmt_stack(v): return ("传播 %d 层"%v) if v and v>0 else "—"
def fmt_shred(v): return ("%d‰"%v) if v and v>0 else "—"
def fmt_knock(v): return ("%gpx"%(v/1000.0)) if v and v>0 else "—"
expect={}
for r in items:
  name=CN.get(r.get("key"), r.get("name"))
  expect[name]=(fmt_stack(r.get("spread_stacks",0)),fmt_shred(r.get("armor_shred_permille",0)),fmt_knock(r.get("knockback",0)))
print("VERSION="+str(cfg.get("version"))+" CONFIG_RX="+str(len(expect)))
allok=True
for vp in ("desktop","mobile375","mobile390"):
  raw=open(base+"/probe-"+vp+".json",encoding="utf-8").read().strip()
  obj=json.loads(raw)
  if isinstance(obj,str): obj=json.loads(obj)
  rows=obj["rows"]; heads=obj["headers"]
  si,ai,ki=heads.index("层数传播"),heads.index("削甲"),heads.index("击退")
  bad=[]; checked=0
  for row in rows:
    name=row[0]
    if name not in expect: bad.append("unknown:"+name); continue
    exp=expect[name]; got=(row[si],row[ai],row[ki])
    checked+=1
    if tuple(got)!=tuple(exp): bad.append(name+" exp="+str(exp)+" got="+str(got))
  ok=(len(bad)==0 and checked==7 and obj.get("badToken")==False and obj["page"]["scrollWidth"]==obj["page"]["clientWidth"]==obj["page"]["innerWidth"])
  allok = allok and ok
  print("VP="+vp+" CHECKED="+str(checked)+" OVERFLOW="+(str(obj["page"]["scrollWidth"]-obj["page"]["clientWidth"]))+" SCROLL="+json.dumps(obj.get("scrollTest"))+" BAD="+json.dumps(bad,ensure_ascii=False)+" OK="+str(ok))
print("FULL_7X3_RECONCILE_OK="+str(allok))
