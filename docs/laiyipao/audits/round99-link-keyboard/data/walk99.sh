#!/bin/bash
S=r99walk; AB(){ agent-browser --session "$S" "$@"; }
D="docs/laiyipao/audits/round99-link-keyboard"
ROOT="$(pwd)"
AB close --all >/dev/null 2>&1
for V in 390 768 1280; do
  H=844; [ "$V" = 768 ] && H=1024; [ "$V" = 1280 ] && H=800
  for PG in bag me; do
    AB set viewport $V $H >/dev/null 2>&1
    AB open --init-script "$ROOT/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/$PG/$PG" >/dev/null 2>&1
    sleep 5; AB set viewport $V $H >/dev/null 2>&1; sleep 1
    echo "=== $PG-$V ATTR ==="
    AB eval --stdin < "$D/data/r99-probe.js" 2>&1 | tail -1
    AB eval --stdin < "$D/data/enter-observer.js" 2>&1 | tail -1
    echo "--- AX ---"
    AB snapshot -i 2>&1 | grep -i button | head -20
    AB screenshot "$ROOT/$D/screenshots/r99-$PG-$V.png" >/dev/null 2>&1
    echo "--- FOCUS/ENTER ---"
    for i in $(seq 1 14); do
      AB press Tab >/dev/null 2>&1
      HIT=$(AB eval --stdin < "$D/data/probe-hit.js" 2>&1 | tail -1 | tr -d '"')
      if [[ "$HIT" == CTA_HIT* ]]; then
        echo "TAB$i $HIT"
        AB eval --stdin < "$D/data/probe-focusring.js" 2>&1 | tail -1
        AB press Enter >/dev/null 2>&1; sleep 1
        AB eval --stdin < "$D/data/probe-actions.js" 2>&1 | tail -1
      else
        echo "TAB$i NO"
      fi
    done
  done
done
echo WALK_DONE
