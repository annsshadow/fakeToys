#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r11
D=docs/laiyipao/audits/round11-number-format
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 3
echo "UNI=$(ev "typeof uni")"
echo "VUE_PROBE=$(ev "(()=>{const e=document.querySelector('.r7-impact')||document.querySelector('.page-root');if(!e)return 'NO_EL';const k=Object.keys(e).find(x=>x.startsWith('__vue'));return k||'NO_VUE_KEY:'+Object.keys(e).slice(-8).join(',')})()")"
echo "SETUP_PROBE=$(ev "(()=>{const e=document.querySelector('.page-root');let c=e&&e.__vueParentComponent;return c?Object.keys(c.setupState||{}).filter(x=>/r7Impact|r6Rec|fmt/.test(x)).join(','):'NO_C'})()")"
