import json,io,sys,hashlib
base="docs/laiyipao/audits/round37-postdeploy/data/"
raw=io.open(base+"config.json","rb").read(); cfg=json.loads(raw.decode("utf-8"))
print("CFG_SHA="+hashlib.sha256(raw).hexdigest())
print("VERSION="+str(cfg.get("version")))
EMPTY="—"
KIND={"element_cap":"元素层数上限","reaction_mult":"反应倍率","heat_cap":"热量上限","extra_slot":"额外插槽","skill_damage":"技能伤害","crit":"暴击","armor":"护甲","mechanic":"机制改造"}
def pos(v): return isinstance(v,int) and not isinstance(v,bool) and v>0
def kind_text(k): return KIND.get(k,k) if isinstance(k,str) and k else EMPTY
def value_text(k,v):
    if not pos(v): return EMPTY
    if not (isinstance(k,str) and k): return EMPTY
    if k=="heat_cap": return "+"+str(v)+"‰"
    if k=="extra_slot": return "+1 槽"
    if k=="element_cap": return "+"+str(v)+"层"
    if k in ("reaction_mult","skill_damage","crit","armor","mechanic"): return "+"+str(v)+"‰"
    return str(v)+"（单位以游戏内结算为准）"
def prereq_text(f,l):
    fam=f if isinstance(f,str) and f else None
    if fam is None and l==0: return "无前置"
    if fam is not None and pos(l): return "前置："+fam+" 第"+str(l)+"层"
    return EMPTY
fams=[f for f in cfg.get("mastery_families",[]) if isinstance(f,dict) and isinstance(f.get("family"),str) and f.get("family")]
expect=[]; nodes=0; gated=0; slots=0; kinds={}
for f in fams:
    for n in f.get("nodes",[]) or []:
        if not isinstance(n,dict) or not isinstance(n.get("name"),str) or not n.get("name"): continue
        nodes+=1
        if not prereq_text(n.get("prereq_family"),n.get("prereq_layer")).startswith("无前置"): gated+=1
        if n.get("kind")=="extra_slot": slots+=1
        kinds[str(n.get("kind"))]=kinds.get(str(n.get("kind")),0)+1
        expect.append([str(f.get("name","")),str(n.get("layer")),str(n.get("slot")),str(n.get("name")),kind_text(n.get("kind")),value_text(n.get("kind"),n.get("value")),prereq_text(n.get("prereq_family"),n.get("prereq_layer"))])
final="共 "+str(len(fams))+" 系："+str(nodes)+" 个节点，含前置 "+str(gated)+" 个，额外插槽 "+str(slots)+" 个"
print("EXPECT_AI="+final)
print("KIND_COUNTS="+repr(kinds))
basis="依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准"
honesty="辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准"
HDR=["系","层","槽","节点","效果","数值","前置"]
def load(vp):
    raw=io.open(base+"probe-"+vp+".json",encoding="utf-8").read().strip(); dec=json.JSONDecoder(); obj,end=dec.raw_decode(raw)
    while isinstance(obj,str): obj,end=dec.raw_decode(obj)
    return obj
allok=True; samples=[]
for vp in sys.argv[1:]:
    p=load(vp); mis=[]; rows=p.get("rows",[])
    if any(h!=HDR for h in p.get("headers",[])): mis.append("HEADERS "+repr(p.get("headers")))
    if len(rows)!=len(expect): mis.append("ROW_COUNT "+str(len(rows))+" EXP "+str(len(expect)))
    for i,(want,got) in enumerate(zip(expect,rows)):
        for j,(w,g) in enumerate(zip(want,got)):
            if w!=g: mis.append("ROW%d C%d EXP=%s GOT=%s"%(i,j,repr(w),repr(g)))
    text=p.get("aiNote","")+"|"+p.get("conclusion","")+"|"+p.get("basis","")+"|"+p.get("honesty","")+"|"+p.get("ctaText","")
    four=(final in text) and (basis in text) and (honesty in text) and ("查看专精树 →" in text) and ("AI 机制概览" in text)
    wrappers=p.get("wrap",[]) or []
    hasOverflow=any(w.get("sw",0)>w.get("cw",0) for w in wrappers)
    scrollOk=len(wrappers)>0 and (p.get("docW",0)<=p.get("innerW",0)+1) and ((not hasOverflow) or any(w.get("after",0)>w.get("before",0) for w in wrappers if w.get("sw",0)>w.get("cw",0)))
    docOk=p.get("docW",0)<=p.get("innerW",0)+1
    ctaOk=p.get("ctaText")=="查看专精树 →" and p.get("ctaVisible") and p.get("ctaHref")=="#/wiki?tab=mastery"
    ok=(not mis) and four and scrollOk and docOk and ctaOk and not p.get("badText") and p.get("sectionVisible")
    allok=allok and ok
    print("VP=%s ROWS=%d MISMATCHES=%d AI_OK=%s FOUR_OK=%s BAD=%s DOC=%s/%s SCROLL=%s CTA=%s VERDICT=%s"%(vp,len(rows),len(mis),final in p.get("conclusion",""),four,p.get("badText"),p.get("docW"),p.get("innerW"),scrollOk,repr(p.get("ctaText")),"PASS" if ok else "FAIL"))
    for m in mis[:12]: print("  MIS "+m)
    for n in rows:
        if n[4]=="热量上限" and len(samples)<3: samples.append("HEAT "+repr(n))
        if n[4]=="反应倍率" and len(samples)<6: samples.append("REACTION "+repr(n))
        if n[4]=="额外插槽" and len(samples)<9: samples.append("SLOT "+repr(n))
        if n[6].startswith("前置：") and len(samples)<12: samples.append("PREREQ "+repr(n))
for s in samples: print("SAMPLE "+s)
print("ALL_RECONCILE_OK="+str(allok))