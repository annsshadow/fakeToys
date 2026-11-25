import json,glob,hashlib
D="docs/laiyipao/audits/round40-wiki-rating-weights"
for w in (390,768,1280):
  for tag in ("fanyinglian","mechanic_depth"):
    before=json.load(open(f"{D}/data/t29b-search-{w}-{tag}.json",encoding="utf-8"))
    after=json.load(open(f"{D}/data/t29b-after-{w}-{tag}.json",encoding="utf-8"))
    click=open(f"{D}/data/t29b-click-{w}-{tag}.txt",encoding="utf-8").read().strip()
    path=f"{D}/screenshots/t29b-{w}-{tag}.png"
    sha=hashlib.sha256(open(path,"rb").read()).hexdigest()[:12]
    print("VIEW=%s Q=%s HITS=%s EMPTY=%r CLICK=%r AFTER_FOCUS=%s FOCUS_TEXT=%r HASH_AFTER=%s SHOT=%s" % (w,tag,[(h["kind"],h["title"]) for h in before["hits"]],before["empty"],click,after["ratingRowFocus"],after["focusText"],after["hash"],sha))
hashes=[hashlib.sha256(open(p,"rb").read()).hexdigest() for p in glob.glob(f"{D}/screenshots/t29b-*.png")]
print("DISTINCT_T29B_SHOTS=%d" % len(set(hashes)))
