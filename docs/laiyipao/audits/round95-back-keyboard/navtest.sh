#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r95nav2
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5174/" >/dev/null 2>&1
sleep 3
echo '--- BEFORE ---' > data/navtest.txt
AB "$S" eval --stdin < data/r95-probe.js >> data/navtest.txt 2>&1
echo '--- NAV CALL ---' >> data/navtest.txt
AB "$S" eval --stdin < data/r95-nav-me.js >> data/navtest.txt 2>&1
sleep 3
echo '--- AFTER ---' >> data/navtest.txt
AB "$S" eval --stdin < data/r95-probe.js >> data/navtest.txt 2>&1
echo NAVTEST_DONE >> data/navtest.txt