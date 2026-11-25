D="docs/laiyipao/audits/round40-wiki-rating-weights"
for p in ["t29b-open-390.txt","t29b-fill-390-fanyinglian.txt","t29b-search-390-fanyinglian.json","t29b-click-390-fanyinglian.txt","t29b-after-390-fanyinglian.json"]:
  b=open(f"{D}/data/{p}","rb").read()
  print(p, len(b), repr(b[:500]))
