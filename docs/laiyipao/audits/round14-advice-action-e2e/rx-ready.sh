#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-rxfix
SH="$PWD/$D/screenshots"
echo '=== BEFORE DB ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select slot||':'||skill_id from user_skill_slots where user_id=10 order by slot" 2>/dev/null|tr -d '\r'|paste -sd, -
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open http://127.0.0.1:5174/#/pages/reactions/reactions >/dev/null 2>&1
sleep 3
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 8
echo '=== READY DOM ==='
"$AB" --session $S eval --stdin 2>&1 < "$PWD/$D/rx-probe.js"
"$AB" --session $S screenshot "$SH/r14-rxfix-01-ready.png" >/dev/null 2>&1
echo SHOT1=$?
echo '=== CLICK CTA ==='
"$AB" --session $S click '#r14-advice-action-btn' >/dev/null 2>&1
sleep 2
"$AB" --session $S eval --stdin 2>&1 < "$PWD/$D/rx-modal-probe.js"
"$AB" --session $S screenshot "$SH/r14-rxfix-02-compare.png" >/dev/null 2>&1
echo SHOT2=$?
echo '=== CONFIRM ==='
"$AB" --session $S click '.r7-btn.primary' >/dev/null 2>&1
sleep 4
"$AB" --session $S eval --stdin 2>&1 < "$PWD/$D/rx-applied-probe.js"
"$AB" --session $S screenshot "$SH/r14-rxfix-03-applied.png" >/dev/null 2>&1
echo SHOT3=$?
echo '=== AFTER DB ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select slot||':'||skill_id from user_skill_slots where user_id=10 order by slot" 2>/dev/null|tr -d '\r'|paste -sd, -