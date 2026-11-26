#!/bin/bash
# R40: visual screenshots with the rating section scrolled into view (DOM probes remain primary evidence).
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round40-wiki-rating-weights
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating"
run_shot() {
  VP=$1; W=$2; H=$3; S=lyp-r40-shot-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval "(() => { const s=document.querySelector('.r40-rating-mechanics'); if(!s) return JSON.stringify({ok:false}); s.scrollIntoView({block:'center'}); return JSON.stringify({ok:true,rows:s.querySelectorAll('tbody tr').length,hash:location.hash,docW:document.documentElement.scrollWidth,innerW:innerWidth}); })()" > "$E/data/shot-$VP.json" 2>&1
  sleep 1
  "$AB" --session "$S" screenshot "$E/screenshots/r40-$VP-rating-visible.png" >/dev/null 2>&1
  echo "=== SHOT $VP ==="; cat "$E/data/shot-$VP.json"; echo
  "$AB" --session "$S" close >/dev/null 2>&1 || true
}
run_shot 390 390 844
run_shot 768 768 1024
run_shot 1280 1280 800
echo SHOTS_DONE=YES
