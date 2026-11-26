#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10final
D=docs/laiyipao/audits/round10-causality
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 3
"$AB" --session "$S" eval "(async()=>{const t=localStorage.getItem('lyp_token');const r=await fetch('http://127.0.0.1:8080/api/v1/me/battles?limit=20',{headers:{Authorization:'Bearer '+t}});const j=await r.json();const items=j.items||[];return JSON.stringify({httpStatus:r.status,count:items.length,rows:items.slice(0,6).map(x=>({id:x.id,level_id:x.level_id,replay_hash:x.replay_hash,created_at:x.created_at,score:x.score,kills:x.kills}))})})()" > "$D/battles-api-evidence.txt" 2>&1
echo "API_EVIDENCE:"; cat "$D/battles-api-evidence.txt"