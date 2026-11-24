#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32p-cta
D=docs/laiyipao/audits/round32-postfix-recheck; E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
printf '%s' 'JSON.stringify({hashBefore:location.hash,ctaText:document.querySelector(".r32-ai-cta a")?.textContent.trim(),ctaHref:document.querySelector(".r32-ai-cta a")?.getAttribute("href"),ctaRect:(()=>{const r=document.querySelector(".r32-ai-cta a")?.getBoundingClientRect();return r?{l:Math.round(r.left),t:Math.round(r.top),w:Math.round(r.width),h:Math.round(r.height)}:null})(),skillTableBefore:!!document.querySelector("#skill-table")})' > "$E/data/cta-pre-eval.txt"
"$AB" --session "$S" eval --stdin < "$E/data/cta-pre-eval.txt" > "$E/data/cta-pre.json" 2>&1
echo '=== CLI CLICK CTA ==='
"$AB" --session "$S" screenshot "$E/screenshots/r32p-04-cta-before.png" >/dev/null 2>&1
"$AB" --session "$S" click '.r32-ai-cta a' > "$E/data/cta-click.txt" 2>&1; echo CLICK_RC=$?
sleep 2
printf '%s' 'JSON.stringify({hashAfter:location.hash,skillStillThere:!!document.querySelector("#skill-table"),atComposite:!!document.querySelector("#composite"),compositeText:(()=>{const e=document.querySelector("#composite");return e?e.textContent.trim().slice(0,40):""})(),compositeRect:(()=>{const r=document.querySelector("#composite")?.getBoundingClientRect();return r?{t:Math.round(r.top),l:Math.round(r.left),w:Math.round(r.width)}:null})(),scrollY:Math.round(scrollY),dashboard:!!document.querySelector(".dashboard")||location.hash.includes("dashboard")})' > "$E/data/cta-post-eval.txt"
"$AB" --session "$S" eval --stdin < "$E/data/cta-post-eval.txt" > "$E/data/cta-post.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r32p-05-cta-after.png" >/dev/null 2>&1
sha256sum "$E/screenshots/r32p-04-cta-before.png" "$E/screenshots/r32p-05-cta-after.png"
echo CTA_DONE=YES
