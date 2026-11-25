import io,json
cfg=json.load(io.open('docs/laiyipao/audits/round33-wiki-enemy-mechanics/data/config.json',encoding='utf-8'))
vals=set()
for e in cfg['enemies']:
    for k,v in e.get('resist',{}).items(): vals.add(v)
print('RESIST_DISTINCT_VALUES=', sorted(vals))
print('NON_MULTIPLE_OF_10=', sorted(v for v in vals if v%10!=0))
