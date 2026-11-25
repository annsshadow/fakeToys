# -*- coding: utf-8 -*-
# R39: reconcile public /api/v1/config against DOM probes captured per viewport.
import json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
D = "docs/laiyipao/audits/round39-wiki-skins-drop-rates"
def load(name):
    with open(D + "/data/" + name, encoding="utf-8") as f:
        v = json.load(f)
    while isinstance(v, str):
        v = json.loads(v)
    return v
cfg = load("config.json")
skins = cfg.get("skins") or []
dr = cfg.get("drop_rates") or {}
RAR = {"rare": "稀有", "epic": "史诗", "legendary": "传说"}
def rtext(v):
    if not isinstance(v, str) or not v.strip():
        return "—"
    return RAR.get(v, v)
total = len(skins)
rare = sum(1 for s in skins if rtext(s.get("rarity")) == "稀有")
epic = sum(1 for s in skins if rtext(s.get("rarity")) == "史诗")
leg = sum(1 for s in skins if rtext(s.get("rarity")) == "传说")
keys = ["coin_base", "coin_per_star", "coin_per_kill", "reaction_bonus", "keys_on_three_star", "energy_on_win"]
present = sum(1 for k in keys if isinstance(dr.get(k), (int, float)) and not isinstance(dr.get(k), bool))
ai_expected = "共 %d 款皮肤：稀有 %d、史诗 %d、传说 %d；掉落 %d/6 项齐备" % (total, rare, epic, leg, present)
print("CONFIG skins=%d ai_expected=%s" % (total, ai_expected))
print("DROP_RATES=" + json.dumps(dr, ensure_ascii=False, sort_keys=True))
overall = True
for vp in ["390", "768", "1280"]:
    p = load("probe-" + vp + ".json")
    rows = p.get("rows") or []
    mism = []
    if len(rows) != len(skins):
        mism.append("ROWCOUNT dom=%d cfg=%d" % (len(rows), len(skins)))
    for i, s in enumerate(skins):
        if i >= len(rows):
            break
        r = rows[i]
        exp = [str(s.get("id")), s.get("name", ""), rtext(s.get("rarity")), s.get("passive", ""), s.get("descr", "")]
        for j in range(min(len(exp), len(r))):
            if str(exp[j]) != str(r[j]):
                mism.append("skin[%d]col%d cfg=%r dom=%r" % (i, j, exp[j], r[j]))
    ai_ok = ai_expected in (p.get("conclusion") or "").strip()
    basis_ok = ("公开 /api/v1/config" in (p.get("basis") or "")) and ("replay_hash" in (p.get("honesty") or ""))
    four_ok = ai_ok and basis_ok and bool(p.get("ctaVisible")) and ("查看皮肤图鉴" in (p.get("ctaText") or ""))
    scroll = p.get("scroll") or {}
    doc_ok = p.get("docW") == p.get("innerW")
    print("== %s == rows=%d mismatches=%d ai_ok=%s four_ok=%s bad=%s doc=%s/%s scroll sw=%s cw=%s before=%s after=%s" % (
        vp, len(rows), len(mism), ai_ok, four_ok, p.get("bad"), p.get("docW"), p.get("innerW"),
        scroll.get("sw"), scroll.get("cw"), scroll.get("before"), scroll.get("after")))
    for m in mism:
        print("   MISMATCH: " + m)
    if mism or not ai_ok or not four_ok or p.get("bad") or not doc_ok:
        overall = False
expect = [("基础金币", dr.get("coin_base")), ("每星金币", dr.get("coin_per_star")),
          ("每次击杀金币", dr.get("coin_per_kill")), ("反应加成", dr.get("reaction_bonus")),
          ("三星钥匙", dr.get("keys_on_three_star")), ("通关返还体力", dr.get("energy_on_win"))]
for vp in ["390", "768", "1280"]:
    try:
        d = load("drop-" + vp + ".json")
    except Exception as e:
        print("DROP %s READ_FAIL %s" % (vp, e)); overall = False; continue
    rows = d.get("rows") or []
    dm = []
    if len(rows) != 6:
        dm.append("ROWCOUNT=%d" % len(rows))
    for i, (lab, val) in enumerate(expect):
        if i < len(rows):
            row = rows[i]
            if row[0] != lab:
                dm.append("label[%d] exp=%r dom=%r" % (i, lab, row[0]))
            dv = row[1] if len(row) > 1 else ""
            if isinstance(val, (int, float)) and str(val) != str(dv).strip():
                dm.append("value[%d] %s cfg=%r dom=%r" % (i, lab, val, dv))
            if "%" in dv or "‰" in dv:
                dm.append("unit[%d] %s dom=%r" % (i, lab, dv))
    print("== DROP %s == mismatches=%d hasPct=%s" % (vp, len(dm), d.get("hasPct")))
    for m in dm:
        print("   DMISMATCH: " + m)
    if dm:
        overall = False
print("ALL_RECONCILE_OK=%s" % overall)
