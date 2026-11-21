#!/bin/bash
# t49 index 诚实文案 + AoE 25% 不回退复查
AB="agent-browser --session lyp-r5after-h5"
BASE="http://127.0.0.1:5174/#"
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/INDEX: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),falseClaimNone:(document.body.innerText.indexOf("与面板攻击力无关")<0&&document.body.innerText.indexOf("无关攻击力")<0),honestContribution:(document.body.innerText.indexOf("占比受限")>=0||document.body.innerText.indexOf("权重上限")>=0),honestLine:(function(){var ls=document.body.innerText.split(String.fromCharCode(10));for(var i=0;i<ls.length;i++){if(ls[i].indexOf("反应伤害")>=0&&ls[i].indexOf("攻击力")>=0)return ls[i]}return ""})(),probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
$AB open "$BASE/pages/reactions/reactions" >/dev/null 2>&1
sleep 3
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/AOE: /"
JSON.stringify({url:location.href,pct25:(document.body.innerText.match(/25%/g)||[]).length,radius120:(document.body.innerText.match(/半径 120/g)||[]).length,steam:(document.body.innerText.indexOf("蒸汽")>=0||document.body.innerText.indexOf("蒸腾")>=0),overheat:(document.body.innerText.indexOf("过热")>=0)})
EOF
echo DONE