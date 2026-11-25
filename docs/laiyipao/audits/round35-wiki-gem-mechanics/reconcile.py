import json,io,sys
base='docs/laiyipao/audits/round35-wiki-gem-mechanics/data/'
cfg=json.load(io.open(base+'config.json',encoding='utf-8'))
LBL={'attack':'攻击','element_coef':'元素系数','crit':'暴击率','reaction_mult':'反应倍率','element_cap':'元素层数上限','heat_cap':'热量上限','armor':'护甲','skill_damage':'技能伤害'}
def attr(v):
    return LBL.get(v,v) if isinstance(v,str) and v else '—'
def gemrow(g):
    return [str(g.get('id','')),g.get('name',''),attr(g.get('attr')),g.get('descr','')]
gexpect={str(g['id']):gemrow(g) for g in cfg.get('gems',[])}
def qrow(q):
    n=q.get('affix_count'); m=q.get('mult')
    c=(str(n)+' 条') if isinstance(n,int) and not isinstance(n,bool) and n>0 else '—'
    d=(str(m)+'‰') if isinstance(m,int) and not isinstance(m,bool) and m>0 else '—'
    return [q.get('name',''),c,d]
qexpect={str(q['name']):qrow(q) for q in cfg.get('gem_qualities',[])}
gems=cfg.get('gems',[]); quals=cfg.get('gem_qualities',[])
counts=[q.get('affix_count') for q in quals if isinstance(q.get('affix_count'),int) and not isinstance(q.get('affix_count'),bool)]
mn=min(counts) if counts else 0; mx=max(counts) if counts else 0
final='共 %d 种宝石，%d 档品质，词条 %d–%d 条'%(len(gems),len(quals),mn,mx)
print('EXPECT_AI='+final)
def load(vp):
    raw=io.open(base+'probe-'+vp+'.json',encoding='utf-8').read().strip()
    dec=json.JSONDecoder();obj,end=dec.raw_decode(raw)
    while isinstance(obj,str):obj,end=dec.raw_decode(obj)
    return obj
allok=True
for vp in sys.argv[1:]:
    p=load(vp);mis=[]
    g=p.get('gem') or {}; q=p.get('quality') or {}
    for row in g.get('rows',[]):
        rid=row[0].strip() if row else ''
        ex=gexpect.get(rid)
        if not ex: mis.append('GEM '+rid+' NOT_IN_CFG');continue
        for i,(want,got) in enumerate(zip(ex,row)):
            if want!=got: mis.append('GEM '+rid+' C%d EXP='%i+repr(want)+' GOT='+repr(got))
    for row in q.get('rows',[]):
        rid=row[0].strip() if row else ''
        ex=qexpect.get(rid)
        if not ex: mis.append('QUAL '+rid+' NOT_IN_CFG');continue
        for i,(want,got) in enumerate(zip(ex,row)):
            if want!=got: mis.append('QUAL '+rid+' C%d EXP='%i+repr(want)+' GOT='+repr(got))
    aiOk=final in p.get('conclusion','')
    noteOk=('词条随机，具体数值以游戏内为准' in p.get('aiNote','')) and ('词条随机，具体数值以游戏内为准' in p.get('basis',''))
    w1=p.get('wrap1') or {}; w2=p.get('wrap2') or {}
    s1=(w1.get('overflow')=='auto' and w1.get('sw',0)>=w1.get('cw',0) and (w1.get('sw',0)==w1.get('cw',0) or w1.get('after',0)>0))
    s2=(w2.get('overflow')=='auto' and w2.get('sw',0)>=w2.get('cw',0) and (w2.get('sw',0)==w2.get('cw',0) or w2.get('after',0)>0))
    docOk=p.get('docW',0)<=p.get('innerW',0)+1
    ctaOk=p.get('ctaText')=='查看宝石图鉴 →' and p.get('ctaVisible') and p.get('ctaHref')=='#/wiki?tab=gems'
    ok=(not mis) and aiOk and noteOk and s1 and s2 and docOk and ctaOk and not p.get('badText') and p.get('sectionVisible') and len(g.get('rows',[]))==len(gems) and len(q.get('rows',[]))==len(quals)
    allok=allok and ok
    print('VP='+vp+' GEMS='+str(len(g.get('rows',[])))+' QUALS='+str(len(q.get('rows',[])))+' MISMATCHES='+str(len(mis))+' AI_OK='+str(aiOk)+' NOTE_OK='+str(noteOk)+' BAD='+str(p.get('badText'))+' DOC='+str(p.get('docW'))+'/'+str(p.get('innerW'))+' W1='+str(w1.get('before'))+'->'+str(w1.get('after'))+'/'+str(w1.get('sw'))+'/'+str(w1.get('cw'))+' W2='+str(w2.get('before'))+'->'+str(w2.get('after'))+'/'+str(w2.get('sw'))+'/'+str(w2.get('cw'))+' CTA='+repr(p.get('ctaText'))+' VERDICT='+('PASS' if ok else 'FAIL'))
    for m in mis[:15]:print('  MIS '+m)
print('ALL_RECONCILE_OK='+str(allok))
