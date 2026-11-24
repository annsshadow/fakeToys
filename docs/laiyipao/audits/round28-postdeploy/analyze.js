const fs=require('fs');
const D='docs/laiyipao/audits/round28-postdeploy/data';
function parseLine(s){s=s.trim(); if(!s)return null; try{var o=JSON.parse(s); if(typeof o==='string')o=JSON.parse(o); return o;}catch(e){return null;}}
var rows=fs.readFileSync(D+'/samples.jsonl','utf8').split(/\r?\n/).map(parseLine).filter(Boolean);
var stats={frames:rows.length,states:{},lineCounts:{},attribOk:0,attribTotal:0,unknownRows:0,ratioOk:0,aiTitleOk:0,basisOk:0,honOk:0,ctaOk:0,r24:0,r26:0,r27:0,undef:0,nan:0,docOverflow:0,feedOverflowFrames:0};
var basisText='依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用';
var honText='辅助归因非权威判据，权威以 replay_hash 为准';
var ctaText='查看完整反应时间线 →';
function attrCheck(l){var a=l.attribution||''; if(a==='归因不可用'){return l.dataWave==='unknown'&&l.dataEnemy==='unknown';} var m=a.match(/第(\d+)波 · #(\d+)/); if(!m)return false; return String(l.dataWave)===m[1]&&String(l.dataEnemy)===m[2];}
rows.forEach(function(s){
  stats.states[s.state]=(stats.states[s.state]||0)+1;
  var n=s.lineCount||0; stats.lineCounts[n]=(stats.lineCounts[n]||0)+1;
  (s.feedLines||[]).forEach(function(l){stats.attribTotal++; if(attrCheck(l))stats.attribOk++; if(l.attribution==='归因不可用')stats.unknownRows++;});
  var m=(s.aiConcl||'').match(/已归因\s*(\d+)\/(\d+)/); if(m&&Number(m[1])===n&&Number(m[2])===n)stats.ratioOk++;
  if((s.aiTitle||'').indexOf('AI')>=0)stats.aiTitleOk++;
  if((s.aiBasis||'')===basisText)stats.basisOk++;
  if((s.aiHonesty||'')===honText)stats.honOk++;
  if((s.aiCta||'')===ctaText)stats.ctaOk++;
  if(s.hasR24)stats.r24++; if(s.hasR26)stats.r26++; if(s.hasR27)stats.r27++;
  if(s.hasUndefined)stats.undef++; if(s.hasNaN)stats.nan++;
  if(s.docOverflow)stats.docOverflow++; if(s.feedOverflow)stats.feedOverflowFrames++;
});
console.log('STATS='+JSON.stringify(stats));
['cta-pre','cta-post','cta-closed'].forEach(function(k){var o=parseLine(fs.readFileSync(D+'/'+k+'.json','utf8')); console.log('CTA_'+k+'='+JSON.stringify({state:o&&o.state,helpVisible:o&&o.helpVisible,lineCount:o&&o.lineCount}));});
var fin=parseLine(fs.readFileSync(D+'/final.json','utf8'));
console.log('FINAL='+JSON.stringify({result:fin&&fin.result,hash:fin&&fin.hash,settle:fin&&fin.settle,hashResponses:fin&&fin.hashResponses,pe:fin&&fin.pe,ce:fin&&fin.ce,af:fin&&fin.af,fr:fin&&fin.fr,peList:fin&&fin.peList}));
var peList=(fin&&fin.peList)||[]; var real=peList.filter(function(x){return String(x).indexOf('Autoplay')<0&&String(x).indexOf('play() request')<0;});
console.log('PE_SAMPLE_REAL='+real.length);