#!/bin/bash
# R39: real CLI click on the skin AI CTA anchor.
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r39p-cta
D=docs/laiyipao/audits/round39-postdeploy
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skins"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$E/cta-pre.js" > "$E/data/cta-pre.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r39p-04-cta-before.png" >/dev/null 2>&1
"$AB" --session "$S" click ".r39p-ai-note a" > "$E/data/cta-click.txt" 2>&1
echo "CTA_CLICK_RC=$?"
sleep 2
"$AB" --session "$S" eval --stdin < "$E/cta-post.js" > "$E/data/cta-post.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r39p-05-cta-after.png" >/dev/null 2>&1
echo "=== CTA PRE ==="; head -c 300 "$E/data/cta-pre.json"; echo
echo "=== CTA POST ==="; head -c 300 "$E/data/cta-post.json"; echo
echo CTA_DONE=YES
