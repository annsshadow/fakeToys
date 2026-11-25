# -*- coding: utf-8 -*-
"""R42 postdeploy reconcile: config chapters/levels vs DOM rows, clicks, CTA, search, regressions."""
import json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
D = "docs/laiyipao/audits/round42-postdeploy"
DATA = D + "/data"
VPS = ["390", "768", "1280"]
problems = []
def load(n):
    with open(DATA + "/" + n, encoding="utf-8") as f: v = json.load(f)
    while isinstance(v, str): v = json.loads(v)
    return v
cfg = load("config.json")
chapters = cfg.get("chapters") or []
levels = cfg.get("levels") or []
name_by_id = {c["id"]: c["name"] for c in chapters}
level_chapter_name = {l["id"]: name_by_id.get(l.get("chapter")) for l in levels}
AI_EXPECT = "AI 章节交叉概览：共100关，覆盖6章"
print("CONFIG_VERSION=", cfg.get("version"))

# 1) 查看每行的「关号 + 所属章节」自校验（不依赖章节下拉导航的正确性）
observed = {}
for vp in VPS:
    for idx in range(1, 7):
        fn = "pd-level-%s-ch%d.json" % (vp, idx)
        try: p = load(fn)
        except Exception as e: problems.append("%s missing: %s" % (fn, e)); continue
        if p.get("err") or p.get("error"):
            problems.append("%s probe err: %s" % (fn, p.get("err") or p.get("error")))
        if p.get("dirty"): problems.append("%s dirty text" % fn)
        if p.get("ai") != AI_EXPECT: problems.append("%s ai=%r" % (fn, p.get("ai")))
        rows = p.get("rows") or []
        for r in rows:
            if not r or len(r) < 14: continue
            try: lid = int(str(r[0]).strip())
            except Exception: continue
            chap_name = str(r[-1]).strip()
            if lid in observed and observed[lid] != chap_name:
                problems.append("vp%s level %d chapter mismatch across captures: %s vs %s" % (vp, lid, observed[lid], chap_name))
            observed[lid] = chap_name
print("OBSERVED_LEVEL_COUNT=", len(observed))
if len(observed) != len(levels):
    problems.append("coverage %d != %d levels" % (len(observed), len(levels)))
mism = [(lid, n, level_chapter_name.get(lid)) for lid, n in sorted(observed.items()) if n != level_chapter_name.get(lid)]
print("CHAPTER_CELL_MISMATCHES=", len(mism))
for m in mism[:20]: print("  MISMATCH=", m)
if mism: problems.append("chapter cells mismatch count=%d" % len(mism))

# 2) 真CLI列内交叉引用点击
for vp in VPS:
    try: a = load("pd-click-after-%s.json" % vp)
    except Exception as e: problems.append("click %s: %s" % (vp, e)); continue
    txt = open(DATA + "/pd-click-link-%s.txt" % vp, encoding="utf-8", errors="replace").read()
    if "Done" not in txt: problems.append("click-link %s not Done" % vp)
    if "chapter-mechanics" not in (a.get("hash") or "") or not a.get("chapterVisible") or "/wiki" not in (a.get("hash") or ""):
        problems.append("click %s hash=%s chapterVisible=%s" % (vp, a.get("hash"), a.get("chapterVisible")))

# 3) AI CTA 真CLI停留 levels&anchor=chapter-mechanics
for vp in VPS:
    try: a = load("pd-cta-after-%s.json" % vp); b = load("pd-cta-before-%s.json" % vp)
    except Exception as e: problems.append("cta %s: %s" % (vp, e)); continue
    txt = open(DATA + "/pd-cta-click-%s.txt" % vp, encoding="utf-8", errors="replace").read()
    if "Done" not in txt: problems.append("cta-click %s not Done" % vp)
    h = a.get("hash") or ""
    if "tab=levels" not in h or "anchor=chapter-mechanics" not in h or not a.get("onWiki"):
        problems.append("cta %s hash=%s onWiki=%s" % (vp, h, a.get("onWiki")))
    if b.get("ctaText") != "查看章节一览 →": problems.append("cta %s text=%r" % (vp, b.get("ctaText")))

# 4) chapter搜索 rotor_vane -> 章节5 母巢外围命中并高亮
for vp in VPS:
    try: hits = load("pd-hits-%s.json" % vp); pick = load("pd-pick-%s.json" % vp)
    except Exception as e: problems.append("search %s: %s" % (vp, e)); continue
    hs = hits.get("hits") or []
    if not hs or not any("母巢外围" in x or "rotor_vane" in x for x in hs):
        problems.append("search %s hits=%s" % (vp, hs))
    if hits.get("query") != "rotor_vane": problems.append("search %s query=%r" % (vp, hits.get("query")))
    if not pick.get("hasChapterFocus") or "chapter:5" not in (pick.get("hash") or ""):
        problems.append("search %s pick hash=%s focus=%s" % (vp, pick.get("hash"), pick.get("hasChapterFocus")))

# 5) 11 tab 回归：无脏数据、无横向溢出、导航项存在
TABS = ["basics","skills","composite","enemies","levels","gems","skins","mastery","rules","rating"]
for vp in VPS:
    for t in TABS:
        try: r = load("pd-reg-%s-%s.json" % (t, vp))
        except Exception as e: problems.append("reg %s/%s: %s" % (t, vp, e)); continue
        if r.get("dirty"): problems.append("reg %s/%s dirty" % (t, vp))
        if r.get("docW") != r.get("innerW"):
            problems.append("reg %s/%s overflow docW=%s innerW=%s" % (t, vp, r.get("docW"), r.get("innerW")))
    nav = load("pd-reg-rating-%s.json" % vp)
    navs = {n.get("href") for n in (nav.get("navs") or [])}
    if len(navs) < 8:
        problems.append("reg %s navs only %d: %s" % (vp, len(navs), sorted(navs)))

print("PROBLEM_COUNT=", len(problems))
for p in problems[:40]: print("PROBLEM=", p)
result = {"allOk": not problems, "problems": problems, "configVersion": cfg.get("version"),
          "observedLevelCount": len(observed), "chapterCellMismatches": len(mism), "viewports": VPS}
with open(DATA + "/reconcile-result.json", "w", encoding="utf-8") as f:
    json.dump(result, f, ensure_ascii=False, indent=2)
print("ALL_RECONCILE_OK=", str(not problems).lower())