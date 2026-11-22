#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r8-growth; D='docs/laiyipao/audits/round8-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round8-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
echo '=== LOGIN ==='
run --init-script "$D/probe-init.js" open 'http://127.0.0.1:5174/#/pages/login/login' >/dev/null 2>&1; sleep 4
run set viewport 390 844 >/dev/null 2>&1
run eval "(function(){try{localStorage.removeItem('lyp_token');localStorage.removeItem('lyp_token_exp');localStorage.removeItem('lyp_profile')}catch(e){};return true})()" >/dev/null 2>&1
run fill "input[type='text']" 'admin' >/dev/null 2>&1
run fill "input[type='password']" 'admin12345' >/dev/null 2>&1
run click '.btn-primary' >/dev/null 2>&1; sleep 6
echo "TOKEN: $(run eval "(function(){return 'len='+(localStorage.getItem('lyp_token')||'').length})()" 2>&1|head -1)"
echo '=== RESET PROBE + SKILLS ==='
run eval "(function(){var p=window.__LYP_PROBE__;if(p){p.pageErrors=[];p.consoleErrors=[];p.apiFailures=[];p.failedRequests=[]}return 'RESET'})()" >/dev/null 2>&1
run open 'http://127.0.0.1:5174/#/pages/skills/skills' >/dev/null 2>&1; sleep 5
echo "SKILLS_LOGGED: $(run eval --stdin < "$D/growth-probe.js" 2>&1|head -1)"
run screenshot "$SHOT/r8-growth-skills-logged.png" >/dev/null 2>&1; echo SHOT_DONE
echo GROWTH_DONE