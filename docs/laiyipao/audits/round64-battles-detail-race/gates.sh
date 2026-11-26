#!/usr/bin/env bash
set -uo pipefail
P="laiyipao/miniapp"
echo "=== CONTRACT R64 ==="
cd "$P" && npx vitest run src/styles/r64BattlesRace.contract.test.ts --reporter=verbose > /tmp/g1.txt 2>&1; C1=$?; tail -8 /tmp/g1.txt; echo "RC=$C1"
echo "=== FULL VITEST ==="
cd "$P" && pnpm test > /tmp/g2.txt 2>&1; C2=$?; tail -6 /tmp/g2.txt; echo "RC=$C2"
echo "=== TYPECHECK ==="
cd "$P" && pnpm exec vue-tsc --noEmit > /tmp/g3.txt 2>&1; C3=$?; tail -6 /tmp/g3.txt; echo "RC=$C3"
echo "=== BUILD H5 ==="
cd "$P" && pnpm run build:h5 > /tmp/g4.txt 2>&1; C4=$?; tail -5 /tmp/g4.txt; echo "RC=$C4"
echo "=== DIFF CHECK ==="
cd "$PWD/../.." && git diff --check > /tmp/g5.txt 2>&1; C5=$?; cat /tmp/g5.txt | head -10; echo "RC=$C5"
echo "SUMMARY contract=$C1 full=$C2 type=$C3 build=$C4 diff=$C5"
