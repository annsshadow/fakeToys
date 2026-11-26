#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
PROBE='(function(){var d=document.documentElement,b=document.body;return JSON.stringify({w:innerWidth,docW:d.scrollWidth,bodyW:b.scrollWidth,clientW:d.clientWidth,hOverflow:d.scrollWidth>d.clientWidth+1,hash:location.hash})})()'
for spec in "390 844 390" "768 1024 768" "1280 800 1280"; do
  set -- $spec; W=$1; H=$2; V=$3; S="ov$V"
  AB "$S" open --init-script "$D/auth-init.js" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
  sleep 3
  echo "OV_$V=$(AB "$S" eval "$PROBE" 2>&1)"
done
echo OV_DONE