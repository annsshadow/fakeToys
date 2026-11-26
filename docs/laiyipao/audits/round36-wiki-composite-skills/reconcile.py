import json,io,sys,hashlib
base='docs/laiyipao/audits/round36-wiki-composite-skills/data/'
raw=io.open(base+'config.json','rb').read(); cfg=json.loads(raw.decode('utf-8'))
print('CFG_SHA='+hashlib.sha256(raw).hexdigest())
def fmt(v):
    return '—' if not (isinstance(v,int) and not isinstance(v,bool) and v>0) else str(v)
def cdt(ms):
    return '—' if not (isinstance(ms,int) and ms>0) else (('%g'%(ms/1000.0))+'秒')
def kd(k): return {'active':'主动','passive':'被动'}.get(k,'未知类型')
def pt(v): return '—' if not (isinstance(v,int) and v>0) else ('穿透 %d'%v)
def at(v): return '—' if not (isinstance(v,int) and v>0) else ('范围 %d'%v)
def ct(v): return '—' if not (isinstance(v,int) and v>0) else ('连锁 %d'%v)
def ut(v): return '—' if not (isinstance(v,int) and v>0) else ('%d 级解锁'%v)
ELEMENT_MAP={'fire':'焰','ice':'冰','lightning':'电','kinetic':'动能','corrosion':'毒'}
def row(r): return [str(r['id']),r['name'],ELEMENT_MAP.get(r.get('element',''),r.get('element','')),r['descr'],kd(r['kind']),cdt(r['cooldown_ms']),str(r['base_damage']),str(r['heat_cost']),pt(r['pierce']),at(r['aoe_radius']),ct(r['chain']),fmt(r['apply_stacks']),ut(r['unlock_level'])]
expect={str(r['id']):row(r) for r in cfg['composite_skills']}
active=sum(1 for r in cfg['composite_skills'] if kd(r['kind'])=='主动')
multi=sum(1 for r in cfg['composite_skills'] if any(isinstance(r.get(k),int) and r[k]>0 for k in ('pierce','aoe_radius','chain')))
final='共 %d 个合成技能：主动 %d 个，多目标 %d 个'%(len(cfg['composite_skills']),active,multi)
print('EXPECT_AI='+final)
basis='依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算'
honesty='辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准'
def load(vp):
    raw=io.open(base+'probe-'+vp+'.json',encoding='utf-8').read().strip(); dec=json.JSONDecoder(); obj,end=dec.raw_decode(raw)
    while isinstance(obj,str): obj,end=dec.raw_decode(obj)
    return obj
allok=True
HDR_EXPECT=['ID','名称','元素','说明','类型','冷却','伤害','热量','穿透','范围','连锁','层数','解锁等级']
for vp in sys.argv[1:]:
    p=load(vp); mis=[]
    rows=p.get('rows',[])
    if list(p.get('headers',[]))!=HDR_EXPECT: mis.append('HEADERS '+repr(p.get('headers')))
    for r in rows:
        rid=r[0].strip() if r else ''
        ex=expect.get(rid)
        if not ex: mis.append('ROW '+rid+' NOT_IN_CFG'); continue
        for i,(want,got) in enumerate(zip(ex,r)):
            if want!=got: mis.append('ID%s C%d EXP=%s GOT=%s'%(rid,i,repr(want),repr(got)))
    aiOk=final in p.get('conclusion','')
    texts=p.get('aiNote','')+'|'+p.get('basis','')+'|'+p.get('honesty','')+'|'+p.get('ctaText','')
    fourOk=(final in texts) and (basis in texts) and (honesty in texts) and ('查看合成技能 →' in texts) and ('AI 机制概览' in texts)
    w=p.get('wrap',{}) or {}
    scrollOk=(w.get('overflow')=='auto' and w.get('sw',0)>=w.get('cw',0) and (w.get('sw',0)==w.get('cw',0) or w.get('after',0)>0))
    docOk=p.get('docW',0)<=p.get('innerW',0)+1
    ctaOk=p.get('ctaText')=='查看合成技能 →' and p.get('ctaVisible') and p.get('ctaHref')=='#/wiki?tab=skills&anchor=composite'
    ok=(not mis) and aiOk and fourOk and scrollOk and docOk and ctaOk and not p.get('badText') and p.get('sectionVisible') and len(rows)==len(expect)
    allok=allok and ok
    print('VP=%s ROWS=%d MISMATCHES=%d AI_OK=%s FOUR_OK=%s BAD=%s DOC=%s/%s W=%s->%s/%s/%s CTA=%s VERDICT=%s'%(vp,len(rows),len(mis),aiOk,fourOk,p.get('badText'),p.get('docW'),p.get('innerW'),w.get('before'),w.get('after'),w.get('sw'),w.get('cw'),repr(p.get('ctaText')),'PASS' if ok else 'FAIL'))
    for m in mis[:15]: print('  MIS '+m)
print('ALL_RECONCILE_OK='+str(allok))
