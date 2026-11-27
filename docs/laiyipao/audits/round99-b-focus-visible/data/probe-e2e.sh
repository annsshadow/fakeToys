#!/bin/bash
AB=agent-browser
S=r99probe
D=/d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/data
$AB close --all --session $S >/dev/null 2>&1
$AB open "http://127.0.0.1:5174/#/pages/me/me" --session $S 2>&1 | head -3
$AB set viewport 390 844 --session $S >/dev/null 2>&1
$AB set media light reduced-motion --session $S >/dev/null 2>&1
$AB wait 1500 --session $S >/dev/null 2>&1
for i in 1 2 3 4 5 6 7 8; do
  echo "=== TAB $i ==="
  $AB press Tab --session $S >/dev/null 2>&1
  $AB eval --stdin --session $S < "$D/probe.js"
done
$AB close --all --session $S >/dev/null 2>&1
echo DONE
