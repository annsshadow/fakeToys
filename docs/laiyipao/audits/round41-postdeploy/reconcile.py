# -*- coding: utf-8 -*-
"""R41 postdeploy: reconcile public config chapters against DOM probes and regression counts."""
import json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
D = "docs/laiyipao/audits/round41-postdeploy"
FIELDS = ["id", "name", "start_level", "end_level", "terrain_kind", "boss_enemy_id"]
TERRAIN = {"oil_drum": "油桶", "collapse_wall": "塌墙", "charge_tower": "蓄能塔", "tidal_gate": "潮汐闸门", "rotor_vane": "旋翼风扇"}
def load(name):
    with open(D + "/data/" + name, encoding="utf-8") as f:
        v = json.load(f)
    while isinstance(v, str): v = json.loads(v)
    return v
def fmt_range(a, b): return str(a) + "–" + str(b) + "关" if isinstance(a,int) and isinstance(b,int) and a>0 and b>0 else "—"
def fmt_terrain(k): return TERRAIN.get(k, k) if isinstance(k,str) and k else "—"
def fmt_boss(v): return "#"+str(v) if isinstance(v,int) and not isinstance(v,bool) and v>0 else "—"
cfg = load("config.json"); chapters = cfg.get("chapters") or []
expected = [[str(ch.get("id")), str(ch.get("name")), fmt_range(ch.get("start_level"),ch.get("end_level")), fmt_terrain(ch.get("terrain_kind")), fmt_boss(ch.get("boss_enemy_id"))] for ch in chapters]
print("CHAPTER_COUNT=" + str(len(expected)))
print("EXPECTED_ROWS=" + json.dumps(expected, ensure_ascii=False))
problems=[]
for vp in ["390","768","1280"]:
    p=load("probe-"+vp+".json")
    mism=0
    if p.get("hash","").find("tab=levels")<0: problems.append(vp+":hash="+str(p.get("hash")))
    if not p.get("sectionVisible"): problems.append(vp+":sectionInvisible")
    if p.get("headers")!=["ID","名称","关卡区间","地形（配置基值）","首领敌人","说明"]: problems.append(vp+":headers="+json.dumps(p.get("headers"),ensure_ascii=False))
    rows=p.get("rows") or []
    if len(rows)!=len(expected): problems.append(vp+":rowCount="+str(len(rows)))
    for i,exp in enumerate(expected):
        got=rows[i] if i<len(rows) else []
        for j in range(5):
            g=got[j] if j<len(got) else None
            if g!=exp[j]:
                mism+=1; problems.append(vp+":cell"+str(i)+"."+str(j)+" got="+repr(g)+" want="+exp[j])
    ai=p.get("ai") or {}
    if "共 6 章" not in ai.get("conclusion",""): problems.append(vp+":ai="+ai.get("conclusion","")[:80])
    if "/api/v1/config" not in ai.get("basis","") or "replay_hash" not in ai.get("honesty",""): problems.append(vp+":aiBasisHonesty")
    if ai.get("ctaHref")!="#/wiki?tab=levels": problems.append(vp+":ctaHref="+str(ai.get("ctaHref")))
    if p.get("dirty"): problems.append(vp+":dirtyText")
    if p.get("page",{}).get("docW")!=p.get("page",{}).get("innerW"): problems.append(vp+":pageOverflow")
    print("VIEWPORT_"+vp+" rows="+str(len(rows))+" mismatches="+str(mism)+" ok="+str(mism==0).lower())
def jload(name):
    try: return load(name)
    except Exception as e: problems.append("reg:"+name+":"+str(e)); return {}
exp_reg={"skills":24,"recipe":18,"enemies":22,"levels":20,"gems":8,"skins":6,"rating":5,"drop":6,"chapter":6,"masteryRows":96}
reg={}
for name,key in [("basics","reaction"),("skills","skills"),("composite","compositeRows"),("enemies","enemies"),("levels","levels"),("gems","gems"),("skins","skins"),("rules","drop"),("levels","chapter"),("mastery","masteryRows")]:
    t=jload("reg-"+name+".json"); reg[key]=t.get(key)
    if t.get("dirty"): problems.append("reg:"+name+":dirtyText")
    if t.get("docW")!=t.get("innerW"): problems.append("reg:"+name+":pageOverflow")
for key,val in exp_reg.items():
    if reg.get(key)!=val: problems.append("reg:"+key+"="+str(reg.get(key)))
print("REGRESSION="+json.dumps(reg,ensure_ascii=False))
print("PROBLEMS="+json.dumps(problems,ensure_ascii=False))
print("ALL_RECONCILE_OK="+str(len(problems)==0).lower())