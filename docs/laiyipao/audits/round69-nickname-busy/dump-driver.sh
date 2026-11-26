#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round69-nickname-busy"; D="$P/data"
function AB(){ agent-browser --session "$1" "${@:2}"; }
S=r69dump
AB "$S" open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
sleep 5
AB "$S" eval --stdin < "$P/r69-open-edit.js"
AB "$S" eval --stdin < "$P/data/dump-dom.js"
AB "$S" close >/dev/null 2>&1 || true
echo DUMP_DONE
