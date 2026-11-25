import json,glob,hashlib,sys
sys.stdout.reconfigure(encoding="utf-8")
D="docs/laiyipao/audits/round40-wiki-rating-weights"
def load(p):
  v=json.load(open(p,encoding="utf-8"))
  return json.loads(v) if isinstance(v,str) else v
for w in (390,768,1280):
  for tag in ("fanyinglian","mechanic_depth"):
    before=load(f"{D}/data/t29b-search-{w}-{tag}.json")
    after=load(f"{D}/data/t29b-after-{w}-{tag}.json")
    click=open(f"{D}/data/t29b-click-{w}-{tag}.txt",encoding="utf-8").read().strip()
    path=f"{D}/screenshots/t29b-{w}-{tag}.png"
    sha=hashlib.sha256(open(path,"rb").read()).hexdigest()[:12]
    print(f"VIEW={w} Q={tag} HITS={[(h['kind'],h['title']) for h in before['hits']]} EMPTY={before['empty']!r} CLICK={click!r} AFTER_FOCUS={after['ratingRowFocus']} FOCUS_TEXT={after['focusText']!r} HASH_AFTER={after['hash']} SHOT={sha}")
hashes=[hashlib.sha256(open(p,"rb").read()).hexdigest() for p in glob.glob(f"{D}/screenshots/t29b-*.png")]
print("DISTINCT_T29B_SHOTS=%d" % len(set(hashes)))
