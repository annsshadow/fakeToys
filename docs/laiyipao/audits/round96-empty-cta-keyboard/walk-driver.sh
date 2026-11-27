#!/usr/bin/env bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB(){ local S="$1"; shift; agent-browser --session "$S" "$@"; }
BASE='http://127.0.0.1:5174/'
D='docs/laiyipao/audits/round96-empty-cta-keyboard/data'
SH='docs/laiyipao/audits/round96-empty-cta-keyboard/screenshots'
mkdir -p "$D" "$SH"
focus_cta(){
  local S="$1" r i
  for i in $(seq 1 12); do
    AB "$S" press Tab >/dev/null 2>&1
    r=$(AB "$S" eval --stdin < "$D/probe-hit.js" 2>&1)
    case "$r" in *CTA_HIT*) echo "CTA_HIT_AT=$i"; return 0;; esac
  done
  echo "CTA_NOT_HIT"; return 1
}
for V in 390 768 1280; do
  case "$V" in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  for P in bag battles enemies me rank skins; do
    S="r96-w-$V-$P"
    {
      AB "$S" close --all >/dev/null 2>&1 || true
      AB "$S" open "$BASE" >/dev/null 2>&1
      AB "$S" evaluate --stdin < "$D/enter-observer.js" >/dev/null 2>&1 || AB "$S" eval --stdin < "$D/enter-observer.js" >/dev/null 2>&1
      AB "$S" set viewport "$V" "$H" >/dev/null 2>&1
      AB "$S" navigate "$BASE#/pages/$P/$P" >/dev/null 2>&1
      sleep 4
      echo "=== $P $V START ==="
      echo "--- PROBE0 ---"; AB "$S" eval --stdin < "$D/probe-cta.js" 2>&1
      echo "--- AX ---"; AB "$S" snapshot 2>&1 | grep -iE "button" | head -6
      echo "--- TAB_FOCUS ---"; focus_cta "$S"
      echo "--- ACTIVE ---"; AB "$S" eval --stdin < "$D/probe-active.js" 2>&1
      echo "--- FOCUS_RING ---"; AB "$S" eval --stdin < "$D/probe-focusring.js" 2>&1
      echo "--- SCREENSHOT ---"; AB "$S" screenshot "$(pwd)/$SH/r96-$P-$V.png" >/dev/null 2>&1; echo SHOT_RC=$?
      echo "--- BEFORE_ENTER ---"; AB "$S" eval --stdin < "$D/after-probe.js" 2>&1
      echo "--- ENTER ---"; AB "$S" press Enter >/dev/null 2>&1; sleep 2
      echo "--- AFTER_ENTER ---"; AB "$S" eval --stdin < "$D/after-probe.js" 2>&1
      AB "$S" close --all >/dev/null 2>&1 || true
      echo "=== $P $V DONE ==="
    } > "$D/w-$P-$V.log" 2>&1
  done
done
echo R96_WALK_ALL_DONE