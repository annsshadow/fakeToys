import json, sys
d=json.load(open(sys.argv[1], encoding="utf-8"))
if isinstance(d, str): d = json.loads(d)
for k in sys.argv[2].split("."): d = d[k]
print(int(round(d)) if isinstance(d, float) else d)