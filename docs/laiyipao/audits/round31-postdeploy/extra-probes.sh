#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
E="$PWD/docs/laiyipao/audits/round31-postdeploy"
S=lyp-r31-postdeploy-mobile390
"$AB" --session "$S" eval --stdin < "$E/wiki-scroll-probe.js" > "$E/data/scroll-lastcol.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r31post-04-deeplink.png" >/dev/null 2>&1; echo SHOT_04
echo SCROLL=$(cat "$E/data/scroll-lastcol.json")
