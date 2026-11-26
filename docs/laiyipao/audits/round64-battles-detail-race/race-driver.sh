#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"
D="$P/data"
S="r64race"
AB(){ agent-browser --session "$1" "${@:2}"; }
AUTH="$D/auth-init.js"
INIT="$P/race-init.js"
PROBE="$P/r64-probe.js"
URL="http://127.0.0.1:5201/#/pages/battles/battles"

AB "$S" open --init-script "$AUTH" --init-script "$INIT" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "$URL" >/dev/null 2>&1
sleep 4
echo "=== BEFORE ==="
AB "$S" eval --stdin < "$PROBE" > "$D/race-00-before.txt" 2>"$D/race-00.err"; cat "$D/race-00-before.txt"; echo

echo "=== ARM ==="
AB "$S" eval "window.__raceCtl.delayMatch=\"/me/battles/477\";window.__raceCtl.delayMs=7000;window.__raceCtl.remaining=1;window.__raceCtl.log=[];\"ARMED\"" 2>&1

echo "=== MARK ROWS ==="
AB "$S" eval "(function(){var r=document.querySelectorAll(\".battle-row\");if(r.length<2)return \"ROWS_LT2:\"+r.length;r[0].setAttribute(\data-qa\",\"A\");r[1].setAttribute(\data-qa\",\"B\");return \"MARKED:\"+r.length+\":\"+r[0].innerText.slice(0,40)+\":\"+r[1].innerText.slice(0,40)})()" 2>&1

echo "=== CLICK A (delayed detail) ==="
AB "$S" click "[data-qa=A]" 2>&1
sleep 1
echo "=== INTERIM AFTER A ==="
AB "$S" eval --stdin < "$PROBE" > "$D/race-01-afterA.txt" 2>"$D/race-01.err"; cat "$D/race-01-afterA.txt"; echo

echo "=== CLICK B (fast detail, becomes current) ==="
AB "$S" click "[data-qa=B]" 2>&1
sleep 3
echo "=== AFTER B (B panels filled) ==="
AB "$S" eval --stdin < "$PROBE" > "$D/race-02-afterB.txt" 2>"$D/race-02.err"; cat "$D/race-02-afterB.txt"; echo
AB "$S" screenshot "$P/screenshots/race-768-afterB.png" >/dev/null 2>&1

echo "=== WAIT FOR STALE A RESPONSE ==="
sleep 8
AB "$S" eval --stdin < "$PROBE" > "$D/race-03-final.txt" 2>"$D/race-03.err"; cat "$D/race-03-final.txt"; echo
AB "$S" screenshot "$P/screenshots/race-768-final.png" >/dev/null 2>&1
echo "=== RACE DRIVER DONE ==="
