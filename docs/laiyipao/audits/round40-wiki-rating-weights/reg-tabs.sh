#!/bin/bash
# R40: regression - click through every wiki tab via CLI and capture table row counts.
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser; S=lyp-r40-reg
D=docs/laiyipao/audits/round40-wiki-rating-weights
E="$PWD/$D"
click_tab() {
  KEY=$1; SEL=$2
  "$AB" --session "$S" click "$SEL" > "$E/data/reg-click-$KEY.txt" 2>&1
  echo "CLICK_$KEY_RC=$?"
  sleep 3
  "$AB" --session "$S" eval --stdin < "$E/reg-probe.js" > "$E/data/reg-$KEY.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r40-08-reg-$KEY.png" >/dev/null 2>&1
  echo "=== REG $KEY ==="; head -c 300 "$E/data/reg-$KEY.json"; echo
}
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "http://127.0.0.1:5175/laiyipao-admin/#/wiki" >/dev/null 2>&1; sleep 6
click_tab basics '.nav-item[href="#/wiki"]'
click_tab skills '.nav-item[href="#/wiki?tab=skills"]'
click_tab composite '.nav-sub-item[href="#/wiki?tab=skills&anchor=composite"]'
click_tab enemies '.nav-item[href="#/wiki?tab=enemies"]'
click_tab levels '.nav-item[href="#/wiki?tab=levels"]'
click_tab gems '.nav-item[href="#/wiki?tab=gems"]'
click_tab mastery '.nav-item[href="#/wiki?tab=mastery"]'
click_tab skins '.nav-item[href="#/wiki?tab=skins"]'
click_tab rules '.nav-item[href="#/wiki?tab=rules"]'
echo REG_DONE=YES
