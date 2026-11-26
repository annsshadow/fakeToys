#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32-cta2
D=docs/laiyipao/audits/round32-wiki-skill-mechanics; E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 5
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 5
"$AB" --session "$S" eval --stdin < "$E/data/cta-pre-eval.txt" > "$E/data/cta2-pre.json" 2>&1
echo '=== CLI CLICK ==='
"$AB" --session "$S" click '.r32-ai-cta a' > "$E/data/cta2-click.txt" 2>&1; echo CLICK_RC=$?
sleep 2
"$AB" --session "$S" eval --stdin < "$E/data/cta-post-eval.txt" > "$E/data/cta2-post.json" 2>&1
timeout 20 "$AB" --session "$S" screenshot "$E/screenshots/r32-04-cta-composite.png" > "$E/data/cta2-shot.txt" 2>&1; echo SHOT_RC=$?
sha256sum "$E/screenshots/r32-04-cta-composite.png" 2>&1