#!/bin/bash
KEYS='D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys'
DOMAIN=$(grep '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//' | tr -d '\r' | sed 's/^https\?:\/\///' | sed 's/\/.*$//')
mask(){ sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9wiki-online; BASE="https://$DOMAIN/laiyipao-admin/"
D='docs/laiyipao/audits/round9-wiki-online'
SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-wiki-online/screenshots'
raw(){ "$AB" --session "$S" "$@" 2>&1; }
run(){ raw "$@" | mask; }
j(){ raw eval --stdin < "$D/$1" 2>&1 | head -1 | mask; }
echo '=== PATH A v2: logged-in dashboard -> sidebar wiki-link ==='
run open "$BASE#/admin/dashboard" >/dev/null; sleep 5
echo "A_DASH: $(j wiki-probe.js)"
echo "A_LINK_EXISTS: $(j check-link.js)"
echo "A_CLICK: $(j wiki-nav-click.js)"
sleep 4
echo "A_LANDING: $(j wiki-probe.js)"
run screenshot "$SHOT/wiki-A-03-sidebar-nav.png" >/dev/null
echo "A_ERRORS: $(raw errors --json | mask | head -c 200)"
echo 'PATHA_DONE'