#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round35-wiki-gem-mechanics; E="$PWD/$D"
BASE="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=gems"
shot() { W=$1; H=$2; WORD=$3; PNG=$4; S="lyp-r35p-shots-$PNG"
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$BASE" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" click 'input[type=search]' >/dev/null 2>&1; echo CLICK_${PNG}=$?
  "$AB" --session "$S" keyboard type "$WORD" >/dev/null 2>&1; echo TYPE_${PNG}=$?
  "$AB" --session "$S" press Enter >/dev/null 2>&1; echo ENTER_${PNG}=$?
  sleep 2
  "$AB" --session "$S" screenshot "$E/screenshots/$PNG" >/dev/null 2>&1
}
shot 768 1024 gold r35p-gold-768.png
shot 1280 800 gold r35p-gold-1280.png
shot 768 1024 锋锐石 r35p-feng-768.png
echo SHOTS_DONE=YES
