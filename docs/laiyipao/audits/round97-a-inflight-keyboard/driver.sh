#!/bin/bash
export AGENT_BROWSER_SESSION=lyp-r97a
B="timeout 25 agent-browser --session lyp-r97a"
P="docs/laiyipao/audits/round97-a-inflight-keyboard"
run_page() {
  local route="$1" name="$2"
  $B open "http://127.0.0.1:5175/#/pages/$route" >/dev/null 2>&1
  sleep 2
  for spec in "390 844" "768 1024" "1280 800"; do
    set -- $spec
    $B set viewport $1 $2 >/dev/null 2>&1
    echo "=== $name $1x$2 ==="
    $B get url 2>&1 | tail -1
    $B eval --stdin < "$P/probe-keys.js" 2>&1 | tail -2
    $B screenshot "$P/$name-$1.png" 2>&1 | tail -1
  done
}
run_page "verify/verify" verify
run_page "signin/signin" signin
echo "=== DRIVER_DONE ==="
