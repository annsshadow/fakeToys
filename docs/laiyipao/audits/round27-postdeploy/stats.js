const fs=require('fs');
const D='docs/laiyipao/audits/round27-postdeploy/data/';
function loadLines(p){try{return fs.readFileSync(D+p,'utf8').split(/\r?\n/).filter(Boolean)}catch(e){return []}}
function parse2(line){try{let v=JSON.parse(line);if(typeof v==='string')v=JSON.parse(v);return v}catch(e){return null}}
const raw=loadLines('postdeploy-samples.jsonl');
const S=raw.map(parse2).filter(Boolean);
console.log('SAMPLES',S.length,'RAWLINES',raw.length);
const byState={}; S.forEach(s=>{byState[s.state||'?']=(byState[s.state||'?']||0)+1});
console.log('STATES',JSON.stringify(byState));
// 波次覆盖
const waves={}; S.forEach(s=>{const k=s.waveIdx; if(k==null)return; if(!waves[k])waves[k]={total:new Set(),pcts:[],resolved:new Set()}; waves[k].total.add(s.domTotal); waves[k].resolved.add(s.domResolved); if(s.domPct!=null)waves[k].pcts.push(Number(s.domPct));});
Object.keys(waves).sort((a,b)=>a-b).forEach(k=>{const w=waves[k];console.log('WAVE',k,'totals',[...w.total].join(','),'resolved',[...w.resolved].sort((a,b)=>a-b).join(','),'pctMin',Math.min(...w.pcts),'pctMax',Math.max(...w.pcts),'has100',w.pcts.includes(100));});
// 100%且下一波归零
const full=S.filter(s=>Number(s.domPct)>=99.95); console.log('FULL_FRAMES',full.length,'fullSample',full.length?JSON.stringify({idx:full[0].waveIdx,total:full[0].domTotal,resolved:full[0].domResolved,pct:full[0].domPct,text:full[0].progText}):'none');
const zeroAfter=S.filter(s=>s.waveIdx>0&&Number(s.domPct)===0); console.log('ZERO_NEXT_FRAMES',zeroAfter.length,'zeroSample',zeroAfter.length?JSON.stringify({idx:zeroAfter[0].waveIdx,total:zeroAfter[0].domTotal,pct:zeroAfter[0].domPct}):'none');
// fill宽度一致性(progText pct vs fillW% of track width) —— 报道只需记录逐帧progText一位小数
const badPct=S.filter(s=>s.progText&&!/\d\.\d%/.test(s.progText)); console.log('BAD_PCT_FORMAT',badPct.length);
// AI四判据
const BASIS='依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash';
const HONESTY='辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准';
const withProg=S.filter(s=>s.hasProg===true||s.hasProg==='true');
console.log('HAS_PROG_FRAMES',withProg.length);
console.log('AI_TITLE_OK',withProg.filter(s=>s.aiTitle&&s.aiTitle.includes('AI')).length);
console.log('AI_CONCL_OK',withProg.filter(s=>s.aiConcl&&/本波共/.test(s.aiConcl)).length);
console.log('AI_BASIS_OK',withProg.filter(s=>s.aiBasis&&s.aiBasis.includes('只读派生不进 replay_hash')).length);
console.log('AI_CTA_OK',withProg.filter(s=>s.aiCta&&s.aiCta.includes('继续清理场上敌人')).length);
console.log('AI_HONESTY_OK',withProg.filter(s=>s.aiHonesty&&s.aiHonesty.includes('权威以 replay_hash 为准')).length);
console.log('R24_FRAMES',S.filter(s=>s.hasR24===true||s.hasR24==='true').length,'R26_FRAMES',S.filter(s=>s.hasR26===true||s.hasR26==='true').length,'HUDWRAP_OK',S.filter(s=>s.hudWrap===true||s.hudWrap==='true').length,'DOC_OF_X',S.filter(s=>s.docOverflow===true||s.docOverflow==='true').length);
console.log('UNDEF_FRAMES',S.filter(s=>s.hasUndefined===true||s.hasUndefined==='true').length,'NAN_FRAMES',S.filter(s=>s.hasNaN===true||s.hasNaN==='true').length);
const ticks=loadLines('postdeploy-tick-log.txt'); console.log('TICKS',ticks.length,'LAST',ticks[ticks.length-1]||'none');
