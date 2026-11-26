#!/bin/bash
# t75 闭环：user10(3场真实战报) reload 后 t74 CTA→impact→两步弹层→确认写入
DA="docs/laiyipao/audits/round7-bugfix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7fix; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$DA/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
clickxy(){ run mouse move "$1" "$2" >/dev/null; sleep 0.15; run mouse down >/dev/null; sleep 0.15; run mouse up >/dev/null; sleep 0.45; }
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=10;' | tr -d ' ')
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
sed "s/__TOKEN__/$TOK/" "$DA/set-token2.js.tpl" > "$DA/set-token2.js"
run open "$BASE/pages/login/login" >/dev/null; sleep 3
run eval --stdin < "$DA/set-token2.js" >/dev/null
echo "AUTH: $(j diag-token.js)"
echo "CTAP: $(j cta-params.js)"
for P in skills equipment; do
  echo "=== PAGE $P ==="
  run open "$BASE/pages/$P/$P" >/dev/null; sleep 4
  run eval --stdin < "$DA/set-token2.js" >/dev/null
  run reload >/dev/null; sleep 9
  echo "STATE_LOADED: $(j state.js)"
  run screenshot "$DA/screenshots/r7fix-$P-loaded.png" >/dev/null 2>&1
  echo "ME_BEFORE: $(j me-build.js)"
  M=$(j open-modal.js); echo "MODAL_CTA: $M"
  set -- $M; if [ "$1" = "MODAL_CTA" ]; then clickxy "$2" "$3"; fi; sleep 2
  echo "MODAL_STATE: $(j modal-state.js)"
  run screenshot "$DA/screenshots/r7fix-$P-modal-data.png" >/dev/null 2>&1
  C=$(j confirm-btn.js); echo "CONFIRM_BTN: $C"
  set -- $C; if [ "$1" = "CONFIRM" ]; then clickxy "$2" "$3"; fi; sleep 4
  echo "STATE_AFTER: $(j state.js)"
  echo "ME_AFTER: $(j me-build.js)"
  run screenshot "$DA/screenshots/r7fix-$P-after-apply.png" >/dev/null 2>&1
  run open "$BASE/pages/$P/$P" >/dev/null; sleep 4
  run eval --stdin < "$DA/set-token2.js" >/dev/null; run reload >/dev/null; sleep 8
  echo "STATE_RELOAD: $(j state.js)"
  run screenshot "$DA/screenshots/r7fix-$P-after-reload.png" >/dev/null 2>&1
done
echo "=== FINAL ==="
echo "FINAL: $(j state.js)"
echo BUGFIX_DONE
