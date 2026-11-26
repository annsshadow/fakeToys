#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r55-baginput
O=docs/laiyipao/audits/round55-visual-readability
AB "$S" open >/dev/null 2>&1
for W in 390 768 1280; do
  H=844; [ "$W" = 768 ] && H=1024; [ "$W" = 1280 ] && H=800
  AB "$S" open --init-script "$O/data/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  sleep 3
  AB "$S" eval --stdin < "$O/data/bag-input-probe.js" > "$O/data/baginput-$W.json" 2>"$O/data/baginput-$W-err.txt"
  echo BAG_INPUT_DONE_$W
done
