#!/bin/bash
KEYS='D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys'
DOMAIN=$(grep '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//' | tr -d '\r' | sed 's/^https\?:\/\///' | sed 's/\/.*$//')
ADMIN_PASS=$(grep '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//' | tr -d '\r')
mask(){ sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g; s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9wiki-online; BASE="https://$DOMAIN/laiyipao-admin/"
D='docs/laiyipao/audits/round9-wiki-online'
SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-wiki-online/screenshots'
raw(){ "$AB" --session "$S" "$@" 2>&1; }
run(){ raw "$@" | mask; }
j(){ raw eval --stdin < "$D/$1" 2>&1 | head -1 | mask; }
echo '=== PATH B: no-token deep link /wiki -> login -> landing ==='
run open "$BASE" >/dev/null; sleep 4
echo "CLEAR: $(raw eval "(function(){try{localStorage.clear()}catch(e){};return 'CLEARED'})()" | mask | head -1)"
run open "$BASE#/wiki" >/dev/null; sleep 5
echo "B_AFTER_DEEPLINK: $(j wiki-probe.js)"
run screenshot "$SHOT/wiki-B-01-login-redirect.png" >/dev/null
raw fill "input[placeholder='admin']" 'admin' >/dev/null
raw fill "input[placeholder='至少 10 位']" "$ADMIN_PASS" >/dev/null
echo "B_LOGIN_CLICK: $(j login-click.js)"
sleep 6
echo "B_AFTER_LOGIN: $(j wiki-probe.js)"
run screenshot "$SHOT/wiki-B-02-after-login.png" >/dev/null
echo "B_ERRORS: $(raw errors --json | mask | head -c 400)"
echo '=== PATH A: logged-in sidebar wiki nav ==='
echo "A_CLICK_NAV: $(j wiki-nav-click.js)"
sleep 4
echo "A_AFTER_NAV: $(j wiki-probe.js)"
run screenshot "$SHOT/wiki-A-03-sidebar-nav.png" >/dev/null
echo 'WIKI_DONE'