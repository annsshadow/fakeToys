#!/bin/bash
# R39 postdeploy: fresh real-Chromium interaction screenshots distinct from pre-deploy QA set.
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round39-postdeploy
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skins"
new_session() {
  S=$1; W=$2; H=$3
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1
  sleep 1
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1
  sleep 5
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" reload >/dev/null 2>&1
  sleep 5
}
shot() { "$AB" --session "$1" screenshot "$2" >/dev/null 2>&1; echo "SHOT $(basename "$2")"; }

new_session r39f390 390 844
"$AB" --session r39f390 eval "var e=document.querySelector('.el-scrollbar__wrap'); if(e)e.scrollLeft=220; 'scrolled='+(e?e.scrollLeft:'none')" > "$E/data/fresh-390-scroll.txt" 2>&1
sleep 1
shot r39f390 "$E/screenshots/r39p-fresh-390-scrolled.png"

new_session r39f768 768 1024
"$AB" --session r39f768 eval "var inp=document.querySelector('input'); if(inp)inp.focus(); 'focused'" > "$E/data/fresh-768-focus.txt" 2>&1
sleep 1
shot r39f768 "$E/screenshots/r39p-fresh-768-focus.png"

new_session r39f1280 1280 800
"$AB" --session r39f1280 click '.nav-item[href="#/wiki?tab=rules"]' >/dev/null 2>&1
sleep 2
"$AB" --session r39f1280 eval "window.scrollTo(0,240); 'y='+window.scrollY" > "$E/data/fresh-1280-scroll.txt" 2>&1
sleep 1
shot r39f1280 "$E/screenshots/r39p-fresh-1280-rules-scrolled.png"

new_session r39f390b 390 844
"$AB" --session r39f390b click '.nav-item[href="#/wiki?tab=rules"]' >/dev/null 2>&1
sleep 2
"$AB" --session r39f390b eval "window.scrollTo(0,180); 'y='+window.scrollY" > "$E/data/fresh-390b-scroll.txt" 2>&1
sleep 1
shot r39f390b "$E/screenshots/r39p-fresh-390-rules-scrolled.png"

new_session r39f768b 768 1024
"$AB" --session r39f768b click '.nav-item[href="#/wiki?tab=rules"]' >/dev/null 2>&1
sleep 2
"$AB" --session r39f768b eval "window.scrollTo(0,120); 'y='+window.scrollY" > "$E/data/fresh-768b-scroll.txt" 2>&1
sleep 1
shot r39f768b "$E/screenshots/r39p-fresh-768-rules-scrolled.png"

new_session r39f1280b 1280 800
"$AB" --session r39f1280b eval "var rows=document.querySelectorAll('.el-table__row'); if(rows[2])rows[2].scrollIntoView(); 'rows='+rows.length" > "$E/data/fresh-1280b.txt" 2>&1
sleep 1
shot r39f1280b "$E/screenshots/r39p-fresh-1280-skins-row.png"

for s in r39f390 r39f768 r39f1280 r39f390b r39f768b r39f1280b; do "$AB" --session "$s" close >/dev/null 2>&1 || true; done
echo RESHOOT_DONE=YES
