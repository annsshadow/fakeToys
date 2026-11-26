#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-reg3
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
  $AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=basics' > "$DATA/reg3-open.txt" 2>&1
  $AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
  $AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=basics"]' > "$DATA/reg3-click-basics.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-basics.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=enemies"]' > "$DATA/reg3-click-enemies.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-enemies.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=skills"]' > "$DATA/reg3-click-skills.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-skills.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=gems"]' > "$DATA/reg3-click-gems.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-gems.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=equipment"]' > "$DATA/reg3-click-equipment.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-equipment.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=skins"]' > "$DATA/reg3-click-skins.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-skins.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=rules"]' > "$DATA/reg3-click-rules.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-rules.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=mastery"]' > "$DATA/reg3-click-mastery.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-mastery.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=levels"]' > "$DATA/reg3-click-levels.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-levels.json" 2>&1
$AB --session "$S" click 'a.nav-item[href="#/wiki?tab=live"]' > "$DATA/reg3-click-live.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/r41-reg-probe3.js" > "$DATA/reg3-live.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo REG3_DONE
