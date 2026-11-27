#!/bin/bash
AB=agent-browser
D=/d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/data
SH=/d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/screenshots
LOG=/d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/data/capture.log
URL=http://127.0.0.1:5174/#/pages
: > "$LOG"
emit() { echo "$1" | tee -a "$LOG"; }
capture() {
  local W=$1 H=$2 PG=$3
  local S="r99_${W}_${PG//\//_}"
  $AB close --all --session "$S" >/dev/null 2>&1
  $AB open "$URL/$PG/$PG" --session "$S" >/dev/null 2>&1
  $AB set viewport $W $H --session "$S" >/dev/null 2>&1
  $AB set media light reduced-motion --session "$S" >/dev/null 2>&1
  $AB wait 1200 --session "$S" >/dev/null 2>&1
  for i in $(seq 1 10); do
    $AB press Tab --session "$S" >/dev/null 2>&1
    local J=$($AB eval --stdin --session "$S" < "$D/probe.js" 2>/dev/null)
    emit "CAP|${W}x${H}|${PG}|${i}|${J}"
  done
  $AB press Tab --session "$S" >/dev/null 2>&1
  $AB screenshot "$SH/focus-${W}-${PG//\//_}.png" --session "$S" >/dev/null 2>&1
  $AB close --all --session "$S" >/dev/null 2>&1
}
for VP in "390 844" "768 1024" "1280 800"; do
  set -- $VP
  for PG in me signin verify announcements; do
    capture "$1" "$2" "$PG"
  done
done
python /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/data/parse.py < "$LOG" > /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round99-b-focus-visible/data/parsed.txt 2>&1
echo R99_FULL_DONE
