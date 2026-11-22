#!/usr/bin/env bash
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7diag2
$AB --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills"
sleep 2
$AB --session "$S" set viewport 390 844
$AB --session "$S" eval "(async()=>{const t=localStorage.getItem('lyp_token');const h={Authorization:'Bearer '+t};const r=await fetch('http://127.0.0.1:8080/api/v1/me/recommendations?context=skills',{headers:h});const txt=await r.text();let j={};try{j=JSON.parse(txt)}catch(e){};const p=j.insights&&j.insights[0]&&j.insights[0].cta&&j.insights[0].cta.params;let impact=null;if(p){const ir=await fetch('http://127.0.0.1:8080/api/v1/me/recommendations/impact?kind=skill&id='+p.id+'&replace='+p.replace,{headers:h});impact={status:ir.status,body:(await ir.text()).slice(0,1800)}}return JSON.stringify({recStatus:r.status,recBody:txt.slice(0,1400),params:p||null,impact})})()"
echo DIAG2_DONE
