#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round29-live-reaction-timeline
node -e '
var fs=require("fs"),path=process.argv[1];
function j(p){var o=JSON.parse(fs.readFileSync(p,"utf8").trim());if(typeof o==="string")o=JSON.parse(o);return o}
var probe=j(path+"/data/settled-open-probe.json");
var det=j(path+"/data/detail.json").data||j(path+"/data/detail.json");
var ev=(det.reaction_timeline&&det.reaction_timeline.events)||[];
var rows=probe.timelineRows||[];
var last=ev.slice(-rows.length);
var ok=rows.length>0&&last.length===rows.length;
var cmp=[];
for(var i=0;i<rows.length&&ok;i++){
  var r=rows[i],e=last[i];
  var m=/^第(\d+)波 · (\d+) · (.+?) · #(\d+) · ([\d.]+) 伤害$/.exec(r.text||"");
  var good=!!m && Number(m[1])===e.wave && Number(m[4])===e.enemy_uid && Number(m[5])===e.damage && Number(r.wave)===e.wave && Number(r.enemy)===e.enemy_uid && Number(r.idx)===i;
  cmp.push({idx:i,wave:e.wave,tick:e.tick,reaction:e.reaction,damage:e.damage,enemy_uid:e.enemy_uid,dom:r.text,match:good});
  ok=ok&&good;
}
console.log("ROW_COUNT="+rows.length+" DETAIL_TOTAL="+ev.length+" TRUNCATED="+(det.reaction_timeline||{}).truncated);
console.log("COMPARE="+JSON.stringify(cmp));
console.log("RECONCILE_OK="+ok);
' "$D"
