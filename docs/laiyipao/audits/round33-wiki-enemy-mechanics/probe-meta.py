import io,json
raw=io.open('docs/laiyipao/audits/round33-wiki-enemy-mechanics/data/probe-390.json',encoding='utf-8').read()
o=json.loads(raw)
if isinstance(o,str): o=json.loads(o)
for k,v in o.items():
    if k!='rows': print(k, '=', json.dumps(v,ensure_ascii=False))
