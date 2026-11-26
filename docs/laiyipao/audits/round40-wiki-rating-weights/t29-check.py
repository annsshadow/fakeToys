import json,glob,os,hashlib
D="docs/laiyipao/audits/round40-wiki-rating-weights"
for w in (390,768,1280):
  for tag in ("fanyinglian","mechanic_depth"):
    before=json.load(open(f"{D}/data/t29-search-{w}-{tag}.json",encoding="utf-8"))
    after=json.load(open(f"{D}/data/t29-after-{w}-{tag}.json",encoding="utf-8"))
    click=open(f"{D}/data/t29-click-{w}-{tag}.txt",encoding="utf-8").read().strip()
    path=f"{D}/screenshots/t29-{w}-{tag}.png"
    sha=hashlib.sha256(open(path,"rb").read()).hexdigest()[:12]
    print(f"VIEW={w} Q={tag} HITS={[(h['kind'],h['title']) for h in before['hits']]} EMPTY={before['empty']!r} CLICK={click!r} AFTER_FOCUS={after['ratingRowFocus']!r} FOCUS_TEXT={after['focusText']!r} HASH_AFTER={after['hash']} SHOT={sha}")
hashes={hashlib.sha256(open(p,"rb").read()).hexdigest() for p in glob.glob(f"{D}/screenshots/t29-*.png")}
print("DISTINCT_T29_SHOTS=%d"%len(hashes))
