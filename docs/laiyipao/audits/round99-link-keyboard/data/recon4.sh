#!/bin/bash
S=r99rec; AB(){ agent-browser --session "$S" "$@"; }
D="docs/laiyipao/audits/round99-link-keyboard"
AB close --all >/dev/null 2>&1
for PG in bag index me announcements; do
  AB set viewport 768 1024 >/dev/null 2>&1
  AB open --init-script "$(pwd)/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/$PG/$PG" >/dev/null 2>&1
  sleep 4
  AB set viewport 768 1024 >/dev/null 2>&1
  sleep 1
  echo "=== $PG ==="
  AB eval --stdin < "$D/data/r99-probe.js" 2>&1 | tail -1
done
echo RECON_DONE
