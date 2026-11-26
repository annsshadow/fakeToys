#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-vis
D=docs/laiyipao/audits/round57-visual-truncation
for pg in me/me index/index reactions/reactions equipment/equipment; do name=${pg/\//-};
  AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$pg" >/dev/null 2>&1
  AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
  AB "$S" eval --stdin < "$D/r57-reg-probe.js" > "$D/data/reg-$name.json" 2>"$D/data/regerr-$name.txt";
  echo "REG $name done"; done
echo REG_FINISH