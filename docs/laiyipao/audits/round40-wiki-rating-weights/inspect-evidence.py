import os
D="docs/laiyipao/audits/round40-wiki-rating-weights"
for p in ["t29-open-390.txt","t29-fill-390-fanyinglian.txt","t29-search-390-fanyinglian.json","t29-click-390-fanyinglian.txt","t29-after-390-fanyinglian.json"]:
  f=f"{D}/data/{p}"
  b=open(f,"rb").read()
  print(p, len(b), repr(b[:300]))
