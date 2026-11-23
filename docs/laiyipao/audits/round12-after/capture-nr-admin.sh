#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-admin
echo URL=$("$AB" --session $S get url 2>&1|head -1)
echo '=== TOPBAR ==='; "$AB" --session $S eval --stdin < "$D/nr-admin-probe.js" 2>&1|head -1
echo CLICK=$("$AB" --session $S click '.command-trigger' 2>&1|head -1)
sleep 1
echo '=== PALETTE ==='; "$AB" --session $S eval 'JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,aria:document.querySelector(".cmd-panel")?document.querySelector(".cmd-panel").getAttribute("aria-modal"):null})' 2>&1|head -1
echo NR_DONE