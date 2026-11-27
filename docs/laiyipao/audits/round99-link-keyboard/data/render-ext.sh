#!/bin/bash
S=r99ext; AB(){ agent-browser --session "$S" "$@"; }
D="docs/laiyipao/audits/round99-link-keyboard"
AB close --all >/dev/null 2>&1
AB set viewport 768 1024 >/dev/null 2>&1
AB open --init-script "$(pwd)/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/announcements/announcements" >/dev/null 2>&1
sleep 5; AB set viewport 768 1024 >/dev/null 2>&1; sleep 1
AB fill 'input[aria-label="搜索公告标题 / 正文"]' NO_SUCH_zzz >/dev/null 2>&1; sleep 1
echo "=== ANN AFTER FILL ==="; AB eval --stdin < "$D/data/r99-probe.js" 2>&1|tail -1
AB open --init-script "$(pwd)/$D/data/r99-init.js" "http://127.0.0.1:5174/#/pages/me/me" >/dev/null 2>&1
sleep 5; AB set viewport 768 1024 >/dev/null 2>&1; sleep 1
AB eval --stdin < "$D/data/click-records.js" >/dev/null 2>&1; sleep 2
echo "=== ME RECORDS ==="; AB eval --stdin < "$D/data/r99-probe.js" 2>&1|tail -1
echo EXT_DONE
