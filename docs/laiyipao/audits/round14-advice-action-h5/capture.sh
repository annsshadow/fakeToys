AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
D=docs/laiyipao/audits/round14-advice-action-h5/screenshots
$AB --session lyp-r14 open http://127.0.0.1:5174/#/pages/battles/battles
$AB --session lyp-r14 set viewport 390 844
$AB --session lyp-r14 wait 3000
$AB --session lyp-r14 screenshot $D/browser-battles.png
$AB --session lyp-r14 open http://127.0.0.1:5174/#/pages/reactions/reactions
$AB --session lyp-r14 set viewport 390 844
$AB --session lyp-r14 wait 3000
$AB --session lyp-r14 screenshot $D/browser-reactions.png
echo CAPTURE_DONE