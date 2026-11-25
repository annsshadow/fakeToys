import json,sys
raw=open(sys.argv[1],encoding='utf-8').read().strip()
d=json.loads(raw)
if isinstance(d,str): d=json.loads(d)
k=sys.argv[2]
v=d['ctaRect'][k] if k.startswith('c') else d[k]
print(int(v))
