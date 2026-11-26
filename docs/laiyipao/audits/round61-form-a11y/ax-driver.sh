#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-axb
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
for P in announcements login verify signin bag equipment enemies gems reactions skills skins; do
 AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null 2>&1
 AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
 AB "$S" snapshot > "$D/data/ax-$P-768.txt" 2>&1
 TB=$(grep -i "textbox" "$D/data/ax-$P-768.txt" | tr "\n" ";")
 echo "AX-$P: $TB"
done
AB "$S" close >/dev/null 2>&1 || true; echo AX_FINISH
