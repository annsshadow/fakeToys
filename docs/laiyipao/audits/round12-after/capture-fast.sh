#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-fast
# 等后端就绪
for i in $(seq 1 30); do [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/healthz 2>/dev/null)" = 200 ] && break; sleep 2; done
# 弱构筑 user10
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "delete from user_skill_slots where user_id=10 and slot>0" >/dev/null 2>&1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "update user_skill_slots set skill_id=1 where user_id=10 and slot=0" >/dev/null 2>&1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "update user_equipment set equipped=false where user_id=10" >/dev/null 2>&1
bash "$D/mint-token.sh" >/dev/null 2>&1
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; sleep 1
"$AB" --session $S --init-script "$D/auth-init.js" open 'http://127.0.0.1:5174/?r12=7#/pages/battle/battle?level=60' >/dev/null 2>&1; sleep 6
echo SPEED=$("$AB" --session $S eval --stdin < "$D/click-speed.js" 2>&1 | head -1)
for i in $(seq 1 100); do
  R=$("$AB" --session $S eval --stdin < "$D/battle-tick.js" 2>&1 | head -1)
  echo "T$i=$R"
  case "$R" in *SETTLED*) break;; esac
  sleep 1
done
sleep 3
echo '=== SETTLE PROBE ==='
"$AB" --session $S eval --stdin < "$D/defeat-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-final-settle.png" 2>&1 | tail -1
echo '=== GOTO REPORT ==='
R=$("$AB" --session $S eval '[].slice.call(document.querySelectorAll("*")).filter(function(e){return(e.innerText||"").trim()==="查看本局战报"}).pop().click()' 2>&1 | head -1)
echo REPORT_CLICK=$R
sleep 3
echo '=== REPORT PROBE ==='
"$AB" --session $S eval --stdin < "$D/defeat-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-final-report.png" 2>&1 | tail -1
echo '=== ERRORS ==='; "$AB" --session $S errors --json 2>&1 | head -c 250
echo FAST_DONE