#!/usr/bin/env bash
set -u
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE="http://127.0.0.1:5174/#"
ROOT="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round98-form-enter"
cd "$ROOT" || exit 1
run_case(){
 local S="$1" W="$2" H="$3" PG="$4" SEL="$5" VAL="$6" CLICK="$7" LABEL="$8"
 AB $S close --all >/dev/null 2>&1; AB $S open >/dev/null 2>&1
 AB $S set viewport $W $H >/dev/null 2>&1
 AB $S navigate "$BASE/pages/$PG/$PG" >/dev/null 2>&1
 sleep 3
 echo "===== $LABEL $PG ${W}x${H} ====="
 echo -n 'HOOK:'; AB $S eval --stdin < data/r98-hook2.js
 echo 'BEFORE:'; AB $S eval --stdin < data/r98-probe2.js
 AB $S fill "$SEL" "$VAL" >/dev/null 2>&1
 echo -n 'INPUT:'; AB $S eval --stdin < data/r98-inputval.js
 AB $S press Enter >/dev/null 2>&1; sleep 2
 echo 'AFTER_ENTER:'; AB $S eval --stdin < data/r98-probe2.js
 AB $S screenshot "$ROOT/screenshots/r98-$PG-$W-enter.png" >/dev/null 2>&1; echo "SHOT_ENTER:r98-$PG-$W-enter.png"
 AB $S reload >/dev/null 2>&1; sleep 3
 echo -n 'HOOK2:'; AB $S eval --stdin < data/r98-hook2.js
 AB $S fill "$SEL" "$VAL" >/dev/null 2>&1
 echo -n 'CLICK:'; AB $S eval --stdin < "$CLICK"
 sleep 2
 echo 'AFTER_CLICK:'; AB $S eval --stdin < data/r98-probe2.js
}
i=0
for V in 390:844 768:1024 1280:800; do
 W=${V%%:*}; H=${V##*:}
 run_case r98s$i $W $H signin 'input[aria-label="输入兑换码"]' 'QA-CODE-001' data/click-signin.js SIGNIN; i=$((i+1))
 run_case r98v$i $W $H verify 'input[aria-label="battle_id"]' '12345' data/click-verify.js VERIFY; i=$((i+1))
 run_case r98b$i $W $H bag 'input[aria-label="预设名（最多 20 字）"]' 'QA预设测试' data/click-bag.js BAG; i=$((i+1))
done
echo '=== ALL_DONE ==='
