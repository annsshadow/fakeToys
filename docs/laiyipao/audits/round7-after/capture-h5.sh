#!/bin/bash
# t67 R7 H5 真浏览器复查（本地回环，非物理真机 390x844）
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7-h5; BASE="http://127.0.0.1:5174/#"; D="."
run(){ $AB --session "$S" "$@" 2>&1; }
j(){ $AB --session "$S" eval --stdin < "$D/$1" 2>&1; }
echo '=== LOGIN ==='
run --init-script "$D/probe-init.js" open "$BASE/pages/login/login" >/dev/null; sleep 5
run set viewport 390 844 >/dev/null
run eval "(function(){try{localStorage.removeItem('lyp_token');localStorage.removeItem('lyp_token_exp');localStorage.removeItem('lyp_profile')}catch(e){};return true})()" >/dev/null
run open "$BASE/pages/login/login" >/dev/null; sleep 5
run fill "input[type='text']" "admin" >/dev/null
run fill "input[type='password']" "admin12345" >/dev/null
run click ".btn-primary" >/dev/null; sleep 7
echo "AUTH: $(j fs.js | head -1)"
echo '=== INDEX ==='
run open "$BASE/pages/index/index" >/dev/null; sleep 5
run screenshot "$D/screenshots/r7-after-index-guide.png" >/dev/null
echo "INDEX: $(j fs.js | head -1)"
echo '=== SKILLS EQUIPMENT ==='
for P in skills equipment; do
  run open "$BASE/pages/$P/$P" >/dev/null; sleep 6
  run screenshot "$D/screenshots/r7-after-$P.png" >/dev/null
  echo "--- $P ---"
  echo "DOM: $(j fs.js | head -1)"
  echo "IMPACT: $(j impact.js | head -1)"
done
echo H5_DONE
