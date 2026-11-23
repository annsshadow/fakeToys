#!/bin/bash
D=docs/laiyipao/audits/round12-after
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
SESSION=lyp-r12-h5b
BASE='http://127.0.0.1:5174/#/pages'
PAGES='index/index skills/skills equipment/equipment gems/gems bag/bag battles/battles battle/battle'
FIRST=1
for P in $PAGES; do
  NAME=$(echo "$P" | sed 's|/|-|g')
  if [ "$FIRST" = 1 ]; then
    "$AB" --session "$SESSION" --init-script "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/$D/auth-init.js" open 'about:blank' 2>/dev/null
    sleep 2
    "$AB" --session "$SESSION" set viewport 390 844 2>/dev/null; sleep 1
    "$AB" --session "$SESSION" open "$BASE/$P" 2>/dev/null
    FIRST=0
  else
    "$AB" --session "$SESSION" open "$BASE/$P" 2>/dev/null
  fi
  sleep 4
  echo "=== $P ==="
  "$AB" --session "$SESSION" get url 2>/dev/null | tail -1
  "$AB" --session "$SESSION" get title 2>/dev/null | tail -1
  if [ "$P" = 'battle/battle' ]; then PROBE=defeat-probe.js; else PROBE=dom-probe.js; fi
  "$AB" --session "$SESSION" eval --stdin < "$D/$PROBE" 2>/dev/null | tail -1
  "$AB" --session "$SESSION" screenshot "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/$D/screenshots/r12-h5-$NAME.png" 2>/dev/null | tail -1
done