import json,io,sys
base='docs/laiyipao/audits/round33-wiki-enemy-mechanics/data/'
cfg=json.load(io.open(base+'config.json',encoding='utf-8'))
enemies=cfg['enemies']
def speed(v):
    if not isinstance(v,(int,float)) or isinstance(v,bool) or v<=0: return '—'
    n=round(v/1000.0,1)
    return (str(int(n)) if float(n).is_integer() else str(n))+'像素/秒'
def ms(v):
    if not isinstance(v,(int,float)) or isinstance(v,bool) or v<=0: return '—'
    s=v/1000.0
    return str(int(s))+'秒' if float(s).is_integer() else str(round(s,1))+'秒'
def pos(v):
    return str(v) if isinstance(v,int) and not isinstance(v,bool) and v>0 else '—'
def armor(v):
    return str(v)+'‰' if isinstance(v,int) and not isinstance(v,bool) and v>0 else '—'
def boolean(v): return '是' if v is True else '—'
def resistText(r):
    order=[('焰','fire'),('冰','ice'),('电','lightning'),('毒','corrosion'),('动能','kinetic')]
    parts=[]
    for label,key in order:
        v=(r or {}).get(key,0)
        pct=v/10.0
        s=str(int(pct)) if float(pct).is_integer() else str(pct)
        parts+=[label, s+'%']
    return '\n'.join(parts)
CAT={'normal':'普通','ranged':'远程','flying':'飞行','special':'特殊','elite':'精英','boss':'BOSS'}
def expected(e):
    cells=[str(e.get('id','')),e.get('name',''),CAT.get(e.get('category'),str(e.get('category',''))),str(e.get('hp',''))]
    cells.append(resistText(e.get('resist')))
    cells+=[speed(e.get('speed')),armor(e.get('armor')),pos(e.get('shield_hp')),pos(e.get('attack')),pos(e.get('attack_range')),ms(e.get('attack_interval')),pos(e.get('fly_height')),boolean(e.get('burrow')),boolean(e.get('is_boss')),e.get('descr','')]
    return cells
expect={str(e['id']):expected(e) for e in enemies}
boss=sum(1 for e in enemies if e.get('is_boss') is True)
fly=sum(1 for e in enemies if e.get('category')=='flying')
burrow=sum(1 for e in enemies if e.get('burrow') is True)
armored=sum(1 for e in enemies if isinstance(e.get('armor'),int) and not isinstance(e.get('armor'),bool) and e.get('armor')>0)
final='共 %d 种敌人：首领 %d 种，飞行 %d 种，潜地 %d 种，带护甲 %d 种'%(len(enemies),boss,fly,burrow,armored)
print('EXPECT_AI='+final)
def load(vp):
    raw=io.open(base+'probe-'+vp+'.json',encoding='utf-8').read().strip()
    dec=json.JSONDecoder();obj,end=dec.raw_decode(raw)
    while isinstance(obj,str):obj,end=dec.raw_decode(obj)
    return obj
allok=True
for vp in sys.argv[1:]:
    p=load(vp);mis=[];seen=set()
    for row in p.get('rows',[]):
        rid=row[0].strip() if row else '';seen.add(rid)
        ex=expect.get(rid)
        if not ex: mis.append(rid+' NOT_IN_CONFIG');continue
        for i,(want,got) in enumerate(zip(ex,row)):
            if want!=got: mis.append(rid+' COL%d EXP='%i+repr(want)+' GOT='+repr(got))
    for rid in expect:
        if rid not in seen:mis.append(rid+' DOM_MISSING')
    aiOk=final in p.get('conclusion','') or final in p.get('aiText','')
    ok=(not mis) and aiOk and p.get('rowCount')==len(enemies) and not p.get('hasBad') and p.get('docW',0)<=p.get('innerW',0)+1 and p.get('wrapSw',0)>=p.get('wrapCw',0) and p.get('ctaText')=='查看敌人列表 →' and p.get('ctaHref')=='#/wiki?tab=enemies'
    allok=allok and ok
    print('VP='+vp+' ROWS='+str(p.get('rowCount'))+' MISMATCHES='+str(len(mis))+' AI_OK='+str(aiOk)+' BAD='+str(p.get('hasBad'))+' DOC='+str(p.get('docW'))+'/'+str(p.get('innerW'))+' WRAP='+str(p.get('wrapSw'))+'/'+str(p.get('wrapCw'))+' SCROLL='+str(p.get('wrapBefore'))+'->'+str(p.get('wrapAfter'))+' CTA='+repr(p.get('ctaText'))+' HREF='+repr(p.get('ctaHref'))+' VERDICT='+('PASS' if ok else 'FAIL'))
    for m in mis[:15]:print('  MIS '+m)
print('ALL_RECONCILE_OK='+str(allok))
