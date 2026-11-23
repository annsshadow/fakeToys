#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-mobile-audit'
echo "=== PAGE 11-gems ==="
"$AB" --session lyp-m11-11 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-11 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/gems/gems" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-11 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-11 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-11 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-11 screenshot "$D/screenshots/11-gems.png" 2>&1 | head -1
echo "=== PAGE 12-skins ==="
"$AB" --session lyp-m11-12 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-12 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/skins/skins" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-12 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-12 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-12 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-12 screenshot "$D/screenshots/12-skins.png" 2>&1 | head -1
echo "=== PAGE 13-battles ==="
"$AB" --session lyp-m11-13 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-13 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-13 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-13 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-13 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-13 screenshot "$D/screenshots/13-battles.png" 2>&1 | head -1
echo "=== PAGE 14-shop ==="
"$AB" --session lyp-m11-14 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-14 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/shop/shop" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-14 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-14 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-14 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-14 screenshot "$D/screenshots/14-shop.png" 2>&1 | head -1
echo "=== PAGE 15-signin ==="
"$AB" --session lyp-m11-15 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-15 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/signin/signin" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-15 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-15 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-15 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-15 screenshot "$D/screenshots/15-signin.png" 2>&1 | head -1
echo "=== PAGE 16-rank ==="
"$AB" --session lyp-m11-16 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-16 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/rank/rank" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-16 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-16 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-16 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-16 screenshot "$D/screenshots/16-rank.png" 2>&1 | head -1
echo "=== PAGE 17-verify ==="
"$AB" --session lyp-m11-17 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-17 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/verify/verify" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-17 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-17 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-17 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-17 screenshot "$D/screenshots/17-verify.png" 2>&1 | head -1
echo "=== PAGE 18-me ==="
"$AB" --session lyp-m11-18 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-18 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/me/me" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-18 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-18 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-18 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-18 screenshot "$D/screenshots/18-me.png" 2>&1 | head -1
echo "=== PAGE 19-login ==="
"$AB" --session lyp-m11-19 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-19 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/login/login" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-19 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-19 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-19 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-19 screenshot "$D/screenshots/19-login.png" 2>&1 | head -1