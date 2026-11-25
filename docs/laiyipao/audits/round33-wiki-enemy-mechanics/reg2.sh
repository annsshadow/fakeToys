#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r33-reg2
D=docs/laiyipao/audits/round33-wiki-enemy-mechanics; E="$PWD/$D"
BASE='http://127.0.0.1:5175/laiyipao-admin/#/wiki'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$BASE?tab=enemies" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
echo '=== 表内横向滚动末列 ==='
"$AB" --session "$S" eval --stdin < "$E/scroll-probe.js" > "$E/data/scroll.json" 2>&1
echo '=== R31 reaction tab ==='
"$AB" --session "$S" open "$BASE?tab=basics" >/dev/null 2>&1; sleep 5
"$AB" --session "$S" eval --stdin < "$E/react-probe.js" > "$E/data/reg-reaction.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r33-06-reaction-tab.png" >/dev/null 2>&1
sha256sum "$E/screenshots/r33-06-reaction-tab.png"
echo '=== R32 skills tab ==='
"$AB" --session "$S" open "$BASE?tab=skills" >/dev/null 2>&1; sleep 5
"$AB" --session "$S" eval --stdin < "$E/skill-probe.js" > "$E/data/reg-skill.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r33-07-skill-tab.png" >/dev/null 2>&1
sha256sum "$E/screenshots/r33-07-skill-tab.png"
echo REG2_DONE=YES