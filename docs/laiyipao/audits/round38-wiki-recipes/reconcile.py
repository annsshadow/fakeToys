import json,io,sys,hashlib
base="docs/laiyipao/audits/round38-wiki-recipes/data/"
raw=io.open(base+"config.json","rb").read(); cfg=json.loads(raw.decode("utf-8"))
print("CFG_SHA="+hashlib.sha256(raw).hexdigest())
print("VERSION="+str(cfg.get("version")))
EMPTY="—"
def pos(v): return isinstance(v,int) and not isinstance(v,bool) and v>0
def names():
 m={}
 for key in ("skills","composite_skills"):
  for s in cfg.get(key,[]) or []:
   if isinstance(s,dict) and pos(s.get("id")) and isinstance(s.get("name"),str) and s.get("name"): m[s["id"]]=s["name"]
 return m
NM=names()
def nm(i): return NM.get(i,"（未知技能#"+str(i)+"）") if pos(i) else EMPTY
rec=[r for r in cfg.get("recipes",[]) or [] if isinstance(r,dict) and pos(r.get("output")) and pos(r.get("a")) and pos(r.get("b")) and pos(r.get("out_tier"))]
expect=[[nm(r["output"]),nm(r["a"]),nm(r["b"]),str(r["out_tier"])+" 阶"] for r in rec]
t2=sum(1 for r in rec if r["out_tier"]==2); t3=sum(1 for r in rec if r["out_tier"]==3)
final="共 "+str(len(rec))+" 条配方：二阶 "+str(t2)+" 条、三阶 "+str(t3)+" 条"
basis="依据：公开 /api/v1/config 的 recipes（与服务端 SeedRecipe / SeedRecipes 同源）；仅展示配方，不改熔炼结算"
honesty="能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准"
HDR=["产物","材料A","材料B","阶数"]
print("EXPECT_AI="+final)
def load(vp):
 raw=io.open(base+"probe-"+vp+".json",encoding="utf-8").read().strip(); dec=json.JSONDecoder(); obj,end=dec.raw_decode(raw)
 while isinstance(obj,str): obj,end=dec.raw_decode(obj)
 return obj
allok=True
for vp in sys.argv[1:]:
 p=load(vp); mis=[]; rows=p.get("rows",[])
 if p.get("headers")!=HDR: mis.append("HEADERS "+repr(p.get("headers")))
 if len(rows)!=len(expect): mis.append("ROW_COUNT "+str(len(rows))+" EXP "+str(len(expect)))
 for i,(want,got) in enumerate(zip(expect,rows)):
  for j,(w,g) in enumerate(zip(want,got)):
   if w!=g: mis.append("ROW%d C%d EXP=%s GOT=%s"%(i,j,repr(w),repr(g)))
 text=p.get("aiNote","")+"|"+p.get("conclusion","")+"|"+p.get("basis","")+"|"+p.get("honesty","")+"|"+p.get("ctaText","")
 four=(final in p.get("conclusion","") and basis in p.get("basis","") and honesty in p.get("honesty","") and "查看合成配方 →" in text and "AI 机制概览" in text)
 wrappers=p.get("wrap",[]) or []
 hasOverflow=any(w.get("sw",0)>w.get("cw",0) for w in wrappers)
 docOk=p.get("docW",0)<=p.get("innerW",0)+1
 scrollOk=len(wrappers)>0 and docOk and ((not hasOverflow) or any(w.get("after",0)>w.get("before",0) for w in wrappers if w.get("sw",0)>w.get("cw",0)))
 ctaOk=p.get("ctaText")=="查看合成配方 →" and p.get("ctaVisible") and p.get("ctaHref")=="#/wiki?tab=skills&anchor=composite"
 r36ok=p.get("r36Rows")==36 and p.get("r36Visible")
 ok=(not mis) and four and scrollOk and docOk and ctaOk and r36ok and not p.get("badText") and p.get("sectionVisible")
 allok=allok and ok
 print("VP=%s ROWS=%d MISMATCHES=%d AI_OK=%s FOUR_OK=%s BAD=%s DOC=%s/%s SCROLL=%s R36=%s CTA=%s VERDICT=%s"%(vp,len(rows),len(mis),final in p.get("conclusion",""),four,p.get("badText"),p.get("docW"),p.get("innerW"),scrollOk,r36ok,repr(p.get("ctaText")),"PASS" if ok else "FAIL"))
 for m in mis[:12]: print("  MIS "+m)
 for i in (0,2,17):
  if i<len(rows): print("SAMPLE VP=%s ROW%d=%s"%(vp,i,repr(rows[i])))
print("ALL_RECONCILE_OK="+str(allok))
