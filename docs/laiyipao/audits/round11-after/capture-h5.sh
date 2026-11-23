#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-after'
i=0
for pg in skills/skills equipment/equipment gems/gems battles/battles; do
  i=$((i+1))
  case $i in 1) nm=skills;; 2) nm=equipment;; 3) nm=gems;; 4) nm=battles;; esac
  S=lyp-r11-h5-$i
  "$AB" --session $S close >/dev/null 2>&1
  "$AB" --session $S open 'about:blank' >/dev/null 2>&1
  sleep 2
  "$AB" --session $S set viewport 390 844 >/dev/null 2>&1
  sleep 1
  "$AB" --session $S --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/$pg" >/dev/null 2>&1
  sleep 4
  vp=$("$AB" --session $S eval 'window.innerWidth+"x"+window.innerHeight' 2>&1 | tail -1)
  if [ "$vp" != '"390x844"' ]; then
    "$AB" --session $S set viewport 390 844 >/dev/null 2>&1; sleep 1;
    "$AB" --session $S reload >/dev/null 2>&1; sleep 4;
  fi
  echo "=== H5 $nm ==="
  echo -n 'VP: '; "$AB" --session $S eval 'window.innerWidth+"x"+window.innerHeight' 2>&1 | tail -1
  "$AB" --session $S eval --stdin < "$D/probe-detail.js" 2>&1 | tail -1
  "$AB" --session $S eval --stdin < "$D/dom-probe.js" 2>&1 | head -1
  "$AB" --session $S screenshot "$D/screenshots/r11-$nm.png" 2>&1 | head -1
done
echo H5_DONE