#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r95fin
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5174/" >/dev/null 2>&1; sleep 3
AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
echo '--- AT ME (gcp=2) ---' > data/final-768.txt; AB "$S" eval --stdin < data/r95-probe.js >> data/final-768.txt 2>&1
echo '--- TAB1 -> focus back-btn ---' >> data/final-768.txt; AB "$S" press Tab >/dev/null 2>&1; AB "$S" eval --stdin < data/r95-probe-active.js >> data/final-768.txt 2>&1
echo '--- ENTER on focused back-btn ---' >> data/final-768.txt; AB "$S" press Enter >/dev/null 2>&1; sleep 3; AB "$S" eval --stdin < data/r95-probe.js >> data/final-768.txt 2>&1
echo '--- RE-NAV ME ---' >> data/final-768.txt; AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
echo '--- REAL CLICK .back-btn ---' >> data/final-768.txt; AB "$S" click '.back-btn' >/dev/null 2>&1; echo CLICK_RC=$? >> data/final-768.txt; sleep 3; AB "$S" eval --stdin < data/r95-probe.js >> data/final-768.txt 2>&1
echo '--- AX SNAPSHOT (role button) ---' >> data/final-768.txt; AB "$S" snapshot >> data/final-768.txt 2>&1
AB "$S" close --all >/dev/null 2>&1 || true
echo FINAL_DONE >> data/final-768.txt