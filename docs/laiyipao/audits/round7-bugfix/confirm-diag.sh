#!/bin/bash
DA="docs/laiyipao/audits/round7-bugfix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7fix
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$DA/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
echo "BEFORE_OPEN: $(j state.js)"
echo "OPEN: $(run click ".r7-btn.primary")"
sleep 2
echo "MODAL_PRE: $(j modal-state.js)"
echo "CONFIRM_CLICK2: $(run click ".r7-compare .r7-actions .r7-btn.primary")"; sleep 5
echo "MODAL_POST: $(j modal-state.js)"
echo "ERRORS: $(j apply-error.js)"
echo "ME_AFTER2: $(j me-build.js)"
run screenshot "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bugfix/screenshots/r7fix-equipment-confirm-diag.png" >/dev/null 2>&1
