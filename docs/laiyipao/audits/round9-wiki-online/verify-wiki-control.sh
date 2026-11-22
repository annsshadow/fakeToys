#!/bin/bash
KEYS='D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys'
DOMAIN=$(grep '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//' | tr -d '\r' | sed 's/^https\?:\/\///' | sed 's/\/.*$//')
ADMIN_PASS=$(grep '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//' | tr -d '\r')
mask(){ sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g; s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9wiki-ctrl; BASE="https://$DOMAIN/laiyipao-admin/"
D='docs/laiyipao/audits/round9-wiki-online'
SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-wiki-online/screenshots'
raw(){ "$AB" --session "$S" "$@" 2>&1; }
run(){ raw "$@" | mask; }
j(){ raw eval --stdin < "$D/$1" 2>&1 | head -1 | mask; }
echo '=== CONTROL: reproduce t94 simplified open-hash method ==='
run open "$BASE" >/dev/null; sleep 4
raw eval "(function(){try{localStorage.clear()}catch(e){};return 1})" >/dev/null
run open "$BASE#/admin/login" >/dev/null; sleep 4
raw fill "input[placeholder='admin']" 'admin' >/dev/null
raw fill "input[placeholder='至少 10 位']" "$ADMIN_PASS" >/dev/null
j login-click.js >/dev/null; sleep 6
echo "CTRL_AFTER_LOGIN: $(j wiki-probe.js)"
echo '--- t94 method: open same-document hash #/wiki (no nav click) ---'
run open "$BASE#/wiki" >/dev/null; sleep 3
echo "CTRL_OPEN_HASH: $(j wiki-probe.js)"
run screenshot "$SHOT/wiki-C-04-open-hash-simplified.png" >/dev/null
echo '--- then native nav click (same session, from wherever) ---'
echo "CTRL_LINK: $(j check-link.js)"
j wiki-nav-click.js >/dev/null; sleep 3
echo "CTRL_AFTER_CLICK: $(j wiki-probe.js)"
echo 'CTRL_DONE'