import io,json
base='docs/laiyipao/audits/round33-wiki-enemy-mechanics/data/'
cfg=json.load(io.open(base+'config.json',encoding='utf-8'))
es=cfg['enemies']
print('ENEMY_COUNT',len(es))
keys=['id','name','category','speed','armor','shield_hp','attack','attack_range','attack_interval','fly_height','burrow','is_boss']
for i in [1,10,13,17,22]:
    e=next(x for x in es if x['id']==i)
    print('KEY'+str(i), json.dumps({k:e.get(k) for k in keys},ensure_ascii=False))
print('BOSSES', sum(1 for e in es if e.get('is_boss')))
print('FLY', sum(1 for e in es if e.get('fly_height',0)>0))
print('BURROW', sum(1 for e in es if e.get('burrow')))
print('ARMOR', sum(1 for e in es if e.get('armor',0)>0))
