import json,io,sys
base='docs/laiyipao/audits/round34-wiki-level-mechanics/data/'
cfg=json.load(io.open(base+'config.json',encoding='utf-8'))
levels=cfg['levels']
LBL={'oil_drum':'油桶','collapse_wall':'塌墙','charge_tower':'蓄能塔'}
def terrain(t):
    if not isinstance(t,list): return '—'
    order=[];cnt={}
    for it in t:
        if not isinstance(it,dict): continue
        k=it.get('kind')
        if not isinstance(k,str) or not k: continue
        if k not in cnt: cnt[k]=0;order.append(k)
        cnt[k]+=1
    if not order: return '—'
    return '、'.join((LBL.get(k,k)+'×'+str(cnt[k])) for k in order)
def pos(v):
    return str(v) if isinstance(v,int) and not isinstance(v,bool) and v>0 else '—'
def permille(v):
    return str(v)+'‰' if isinstance(v,int) and not isinstance(v,bool) and v>0 else '—'
def boolean(v): return '是' if v is True else '—'
def expected(l):
    return [str(l.get('id','')),l.get('name',''),str(l.get('difficulty','')),str(l.get('base_hp','')),str(l.get('wave_count','')),str(l.get('energy_cost','')),str(l.get('element_cap','')),str(l.get('max_reaction_tier','')),' / '.join(str(s) for s in l.get('star_targets',[])),boolean(l.get('is_boss')),terrain(l.get('terrain')),permille(l.get('armor_permille')),pos(l.get('max_score'))]
expect={str(l['id']):expected(l) for l in levels}
boss=sum(1 for l in levels if l.get('is_boss') is True)
wt=sum(1 for l in levels if terrain(l.get('terrain'))!='—')
kinds=set()
for l in levels:
    for t in (l.get('terrain') or []) if isinstance(l.get('terrain'),list) else []:
        if isinstance(t,dict) and isinstance(t.get('kind'),str) and t.get('kind'): kinds.add(t['kind'])
final='共 %d 关：首领关 %d 个，含地形关 %d 个，地形种类 %d 种'%(len(levels),boss,wt,len(kinds))
print('EXPECT_AI='+final)
def load(vp):
    raw=io.open(base+'probe-'+vp+'.json',encoding='utf-8').read().strip()
    dec=json.JSONDecoder();obj,end=dec.raw_decode(raw)
    while isinstance(obj,str):obj,end=dec.raw_decode(obj)
    return obj
allok=True; seenall=set()
for vp in sys.argv[1:]:
    p=load(vp);mis=[];seen=set()
    for row in p.get('rows',[]):
        rid=row[0].strip() if row else '';seen.add(rid);seenall.add(rid)
        ex=expect.get(rid)
        if not ex: mis.append(rid+' NOT_IN_CONFIG');continue
        for i,(want,got) in enumerate(zip(ex,row)):
            if want!=got: mis.append(rid+' COL%d EXP='%i+repr(want)+' GOT='+repr(got))
    aiOk=final in p.get('conclusion','') or final in p.get('aiText','')
    ok=(not mis) and aiOk and p.get('rowCount')==len(p.get('rows',[])) and not p.get('hasBad') and p.get('docW',0)<=p.get('innerW',0)+1 and p.get('wrapSw',0)>=p.get('wrapCw',0) and p.get('hasConfigBase') and p.get('ctaText')=='查看关卡一览 →' and p.get('ctaVisible')
    allok=allok and ok
    print('VP='+vp+' ROWS='+str(p.get('rowCount'))+' MISMATCHES='+str(len(mis))+' AI_OK='+str(aiOk)+' BAD='+str(p.get('hasBad'))+' DOC='+str(p.get('docW'))+'/'+str(p.get('innerW'))+' WRAP='+str(p.get('wrapSw'))+'/'+str(p.get('wrapCw'))+' CFGBASE='+str(p.get('hasConfigBase'))+' CTA='+repr(p.get('ctaText'))+' VERDICT='+('PASS' if ok else 'FAIL'))
    for m in mis[:15]:print('  MIS '+m)
print('COVERED_IDS='+str(len(seenall))+'/'+str(len(levels)))
print('ALL_RECONCILE_OK='+str(allok))
