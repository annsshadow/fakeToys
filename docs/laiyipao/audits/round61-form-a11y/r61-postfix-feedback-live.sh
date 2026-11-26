#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; S=lyp-r61-pf-live
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
# ---- bag presetSaving ----
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "BAG_INPUTS=$(AB "$S" get count input 2>&1)"
echo "BAG_FILL=$(AB "$S" fill "input" "QA预设PF" 2>&1)"
echo "BAG_SAVE=$(AB "$S" click ".preset-save-row .link" 2>&1)"
for n in 1 2 3 4 5 6 7 8 9 10; do AB "$S" eval --stdin < "$ROOT/docs/laiyipao/audits/round61-form-a11y/pf-capture.js" > "$D/pf-fb-bag-live-$n.json" 2>"$D/pf-fb-bag-live-$n.err"; echo "POLL$n=$(cat "$D/pf-fb-bag-live-$n.json" | head -c 180)"; sleep 0.1; done
AB "$S" snapshot --json > "$D/pf-fb-ax-bag-live.json" 2>&1
# ---- me passwordMsg via upgrade form ----
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
REF=$(AB "$S" snapshot -i 2>&1 | grep "设置账号密码" | grep -o "@e[0-9]*" | head -1)
echo "ME_REF=$REF"
echo "ME_CLICK=$(AB "$S" click "$REF" 2>&1)"; sleep 1
echo "ME_AFTER=$(AB "$S" snapshot -i 2>&1 | head -30)"
AB "$S" snapshot --json > "$D/pf-me-upgrade-ax.json" 2>&1
echo "ME_INPUTS=$(AB "$S" get count input 2>&1)"
if [ "$ME_INPUTS" = "2" ]; then
  R1=$(AB "$S" snapshot -i 2>&1 | head -30)
  echo "ME_SNAP=$R1"
  echo "ME_FILL1=$(AB "$S" fill "uni-input input >> nth=0" "qa_pf_user" 2>&1)"
  echo "ME_FILL2=$(AB "$S" fill "uni-input input >> nth=1" "QaPfPass123456" 2>&1)"
  R2=$(AB "$S" snapshot -i 2>&1 | grep "确认升级" | grep -o "@e[0-9]*" | head -1)
  echo "ME_SUBMIT_REF=$R2"
  echo "ME_SUBMIT=$(AB "$S" click "$R2" 2>&1)"; sleep 3
  AB "$S" eval --stdin < "$ROOT/docs/laiyipao/audits/round61-form-a11y/pf-capture.js" > "$D/pf-fb-me-upgrade-live.json" 2>&1
  echo "ME_FB=$(cat "$D/pf-fb-me-upgrade-live.json" | head -c 300)"
  AB "$S" snapshot --json > "$D/pf-fb-ax-me-upgrade-live.json" 2>&1
fi
AB "$S" close >/dev/null 2>&1 || true
echo PF_LIVE_FINISH