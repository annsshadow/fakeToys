#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32-cta; V=390
D=docs/laiyipao/audits/round32-wiki-skill-mechanics; E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
printf '%s' '{"hashBefore":location.hash,"ctaText":document.querySelector(".r32-ai-cta a")?.textContent.trim(),"ctaHref":document.querySelector(".r32-ai-cta a")?.getAttribute("href"),"ctaRect":(()=>{const r=document.querySelector(".r32-ai-cta a")?.getBoundingClientRect();return r?{l:Math.round(r.left),t:Math.round(r.top),w:Math.round(r.width),h:Math.round(r.height)}:null})()}' > "$E/data/cta-pre-eval.txt"
"$AB" --session "$S" eval --stdin < "$E/data/cta-pre-eval.txt" > "$E/data/cta-pre.json" 2>&1
echo '=== CLI CLICK CTA ==='
"$AB" --session "$S" click '.r32-ai-cta a' > "$E/data/cta-click.txt" 2>&1; echo CLICK_RC=$?
sleep 2
printf '%s' '{"hashAfter":location.hash,"atComposite":!!document.querySelector("#composite"),"compositeRect":(()=>{const r=document.querySelector("#composite")?.getBoundingClientRect();return r?{t:Math.round(r.top)}:null})(),"skillStillThere":!!document.querySelector("#skill-table")}' > "$E/data/cta-post-eval.txt"
"$AB" --session "$S" eval --stdin < "$E/data/cta-post-eval.txt" > "$E/data/cta-post.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r32-04-cta-composite.png" >/dev/null 2>&1; echo SHOT_CTA
echo '=== SHA ==='; sha256sum "$E/screenshots/r32-04-cta-composite.png"