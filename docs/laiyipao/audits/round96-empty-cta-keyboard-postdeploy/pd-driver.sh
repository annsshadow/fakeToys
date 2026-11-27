set -uo pipefail
cd "$(dirname "$0")/../../../.." || exit 1
AB(){ local S="$1"; shift; agent-browser --session "$S" "$@"; }
BASE='http://127.0.0.1:5201/'
D='docs/laiyipao/audits/round96-empty-cta-keyboard-postdeploy/data'
SH='docs/laiyipao/audits/round96-empty-cta-keyboard-postdeploy/screenshots'
mkdir -p "$D" "$SH"
for V in 414 820 1440; do
  case "$V" in 414) H=896;; 820) H=1180;; 1440) H=900;; esac
  for P in battles me; do
    S="r96pd-$V-$P"
    {
      AB "$S" close --all >/dev/null 2>&1 || true
      AB "$S" open "$BASE" >/dev/null 2>&1
      AB "$S" set viewport "$V" "$H" >/dev/null 2>&1
      AB "$S" navigate "$BASE#/pages/$P/$P" >/dev/null 2>&1
      sleep 4
      AB "$S" eval --stdin < "$D/enter-observer.js" >/dev/null 2>&1 || true
      echo "=== $P $V START ==="
      echo "--- PROBE0 ---"; AB "$S" eval --stdin < "$D/probe-cta.js" 2>&1
      echo "--- AX ---"; AB "$S" snapshot 2>&1 | grep -i "button" | head -6
      echo "--- TAB_FOCUS ---"
      HIT=0
      for i in $(seq 1 12); do
        AB "$S" press Tab >/dev/null 2>&1
        r=$(AB "$S" eval --stdin < "$D/probe-hit.js" 2>&1)
        case "$r" in *CTA_HIT*) echo "CTA_HIT_AT=$i"; HIT=1; break;; esac
      done
      if [ "$HIT" -eq 0 ]; then echo CTA_NOT_HIT; fi
      echo "--- ACTIVE ---"; AB "$S" eval --stdin < "$D/probe-active.js" 2>&1
      echo "--- FOCUS_RING ---"; AB "$S" eval --stdin < "$D/probe-focusring.js" 2>&1
      echo "--- SCREENSHOT ---"; AB "$S" screenshot "$(pwd)/$SH/pd96-$V-$P.png" >/dev/null 2>&1; echo SHOT_RC=$?
      echo "--- BEFORE_ENTER ---"; AB "$S" eval --stdin < "$D/after-probe.js" 2>&1
      AB "$S" press Enter >/dev/null 2>&1; sleep 2
      echo "--- AFTER_ENTER ---"; AB "$S" eval --stdin < "$D/after-probe.js" 2>&1
      AB "$S" close --all >/dev/null 2>&1 || true
      echo "=== $P $V DONE ==="
    } > "$D/pd-$V-$P.log" 2>&1
  done
done
echo PD96_ALL_DONE
