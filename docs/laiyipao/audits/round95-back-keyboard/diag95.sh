#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r95diag
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5174/" >/dev/null 2>&1; sleep 3
AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
echo '--- AT ME ---' > data/diag.txt; AB "$S" eval --stdin < data/r95-probe.js >> data/diag.txt 2>&1
echo '--- TAB1 FOCUS ---' >> data/diag.txt; AB "$S" press Tab >/dev/null 2>&1; AB "$S" eval --stdin < data/r95-probe-active.js >> data/diag.txt 2>&1
echo '--- PRESS ENTER (focus on back) ---' >> data/diag.txt; AB "$S" press Enter >/dev/null 2>&1; sleep 2; AB "$S" eval --stdin < data/r95-probe.js >> data/diag.txt 2>&1
echo '--- RE-NAV ME ---' >> data/diag.txt; AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
echo '--- EVAL CLICK BACK ---' >> data/diag.txt; AB "$S" eval --stdin < data/r95-click-back.js >> data/diag.txt 2>&1; sleep 2; AB "$S" eval --stdin < data/r95-probe.js >> data/diag.txt 2>&1
echo DIAG_DONE >> data/diag.txt