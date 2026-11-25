#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r34-cta
D=docs/laiyipao/audits/round34-wiki-level-mechanics
E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$E/cta-pre.js" > "$E/data/cta-pre.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r34-04-cta-before.png" >/dev/null 2>&1
echo "=== CLI CLICK R34 CTA ==="
"$AB" --session "$S" click '[data-r34-cta]' > "$E/data/cta-click.txt" 2>&1; echo CLICK_RC=$?
sleep 2
"$AB" --session "$S" eval --stdin < "$E/cta-post.js" > "$E/data/cta-post.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r34-05-cta-after.png" >/dev/null 2>&1
sha256sum "$E/screenshots/r34-04-cta-before.png" "$E/screenshots/r34-05-cta-after.png"
echo CTA_DONE=YES
