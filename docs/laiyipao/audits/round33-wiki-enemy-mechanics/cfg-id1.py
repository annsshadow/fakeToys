import io,json
cfg=json.load(io.open('docs/laiyipao/audits/round33-wiki-enemy-mechanics/data/config.json',encoding='utf-8'))
e=next(x for x in cfg['enemies'] if x['id']==1)
print('ALL KEYS:', list(e.keys()))
print('ID1 FULL:', json.dumps(e,ensure_ascii=False))
