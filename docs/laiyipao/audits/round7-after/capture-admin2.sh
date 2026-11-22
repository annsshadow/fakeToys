#!/bin/bash
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7-admin; BASE="http://127.0.0.1:5175/laiyipao-admin/#"; D="."
run(){ $AB --session "$S" "$@" 2>&1; }
echo "BEFORE: $(run get url | head -1)"
run open "$BASE/admin/login" >/dev/null; sleep 4
run eval "(function(){localStorage.clear();return true})()" >/dev/null
run fill "input[placeholder='admin']" "admin"
run fill "input[placeholder='至少 10 位']" "admin12345"
run eval "(function(){var bs=[].slice.call(document.querySelectorAll('button'));for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').indexOf('登录')>=0){bs[i].click();return 'clicked'}}return 'no-btn'})()"
sleep 6; echo "AFTER: $(run get url | head -1)"
run eval "(function(){return JSON.stringify({url:location.href,tok:!!localStorage.getItem('lyp_admin_token'),keys:Object.keys(localStorage)})})()"
for V in dashboard levels users; do
  run open "$BASE/admin/$V" >/dev/null; sleep 5
  run screenshot "$D/screenshots/r7-after-admin-$V.png" >/dev/null
  echo "--- $V --- $($AB --session "$S" eval --stdin < "$D/adminfs.js" 2>&1 | head -1)"
done
run open "$BASE/admin/dashboard" >/dev/null; sleep 3
run press "Control+k" >/dev/null; sleep 2
echo "PALETTE: $(run eval "(function(){var p=document.querySelector('.cmd-panel');return JSON.stringify({panel:document.querySelectorAll('.cmd-panel').length,aiGroup:document.querySelectorAll('.cmd-ai-group-label').length,aiOpts:document.querySelectorAll('.cmd-ai-option').length,ariaModal:p?p.getAttribute('aria-modal'):null,focused:(document.activeElement||{}).className||''})})()" | head -1)"
run screenshot "$D/screenshots/r7-after-admin-palette.png" >/dev/null
echo ADMIN_DONE
