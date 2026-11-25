#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r37-reg3
D=docs/laiyipao/audits/round37-wiki-mastery-nodes
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=mastery"
click_tab() {
  KEY=$1
  case $KEY in
    basics) SEL=".nav-item[href=\"#/wiki\"]" ;;
    skills) SEL=".nav-item[href=\"#/wiki?tab=skills\"]" ;;
    composite) SEL=".nav-sub-item[href=\"#/wiki?tab=skills&anchor=composite\"]" ;;
    enemies) SEL=".nav-item[href=\"#/wiki?tab=enemies\"]" ;;
    levels) SEL=".nav-item[href=\"#/wiki?tab=levels\"]" ;;
    gems) SEL=".nav-item[href=\"#/wiki?tab=gems\"]" ;;
    mastery) SEL=".nav-item[href=\"#/wiki?tab=mastery\"]" ;;
  esac
  echo "=== CLI CLICK TAB $KEY $SEL ==="
  "$AB" --session "$S" click "$SEL" > "$E/data/reg3-click-$KEY.txt" 2>&1; echo "CLICK_${KEY}_RC=$?"
  sleep 3
  "$AB" --session "$S" eval --stdin < "$E/reg3-probe.js" > "$E/data/reg3-$KEY.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r37-reg3-$KEY.png" >/dev/null 2>&1
}
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
for TAB in basics skills composite enemies levels gems mastery; do click_tab "$TAB"; done
echo REG3_DONE=YES