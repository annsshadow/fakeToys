#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round61-form-a11y"
S=lyp-r61-postfix
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
CASES=("390 844 announcements" "768 1024 announcements" "1280 800 announcements" "390 844 login" "768 1024 login" "1280 800 login" "390 844 verify" "768 1024 verify" "1280 800 verify" "390 844 signin" "768 1024 signin" "1280 800 signin" "390 844 me" "768 1024 me" "1280 800 me" "390 844 bag" "768 1024 bag" "1280 800 bag" "390 844 equipment" "768 1024 equipment" "1280 800 equipment" "390 844 enemies" "768 1024 enemies" "1280 800 enemies" "390 844 gems" "768 1024 gems" "1280 800 gems" "390 844 reactions" "768 1024 reactions" "1280 800 reactions" "390 844 skills" "768 1024 skills" "1280 800 skills" "390 844 skins" "768 1024 skins" "1280 800 skins")
for C in "${CASES[@]}"; do
 set -- $C; W=$1; H=$2; P=$3; N="$W-$P"
 AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null 2>&1
 AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
 sleep 2
 AB "$S" eval --stdin < "$D/r61-postfix-probe.js" > "$D/data/pf-empty-$N.json" 2>"$D/data/pf-empty-$N.err"
 AB "$S" snapshot --json > "$D/data/pf-ax-empty-$N.json" 2>"$D/data/pf-ax-empty-$N.err"
 AB "$S" eval --stdin < "$D/r61-postfix-fill.js" > "$D/data/pf-fill-$N.txt" 2>&1
 sleep 1
 AB "$S" eval --stdin < "$D/r61-postfix-probe.js" > "$D/data/pf-filled-$N.json" 2>"$D/data/pf-filled-$N.err"
 AB "$S" snapshot --json > "$D/data/pf-ax-filled-$N.json" 2>"$D/data/pf-ax-filled-$N.err"
 AB "$S" screenshot "$D/screenshots/pf-$N.png" >/dev/null 2>&1
 echo "DONE $N probeE=$(wc -c < "$D/data/pf-empty-$N.json") probeF=$(wc -c < "$D/data/pf-filled-$N.json") axE=$(wc -c < "$D/data/pf-ax-empty-$N.json") axF=$(wc -c < "$D/data/pf-ax-filled-$N.json") errE=$(wc -c < "$D/data/pf-empty-$N.err") errF=$(wc -c < "$D/data/pf-filled-$N.err")"
done
# me expanded password form at 768
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "UPG=$(AB "$S" eval --stdin < "$D/trig-me-upgrade.js" 2>&1)"; sleep 1
echo "PWD=$(AB "$S" eval --stdin < "$D/trig-me-pwd.js" 2>&1)"; sleep 1
AB "$S" eval --stdin < "$D/r61-postfix-probe.js" > "$D/data/pf-empty-768-me-expanded.json" 2>"$D/data/pf-empty-768-me-expanded.err"
AB "$S" snapshot --json > "$D/data/pf-ax-empty-768-me-expanded.json" 2>"$D/data/pf-ax-empty-768-me-expanded.err"
AB "$S" eval --stdin < "$D/r61-postfix-fill.js" > "$D/data/pf-fill-768-me-expanded.txt" 2>&1; sleep 1
AB "$S" eval --stdin < "$D/r61-postfix-probe.js" > "$D/data/pf-filled-768-me-expanded.json" 2>"$D/data/pf-filled-768-me-expanded.err"
AB "$S" snapshot --json > "$D/data/pf-ax-filled-768-me-expanded.json" 2>"$D/data/pf-ax-filled-768-me-expanded.err"
AB "$S" screenshot "$D/screenshots/pf-768-me-expanded.png" >/dev/null 2>&1
echo "DONE 768-me-expanded probeE=$(wc -c < "$D/data/pf-empty-768-me-expanded.json") probeF=$(wc -c < "$D/data/pf-filled-768-me-expanded.json")"
AB "$S" close >/dev/null 2>&1 || true
echo PF_WALK_FINISH