#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"; DATA="$E/data"
echo "=== deeplink2 ==="; cat "$DATA/reg-deeplink2.json" 2>&1
echo "=== focus ==="; timeout 25 agent-browser --session lyp-r32p-reg-all focus 'input[placeholder*="搜索关卡"]' 2>&1 | head -n 3; echo FOCUS_RC=$?
echo "=== keyboard type ==="; timeout 25 agent-browser --session lyp-r32p-reg-all keyboard type '温压弹' 2>&1 | head -n 3; echo TYPE_RC=$?
echo "=== press Enter ==="; timeout 25 agent-browser --session lyp-r32p-reg-all press Enter 2>&1 | head -n 3; echo ENTER_RC=$?
sleep 2
printf "%s" 'JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),inputValue:document.querySelector("input[placeholder]")?.value,hasWenya:document.body.innerText.includes("温压弹")})' > "$DATA/reg-search2.txt"
agent-browser --session lyp-r32p-reg-all eval --stdin < "$DATA/reg-search2.txt" > "$DATA/reg-search2.json" 2>&1; echo PROBE_RC=$?
cat "$DATA/reg-search2.json"
agent-browser --session lyp-r32p-reg-all screenshot "$E/screenshots/r32p-07-search.png" >/dev/null 2>&1; echo SEARCH_DONE=YES
