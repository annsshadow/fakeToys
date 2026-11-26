# -*- coding: utf-8 -*-
"""R40: reconcile public /api/v1/config rating_weights against DOM probes per viewport,
then verify R31-R39 regression row counts. Prints ALL_RECONCILE_OK at the end."""
import json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
D = "docs/laiyipao/audits/round40-postdeploy"
KEYS = ["element_coverage", "reaction_coverage", "mastery_done", "equipment_synergy", "mechanic_depth"]
LABELS = {"element_coverage": "元素覆盖", "reaction_coverage": "反应链", "mastery_done": "专精投入",
          "equipment_synergy": "装备契合", "mechanic_depth": "机制深度"}
def load(name):
    with open(D + "/data/" + name, encoding="utf-8") as f:
        v = json.load(f)
    while isinstance(v, str):
        v = json.loads(v)
    return v
def label(k):
    if not isinstance(k, str) or not k.strip():
        return "—"
    return LABELS.get(k, "（未知维度#" + k + "）")
def value(v):
    if isinstance(v, bool) or not isinstance(v, int) or v < 0:
        return "—"
    return str(v)
cfg = load("config.json")
rw = cfg.get("rating_weights") or {}
expected = [[label(k), value(rw.get(k))] for k in KEYS]
present = sum(1 for k in KEYS if value(rw.get(k)) != "—")
ai_concl = "权重 " + str(present) + "/5 项齐备"
print("EXPECTED_ROWS=" + json.dumps(expected, ensure_ascii=False))
print("EXPECTED_AI=" + ai_concl)
problems = []
for vp in ["390", "768", "1280"]:
    p = load("probe-" + vp + ".json")
    mism = 0
    if p.get("hash", "").find("tab=rating") < 0:
        problems.append(vp + ":hash=" + str(p.get("hash")))
    if not p.get("sectionVisible"):
        problems.append(vp + ":sectionInvisible")
    if p.get("tocRating", "").find("构筑评分") < 0:
        problems.append(vp + ":tocMissing")
    if p.get("headers") != ["维度", "权重"]:
        problems.append(vp + ":headers=" + json.dumps(p.get("headers"), ensure_ascii=False))
    rows = p.get("rows") or []
    if len(rows) != 5:
        problems.append(vp + ":rowCount=" + str(len(rows)))
    for i, exp in enumerate(expected):
        got = rows[i] if i < len(rows) else []
        for j in range(2):
            g = got[j] if j < len(got) else None
            if g != exp[j]:
                mism += 1
                problems.append(vp + ":cell" + str(i) + "." + str(j) + " got=" + repr(g) + " want=" + exp[j])
    c = p.get("conclusion", "")
    if c.find(ai_concl) < 0:
        problems.append(vp + ":conclusion=" + c[:80])
    if c.find("权重") < 0 or c.find("项齐备") < 0:
        problems.append(vp + ":conclusionShape")
    if p.get("basis", "").find("公开 /api/v1/config") < 0:
        problems.append(vp + ":basis=" + str(p.get("basis"))[:80])
    if p.get("honesty", "").find("replay_hash") < 0:
        problems.append(vp + ":honesty=" + str(p.get("honesty"))[:80])
    if p.get("ctaText", "").find("查看构筑评分") < 0:
        problems.append(vp + ":ctaText=" + str(p.get("ctaText"))[:60])
    if p.get("ctaHref") != "#/wiki?tab=rating":
        problems.append(vp + ":ctaHref=" + str(p.get("ctaHref")))
    if not p.get("ctaVisible"):
        problems.append(vp + ":ctaInvisible")
    if p.get("secHasPct"):
        problems.append(vp + ":hasPercent")
    if p.get("bad"):
        problems.append(vp + ":dirtyText")
    if p.get("docW") != p.get("innerW"):
        problems.append(vp + ":pageOverflow docW=" + str(p.get("docW")) + " innerW=" + str(p.get("innerW")))
    sc = p.get("scroll") or {}
    sw, cw, after = sc.get("sw", 0), sc.get("cw", 0), sc.get("after", 0)
    if sw > cw and after <= 0:
        problems.append(vp + ":cannotScroll sw=" + str(sw) + " cw=" + str(cw))
    print("VIEWPORT_" + vp + " rows=" + str(len(rows)) + " mismatches=" + str(mism) +
          " scroll=" + json.dumps(sc) + " ok=" + str(mism == 0).lower())
# regression R31-R39
def tab(name):
    try:
        return load("reg-" + name + ".json")
    except Exception as e:
        problems.append("reg:" + name + ":" + str(e))
        return {}
def table_rows(t):
    out = []
    for s in t.get("sections", []):
        for tb in s.get("tables", []):
            out.append(tb.get("rows", 0))
    return out
b = tab("basics")
if b.get("reactionRows") != 7: problems.append("reg:reactionRows=" + str(b.get("reactionRows")))
if not b.get("corrosionSpread"): problems.append("reg:corrosionSpread")
if not b.get("armorShred"): problems.append("reg:armorShred")
if not b.get("armorKnock"): problems.append("reg:armorKnock")
if not any("r40-rating-mechanics" in s.get("cls", "") for s in b.get("sections", [])):
    problems.append("reg:basicsMissingRatingSection")
sk = tab("skills")
if 24 not in table_rows(sk): problems.append("reg:skillRows=" + json.dumps(table_rows(sk)))
cp = tab("composite")
if not cp.get("compositeVisible") or cp.get("compositeRows") != 18: problems.append("reg:composite=" + str(cp.get("compositeRows")))
if not cp.get("recipeVisible") or cp.get("recipeRows") != 18: problems.append("reg:recipe=" + str(cp.get("recipeRows")))
en = tab("enemies")
if 22 not in table_rows(en): problems.append("reg:enemyRows=" + json.dumps(table_rows(en)))
lv = tab("levels")
if 20 not in table_rows(lv): problems.append("reg:levelRows=" + json.dumps(table_rows(lv)))
gm = tab("gems")
if 8 not in table_rows(gm): problems.append("reg:gemRows=" + json.dumps(table_rows(gm)))
ma = tab("mastery")
mrows = sum(table_rows(ma))
if mrows != 96: problems.append("reg:masteryRows=" + str(mrows))
sn = tab("skins")
if sn.get("skinRows") != 6: problems.append("reg:skinRows=" + str(sn.get("skinRows")))
ru = tab("rules")
if not ru.get("dropVisible") or ru.get("dropRows") != 6: problems.append("reg:dropRows=" + str(ru.get("dropRows")))
for name in ["basics", "skills", "composite", "enemies", "levels", "gems", "mastery", "skins", "rules"]:
    t = tab(name)
    if t.get("bad"): problems.append("reg:" + name + ":dirtyText")
    if t.get("docW") != t.get("innerW"): problems.append("reg:" + name + ":pageOverflow")
print("REGRESSION reaction=7 skills=24 composite=18 recipe=18 enemies=22 levels=20 gems=8 mastery=96 skins=6 drop=6")
if problems:
    print("PROBLEMS=" + json.dumps(problems, ensure_ascii=False, indent=1))
    print("ALL_RECONCILE_OK=False")
else:
    print("ALL_RECONCILE_OK=True")