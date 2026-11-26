#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round60-a11y-keyboard"
SESS=lyp-r60-cta
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$SESS" open >/dev/null
for P in equipment index battles; do
  AB "$SESS" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null
  AB "$SESS" set viewport 768 1024 >/dev/null
  sleep 2
  AB "$SESS" eval --stdin < "$D/r60-cta-probe.js" > "$D/data/cta-$P.json" 2>"$D/data/cta-$P.err"
  echo "CTA $P done"
done
AB "$SESS" close >/dev/null 2>&1 || true
echo CTA_FINISH
