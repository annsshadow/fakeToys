#!/bin/bash
S=r99diag2; AB(){ agent-browser --session "$S" "$@"; }
D="docs/laiyipao/audits/round99-link-keyboard"
AB close --all >/dev/null 2>&1
for PG in index me; do
 AB set viewport 768 1024 >/dev/null 2>&1
 AB open --init-script "$(pwd)/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/$PG/$PG" >/dev/null 2>&1
 sleep 5; AB set viewport 768 1024 >/dev/null 2>&1; sleep 1
 echo "=== $PG DUMP ==="; AB eval --stdin < "$D/data/r99-dump.js" 2>&1|tail -1
 echo "=== $PG HITS ==="; AB eval --stdin < "$D/data/r99-hits.js" 2>&1|tail -1
done
echo DIAG2_DONE
