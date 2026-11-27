#!/usr/bin/env bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB(){ local S="$1"; shift; agent-browser --session "$S" "$@"; }
BASE='http://127.0.0.1:5174/'
D='docs/laiyipao/audits/round96-empty-cta-keyboard/data'
{
 for P in bag enemies me rank skins; do
   S="r96-recon-$P"
   AB "$S" close --all >/dev/null 2>&1 || true
   AB "$S" open "$BASE" >/dev/null 2>&1
   AB "$S" set viewport 768 1024 >/dev/null 2>&1
   AB "$S" navigate "$BASE#/pages/$P/$P" >/dev/null 2>&1
   sleep 4
   echo "=== $P ==="
   AB "$S" eval --stdin < "$D/probe-cta.js" 2>&1
   AB "$S" close --all >/dev/null 2>&1 || true
 done
}
> "$D/recon5-out.txt" 2>&1
echo RECON5_DONE
cat "$D/recon5-out.txt"