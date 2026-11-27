#!/usr/bin/env bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB(){ local S="$1"; shift; agent-browser --session "$S" "$@"; }
BASE='http://127.0.0.1:5174/'
D='docs/laiyipao/audits/round96-empty-cta-keyboard/data'
S="r96-trial-battles"
{
 AB "$S" close --all >/dev/null 2>&1 || true
 AB "$S" open "$BASE" >/dev/null 2>&1
 AB "$S" set viewport 768 1024 >/dev/null 2>&1
 AB "$S" navigate "$BASE#/pages/battles/battles" >/dev/null 2>&1
 sleep 4
 echo "--- PROBE ---"
 AB "$S" eval --stdin < "$D/probe-cta.js" 2>&1
 echo "--- AX_SNAPSHOT ---"
 AB "$S" snapshot 2>&1 | grep -iE "button|关卡|开打|重新加载" | head -20
 echo "--- TABS ---"
 for i in 1 2 3 4 5 6 7 8; do
   AB "$S" press Tab >/dev/null 2>&1; printf "T%s " "$i"
   AB "$S" eval --stdin < "$D/probe-active.js" 2>&1
 done
 AB "$S" close --all >/dev/null 2>&1 || true
} > "$D/trial-out.txt" 2>&1
echo TRIAL_DONE
cat "$D/trial-out.txt"