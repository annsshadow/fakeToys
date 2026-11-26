#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10settle
D=docs/laiyipao/audits/round10-causality/screenshots
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
"$AB" --session "$S" click "text=8×" >/dev/null 2>&1 || "$AB" --session "$S" eval "[...document.querySelectorAll('view,text')].find(e=>e.textContent.trim()==='8×')?.click()" >/dev/null 2>&1
for i in 1 2 3 4 5 6 7 8; do
  sleep 12
  TXT=$("$AB" --session "$S" eval "document.body.innerText.includes('养成前后')" 2>/dev/null)
  echo "T$i $TXT"
  if echo "$TXT" | grep -q true; then
    "$AB" --session "$S" screenshot "$D/after-07-settle-growth.png" >/dev/null 2>&1
    echo SETTLE_CAPTURED
    break
  fi
done
echo CAPTURE_END