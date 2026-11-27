#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r95ax
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5174/" >/dev/null 2>&1; sleep 3
AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
echo '--- AX SNAPSHOT AT ME (before back) ---' > data/ax-768.txt
AB "$S" snapshot >> data/ax-768.txt 2>&1
AB "$S" close --all >/dev/null 2>&1 || true
echo AX_DONE >> data/ax-768.txt