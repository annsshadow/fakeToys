#!/usr/bin/env bash
set -uo pipefail
cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
PD="docs/laiyipao/audits/round72-search-empty-overflow-postdeploy"
QA="docs/laiyipao/audits/round72-search-empty-overflow"
D="$PD/data"; S="$PD/screenshots"; QD="$QA/data"
BASE="http://127.0.0.1:5201/#/pages"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_one(){
  local PG="$1" KEY="$2" V="$3" W="$4" H="$5"
  local SS="r72pd-${KEY}-${V}"
  AB "$SS" close >/dev/null 2>&1 || true
  AB "$SS" open --init-script "$D/auth-init.js" >/dev/null 2>&1
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "$BASE/$PG" >/dev/null 2>&1
  AB "$SS" wait 3500 >/dev/null 2>&1
  AB "$SS" eval --stdin < "$QD/inject-search.js" > "$D/pd-inj-${KEY}-${V}.txt" 2>&1
  AB "$SS" wait 800 >/dev/null 2>&1
  AB "$SS" eval --stdin < "$PD/pd-probe.js" > "$D/pd-${KEY}-${V}.txt" 2>&1
  AB "$SS" screenshot "$S/pd-${KEY}-${V}.png" > "$D/pd-shot-${KEY}-${V}.txt" 2>&1
  AB "$SS" eval --stdin < "$QD/clear-search.js" > "$D/pd-clear-${KEY}-${V}.txt" 2>&1
  AB "$SS" wait 600 >/dev/null 2>&1
  AB "$SS" eval --stdin < "$PD/pd-probe.js" > "$D/pd-cleared-${KEY}-${V}.txt" 2>&1
  AB "$SS" close >/dev/null 2>&1 || true
  echo "PD_DONE ${KEY} ${V}"
}
for V in 414 820 1440; do
  case "$V" in 414) HH=896;; 820) HH=1180;; 1440) HH=900;; esac
  for PG in skills/skills skins/skins gems/gems; do
    KEY=$(echo "$PG" | tr "/" "_")
    run_one "$PG" "$KEY" "$V" "$V" "$HH"
  done
done
echo PD_DONE_ALL
