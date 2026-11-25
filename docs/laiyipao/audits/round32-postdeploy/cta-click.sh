#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32post-cta
D=docs/laiyipao/audits/round32-postdeploy; E="$PWD/$D"; DATA="$E/data"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$DATA/cta-pre-eval.txt" > "$DATA/cta-pre.json" 2>&1
echo '=== PRE ==='; cat "$DATA/cta-pre.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32post-cta-before.png" >/dev/null 2>&1; echo BEFORE_SHOT_RC=$?
echo '=== CLI CLICK ==='
"$AB" --session "$S" click '.r32-ai-cta a' > "$DATA/cta-click.txt" 2>&1; echo CLICK_RC=$?
cat "$DATA/cta-click.txt"
sleep 2
"$AB" --session "$S" eval --stdin < "$DATA/cta-post-eval.txt" > "$DATA/cta-post.json" 2>&1
echo '=== POST ==='; cat "$DATA/cta-post.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32post-cta-after.png" > "$DATA/cta-shot.txt" 2>&1; echo SHOT_RC=$?
"$AB" --session "$S" close >/dev/null 2>&1 || true
sha256sum "$E/screenshots/r32post-cta-before.png" "$E/screenshots/r32post-cta-after.png" 2>&1
echo CTA_DONE=YES
