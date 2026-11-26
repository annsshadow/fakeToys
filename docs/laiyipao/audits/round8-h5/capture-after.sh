#!/usr/bin/env bash
# R7 H5 after 截图：桌面 Chrome 模拟移动视口，非物理真机；本地回环 127.0.0.1。
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
SESSION=lyp-r7a
BASE="http://127.0.0.1:5174/#"
OUT="docs/laiyipao/audits/round8-h5/screenshots"
mkdir -p "$OUT"

capture() {
  local route="$1" name="$2"
  $AB --session "$SESSION" open "$BASE$route"
  sleep 2
  $AB --session "$SESSION" set viewport 390 844
  sleep 1
  $AB --session "$SESSION" wait 2600
  $AB --session "$SESSION" screenshot "$OUT/$name.png"
  $AB --session "$SESSION" eval "(()=>{const el=[...document.querySelectorAll('[role=button]')].find(e=>e.textContent.includes('应用推荐'));if(el){el.click();return 'clicked'}return 'no-impact-cta'})()"
  sleep 1
  $AB --session "$SESSION" screenshot "$OUT/$name-modal.png"
}

capture "/pages/skills/skills" "after-skills"
capture "/pages/equipment/equipment" "after-equipment"
echo CAPTURE_DONE
