#!/usr/bin/env bash
set -u
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd
SESSION=lyp-r7growth
BASE=http://127.0.0.1:5174/#
OUT=docs/laiyipao/audits/round7-growth/screenshots
mkdir -p "$OUT"
"$AB" --session "$SESSION" open "$BASE/pages/index/index"
"$AB" --session "$SESSION" wait 1800
"$AB" --session "$SESSION" set viewport 390 844
"$AB" --session "$SESSION" eval "(()=>{try{localStorage.getItem('lyp_token')||'';return 'logged'}catch(e){return 'no'}})()"
for page in skills equipment gems; do
  "$AB" --session "$SESSION" open "$BASE/pages/$page/$page"
  "$AB" --session "$SESSION" wait 2200
  "$AB" --session "$SESSION" set viewport 390 844
  "$AB" --session "$SESSION" screenshot "$OUT/after-$page.png"
  echo "CAPTURED $page"
done
