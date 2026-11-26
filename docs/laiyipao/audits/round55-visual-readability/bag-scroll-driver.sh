#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
O=docs/laiyipao/audits/round55-visual-readability
for W in 390 768 1280; do
  H=844; [ "$W" = 768 ] && H=1024; [ "$W" = 1280 ] && H=800
  S=lyp-r55-bagscroll-$W
  AB "$S" open >/dev/null 2>&1
  AB "$S" open --init-script "$O/data/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  sleep 3
  echo "document.querySelector('.preset-input').scrollIntoView({block:'center'}); 'BAG_SCROLLED'" | AB "$S" eval --stdin > "$O/data/bagscroll-$W.txt" 2>&1
  sleep 1
  AB "$S" screenshot "$O/screenshots/r55-$W-bag-input.png" >/dev/null 2>&1
  AB "$S" close >/dev/null 2>&1
  echo BAG_SCROLL_DONE_$W
done
echo R55_BAG_SCROLL_FINISH
