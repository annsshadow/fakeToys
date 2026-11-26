const fs=require('fs');
const p='docs/laiyipao/audits/round28-live-reaction-attribution/data/samples.jsonl';
const ctaData=require('child_process');
const lines=fs.readFileSync(p,'utf8').trim().split('\n').filter(Boolean);
function parse(l){let s=l;try{s=JSON.parse(s)}catch(e){};return JSON.parse(s)}
const arr=lines.map(parse).filter(Boolean);
const BASIS='依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用';
const HON='辅助归因非权威判据，权威以 replay_hash 为准';
const CTA='查看完整反应时间线 →';
let r={total:arr.length,states:{},lineCounts:{},basisOk:0,honOk:0,ctaOk:0,ratioOk:0,attribOk:0,unknownRows:0,overflow:0,undef:0,nan:0,r24:0,r26:0,r27:0,helpSeen:0,aiTitleOk:0,firstSample:null,lastSample:null};
for(const s of arr){
  r.states[s.state]=(r.states[s.state]||0)+1;
  r.lineCounts[s.lineCount]=(r.lineCounts[s.lineCount]||0)+1;
  if(s.aiBasis===BASIS)r.basisOk++;
  if(s.aiHonesty===HON)r.honOk++;
  if(s.aiCta===CTA)r.ctaOk++;
  if(s.aiTitle==='AI · 反应归因')r.aiTitleOk++;
  if(s.docOverflow||s.feedOverflow)r.overflow++;
  if(s.hasUndefined)r.undef++;if(s.hasNaN)r.nan++;
  if(s.hasR24)r.r24++;if(s.hasR26)r.r26++;if(s.hasR27)r.r27++;
  if(s.helpVisible)r.helpSeen++;
  const m=(s.aiConcl||'').match(/已归因\s*(\d+)\/(\d+)/);
  if(m&&Number(m[2])===s.lineCount)r.ratioOk++;
  for(const ln of (s.feedLines||[])){
    if(ln.attribution==='归因不可用'){r.unknownRows++;continue}
    const mm=(ln.attribution||'').match(/^第(\d+)波 · #(\d+)$/);
    if(mm&&mm[1]===ln.dataWave&&mm[2]===ln.dataEnemy)r.attribOk++;
  }
  if(!r.firstSample)r.firstSample={lineCount:s.lineCount,concl:s.aiConcl,lines:s.feedLines};
  r.lastSample={state:s.state,lineCount:s.lineCount,concl:s.aiConcl,lines:s.feedLines};
}
console.log(JSON.stringify(r,null,2));
