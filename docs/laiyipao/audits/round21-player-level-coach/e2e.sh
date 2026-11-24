#!/bin/bash
D=docs/laiyipao/audits/round21-player-level-coach
AB=agent-browser
URL="http://127.0.0.1:5174/#/pages/stage/stage"
wiki(){
  local Q_UID="$1" SESSION="$2" SUFFIX="$3"
  bash "$D/mint-token.sh" "$Q_UID" >/dev/null || return 1
  "$AB" close --session "$SESSION" >/dev/null 2>&1
  "$AB" open about:blank --session "$SESSION" >/dev/null 2>&1
  "$AB" set viewport 390 844 --session "$SESSION" >/dev/null 2>&1
  "$AB" open "$URL" --session "$SESSION" --init-script "$D/auth-init.js" >/dev/null 2>&1
  sleep 3
  "$AB" reload --session "$SESSION" >/dev/null 2>&1
  sleep 3
  "$AB" eval --session "$SESSION" --stdin < "$D/probe.js" > "$D/data/dom-${SUFFIX}.json" 2>&1
  "$AB" screenshot --session "$SESSION" "$D/screenshots/r21-${SUFFIX}.png" >/dev/null 2>&1
  echo "SHOT ${SUFFIX} done"
}
wiki 100 lyp-r21-data data
wiki 11 lyp-r21-empty empty
echo E2E_DONE