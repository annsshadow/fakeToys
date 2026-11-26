import json,io,sys
base='docs/laiyipao/audits/round32-postdeploy/data/'
cfg=json.load(io.open(base+'config.json',encoding='utf-8'))
skills=cfg['skills']
def fmt(ms):
    if not isinstance(ms,(int,float)) or isinstance(ms,bool) or ms<=0: return '—'
    s=ms/1000.0
    if float(s).is_integer(): return str(int(s))+'秒'
    return str(round(s,1))+'秒'
def pos(v,pre):
    return pre+' '+str(v) if isinstance(v,int) and not isinstance(v,bool) and v>0 else '—'
def kind(v): return {'active':'主动','passive':'被动'}.get(v,'未知类型')
expect={}
for s in skills:
    expect[str(s['id'])]={'name':s.get('name',''),'冷却':fmt(s.get('cooldown_ms')),'类型':kind(s.get('kind')),'穿透':pos(s.get('pierce'),'穿透'),'范围':pos(s.get('aoe_radius'),'范围'),'连锁':pos(s.get('chain'),'连锁')}
act=sum(1 for s in skills if s.get('kind')=='active'); pas=sum(1 for s in skills if s.get('kind')=='passive')
multi=sum(1 for s in skills if any(isinstance(s.get(k),int) and not isinstance(s.get(k),bool) and s[k]>0 for k in ('pierce','aoe_radius','chain')))
ai='共 %d 个技能：主动 %d 个，被动 %d 个，多目标 %d 个'%(len(skills),act,pas,multi)
print('EXPECT_AI='+ai)
def load_probe(vp):
    raw=io.open(base+'probe-'+vp+'.json',encoding='utf-8').read().strip()
    dec=json.JSONDecoder(); obj,end=dec.raw_decode(raw)
    while isinstance(obj,str): obj,end=dec.raw_decode(obj)
    return obj
allok=True
for vp in sys.argv[1:]:
    p=load_probe(vp)
    mis=[]; seen=set()
    for row in p.get('rows',[]):
        if not row: continue
        rid=row[0].strip(); seen.add(rid)
        e=expect.get(rid)
        if not e: mis.append(rid+' NOT_IN_CONFIG'); continue
        actual={'name':row[1] if len(row)>1 else '','冷却':row[6] if len(row)>6 else '','类型':row[7] if len(row)>7 else '','穿透':row[8] if len(row)>8 else '','范围':row[9] if len(row)>9 else '','连锁':row[10] if len(row)>10 else ''}
        for k,v in e.items():
            if actual[k]!=v: mis.append(rid+' '+k+' EXP='+repr(v)+' GOT='+repr(actual[k]))
    for rid in expect:
        if rid not in seen: mis.append(rid+' DOM_MISSING')
    aiOk=ai in p.get('conclusion','') or ai in p.get('aiText','')
    ok=(not mis) and aiOk and p.get('rowCount')==len(skills) and not p.get('hasBad') and p.get('docW',0)<=p.get('innerW',0)+1 and p.get('wrapSw',0)>=p.get('wrapCw',0) and p.get('ctaText')=='查看合成技能 →' and p.get('ctaHref')=='#/wiki?tab=skills&anchor=composite'
    allok=allok and ok
    print('VP='+vp+' ROWS='+str(p.get('rowCount'))+' MISMATCHES='+str(len(mis))+' AI_OK='+str(aiOk)+' BAD='+str(p.get('hasBad'))+' DOC='+str(p.get('docW'))+'/'+str(p.get('innerW'))+' WRAP='+str(p.get('wrapSw'))+'/'+str(p.get('wrapCw'))+' CTA='+repr(p.get('ctaText'))+' HREF='+repr(p.get('ctaHref'))+' VERDICT='+('PASS' if ok else 'FAIL'))
    for m in mis[:12]: print('  MIS '+m)
print('ALL_RECONCILE_OK='+str(allok))