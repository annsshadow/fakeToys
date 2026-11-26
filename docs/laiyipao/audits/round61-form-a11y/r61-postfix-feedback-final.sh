#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y/pf-capture.js"; S=lyp-r61-pf-final
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
# ---- BAG presetSaving ----
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "FILL=$(AB "$S" fill "input" "QA预设PF" 2>&1)"
echo "SAVE=$(AB "$S" click ".preset-save-row uni-text.link" 2>&1)"
for n in 1 2 3 4 5 6 7 8; do AB "$S" eval --stdin < "$P" > "$D/pf-fb-bag-final-$n.json" 2>&1; echo "POLL$n=$(cat "$D/pf-fb-bag-final-$n.json" | head -c 200)"; sleep 0.08; done
AB "$S" snapshot --json > "$D/pf-fb-ax-bag-final.json" 2>&1
# ---- ME upgradeMsg as passwordMsg equivalent check + try password form ----
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "ME_CLICK=$(AB "$S" click ".card uni-button.btn >> nth=0" 2>&1)"; sleep 1
echo "ME_INPUTS=$(AB "$S" get count input 2>&1)"
AB "$S" eval --stdin < "$P" > "$D/pf-fb-me-form.json" 2>&1; echo "ME_FORM_FB=$(cat "$D/pf-fb-me-form.json" | head -c 250)"
echo "ME_F1=$(AB "$S" fill "input >> nth=0" "qa_pf_user" 2>&1)"
echo "ME_F2=$(AB "$S" fill "input >> nth=1" "QaPfPass123456" 2>&1)"
echo "ME_SUBMIT=$(AB "$S" click ".card .btn-primary >> nth=0" 2>&1)"; sleep 3
AB "$S" eval --stdin < "$P" > "$D/pf-fb-me-final.json" 2>&1; echo "ME_FB=$(cat "$D/pf-fb-me-final.json" | head -c 350)"
AB "$S" snapshot --json > "$D/pf-fb-ax-me-final.json" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo PF_FINAL_FINISH