#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r33post-reg2
D=docs/laiyipao/audits/round33-postdeploy; E="$PWD/$D"
BASE='http://127.0.0.1:5175/laiyipao-admin/#/wiki'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$BASE?tab=enemies" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
echo '=== TABLE HORIZONTAL SCROLL ==='
"$AB" --session "$S" eval --stdin < "$E/scroll-probe.js" > "$E/data/scroll.json" 2>&1
echo '=== R31 REACTION TAB ==='
"$AB" --session "$S" open "$BASE?tab=basics" >/dev/null 2>&1; sleep 5
"$AB" --session "$S" eval --stdin < "$E/react-probe.js" > "$E/data/reg-reaction.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r33post-reaction-tab.png" >/dev/null 2>&1
echo '=== R32 SKILL TAB ==='
"$AB" --session "$S" open "$BASE?tab=skills" >/dev/null 2>&1; sleep 5
"$AB" --session "$S" eval --stdin < "$E/skill-probe.js" > "$E/data/reg-skill.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r33post-skill-tab.png" >/dev/null 2>&1
"$AB" --session "$S" close >/dev/null 2>&1 || true
sha256sum "$E/screenshots/r33post-reaction-tab.png" "$E/screenshots/r33post-skill-tab.png"
echo REG2_DONE=YES
