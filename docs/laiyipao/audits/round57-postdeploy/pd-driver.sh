#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-pd
D=docs/laiyipao/audits/round57-postdeploy
INIT="$D/data/auth-init.js"
BASE="http://127.0.0.1:5201/#/pages"
PROBE="$D/pd-probe.js"
mkdir -p "$D/data" "$D/screenshots"
cp docs/laiyipao/audits/round57-visual-truncation/data/auth-init.js "$INIT"
echo PD_START=$(date +%s)
# 主门禁768: announcements + enemies
for pg in announcements/announcements enemies/enemies; do
  name=${pg/\//-}
  AB "$S" open --init-script "$INIT" "$BASE/$pg" >/dev/null 2>&1
  AB "$S" set viewport 768 1024 >/dev/null 2>&1
  sleep 2
  AB "$S" eval --stdin < "$PROBE" > "$D/data/pd-768-$name.json" 2>"$D/data/pderr-768-$name.txt"
  AB "$S" screenshot "$D/screenshots/pd-768-$name.png" >/dev/null 2>&1
  echo "DONE 768 $name"
done
# 新图互异交叉视口: 414/820/1440
VW=(414 820 1440); VH=(896 1180 900)
for i in 0 1 2; do W=${VW[$i]}; H=${VH[$i]};
  for pg in announcements/announcements enemies/enemies; do name=${pg/\//-};
    AB "$S" open --init-script "$INIT" "$BASE/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1; sleep 2
    AB "$S" eval --stdin < "$PROBE" > "$D/data/pd-$W-$name.json" 2>"$D/data/pderr-$W-$name.txt";
    AB "$S" screenshot "$D/screenshots/pd-$W-$name.png" >/dev/null 2>&1;
    echo "DONE $W $name"; done; done
# 回归锚点四页
for pg in reactions/reactions equipment/equipment me/me index/index; do name=${pg/\//-};
  AB "$S" open --init-script "$INIT" "$BASE/$pg" >/dev/null 2>&1
  AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
  AB "$S" eval --stdin < "$PROBE" > "$D/data/pd-reg-$name.json" 2>"$D/data/pdregerr-$name.txt";
  echo "DONE reg $name"; done
echo PD_FINISH=$(date +%s)