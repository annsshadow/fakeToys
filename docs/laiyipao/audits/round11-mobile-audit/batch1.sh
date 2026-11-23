#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-mobile-audit'
echo "=== PAGE 01-index ==="
"$AB" --session lyp-m11-01 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-01 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-01 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-01 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-01 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-01 screenshot "$D/screenshots/01-index.png" 2>&1 | head -1
echo "=== PAGE 02-announcements ==="
"$AB" --session lyp-m11-02 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-02 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/announcements/announcements" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-02 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-02 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-02 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-02 screenshot "$D/screenshots/02-announcements.png" 2>&1 | head -1
echo "=== PAGE 03-stage ==="
"$AB" --session lyp-m11-03 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-03 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/stage/stage" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-03 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-03 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-03 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-03 screenshot "$D/screenshots/03-stage.png" 2>&1 | head -1
echo "=== PAGE 04-battle ==="
"$AB" --session lyp-m11-04 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-04 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-04 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-04 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-04 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-04 screenshot "$D/screenshots/04-battle.png" 2>&1 | head -1
echo "=== PAGE 05-bag ==="
"$AB" --session lyp-m11-05 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-05 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/bag/bag" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-05 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-05 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-05 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-05 screenshot "$D/screenshots/05-bag.png" 2>&1 | head -1
echo "=== PAGE 06-mastery ==="
"$AB" --session lyp-m11-06 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-06 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/mastery/mastery" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-06 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-06 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-06 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-06 screenshot "$D/screenshots/06-mastery.png" 2>&1 | head -1
echo "=== PAGE 07-reactions ==="
"$AB" --session lyp-m11-07 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-07 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/reactions/reactions" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-07 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-07 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-07 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-07 screenshot "$D/screenshots/07-reactions.png" 2>&1 | head -1
echo "=== PAGE 08-enemies ==="
"$AB" --session lyp-m11-08 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-08 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/enemies/enemies" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-08 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-08 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-08 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-08 screenshot "$D/screenshots/08-enemies.png" 2>&1 | head -1
echo "=== PAGE 09-skills ==="
"$AB" --session lyp-m11-09 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-09 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-09 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-09 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-09 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-09 screenshot "$D/screenshots/09-skills.png" 2>&1 | head -1
echo "=== PAGE 10-equipment ==="
"$AB" --session lyp-m11-10 set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11-10 --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/equipment/equipment" >/dev/null 2>&1
sleep 3
echo -n 'URL='; "$AB" --session lyp-m11-10 get url 2>&1 | head -1
echo -n 'TITLE='; "$AB" --session lyp-m11-10 get title 2>&1 | head -1
echo 'PROBE='; "$AB" --session lyp-m11-10 eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-10 screenshot "$D/screenshots/10-equipment.png" 2>&1 | head -1