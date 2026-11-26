#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-vis
D=docs/laiyipao/audits/round57-visual-truncation
VW=(390 768 1280); VH=(844 1024 800)
for i in 0 1 2; do W=${VW[$i]}; H=${VH[$i]};
  for pg in announcements/announcements enemies/enemies; do name=${pg/\//-};
    AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1; sleep 2
    AB "$S" eval --stdin < "$D/r57-refined-probe.js" > "$D/data/refined-$W-$name.json" 2>"$D/data/refinederr-$W-$name.txt";
    echo "DONE $W $name"; done; done
echo REFINE_FINISH