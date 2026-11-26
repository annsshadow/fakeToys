#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round29-postdeploy
node -e '
var fs=require("fs"),path=process.argv[1];
function j(p){var o=JSON.parse(fs.readFileSync(p,"utf8").trim());if(typeof o==="string")o=JSON.parse(o);return o}
var probe=j(path+"/data/result-open-probe.json");
var det=j(path+"/data/detail.json").data||j(path+"/data/detail.json");
var ev=(det.reaction_timeline&&det.reaction_timeline.events)||[];
var rows=probe.timelineRows||[];
var last=ev.slice(-rows.length);
var ok=rows.length>0&&last.length===rows.length;
var mismatches=[];
for(var i=0;i<rows.length&&i<last.length;i++){
  var r=rows[i],e=last[i];
  var m=/^第(\d+)波 · (\d+) · (.+?) · #(\d+) · ([\d.]+) 伤害$/.exec(r.text||"");
  var good=!!m && Number(m[1])===e.wave && Number(m[4])===e.enemy_uid && Math.abs(Number(m[5])-Number(e.damage))<0.01 && Number(r.wave)===e.wave && Number(r.enemy)===e.enemy_uid && Number(r.idx)===i;
  if(!good)mismatches.push({idx:i,dom:r.text,api:{wave:e.wave,tick:e.tick,reaction:e.reaction,enemy_uid:e.enemy_uid,damage:e.damage}});
}
console.log("ROW_COUNT="+rows.length+" DETAIL_TOTAL="+ev.length+" TRUNCATED="+(det.reaction_timeline||{}).truncated+" MISMATCHES="+mismatches.length);
console.log("LAST3_DOM="+JSON.stringify(rows.slice(-3).map(function(r){return r.text})));
console.log("LAST3_API="+JSON.stringify(last.slice(-3).map(function(e){return {wave:e.wave,tick:e.tick,reaction:e.reaction,enemy_uid:e.enemy_uid,damage:e.damage}})));
console.log("FULL_RECONCILE_OK="+(ok&&mismatches.length===0));
if(mismatches.length)console.log("MISMATCH_DETAIL="+JSON.stringify(mismatches.slice(0,5)));
' "$D"
