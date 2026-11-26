#!/bin/bash
# t78 阶段B/C：授予拥有权后 skills/equipment 写入闭环四态截图
D="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bug2fix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$1" "${@:2}" 2>&1 | head -1; }
j(){ $AB --session "$1" eval --stdin < "$D/$2" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
echo "=== GRANT ownership to user10 (QA only, will restore) ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "insert into user_skills(user_id,skill_id,level) values(10,2,1) on conflict do nothing;" 2>&1 | tail -1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "insert into user_equipment(user_id,equipment_id,slot,equipped) values(10,11,'gloves',false) on conflict do nothing;" 2>&1 | tail -1

PAGE=skills; S=lyp-r7b2-skills; run "$S" set viewport 390 844 >/dev/null 2>&1
echo "=== PAGE $PAGE ==="
$AB --session "$S" --init-script "$D/auth-init.js" open "$BASE/pages/$PAGE/$PAGE" 2>&1 | head -1
sleep 10
echo "B_AUTH: $(j "$S" diag-token.js)"
echo "B_CTA: $(j "$S" cta-params.js)"
echo "B_BEFORE_STATE: $(j "$S" state.js)"
echo "B_ME_BEFORE: $(j "$S" me-build.js)"
run "$S" screenshot "$D/screenshots/write-skills-1-before.png" >/dev/null 2>&1
echo "B_OPEN: $(run "$S" click ".r7-btn.primary")"; sleep 2
echo "B_MODAL: $(j "$S" modal-state.js)"
run "$S" screenshot "$D/screenshots/write-skills-2-modal.png" >/dev/null 2>&1
echo "B_CONFIRM: $(run "$S" click ".r7-compare .r7-actions .r7-btn.primary")"; sleep 5
echo "B_APPLY_MSG: $(j "$S" apply-error.js)"
echo "B_ME_AFTER: $(j "$S" me-build.js)"
run "$S" screenshot "$D/screenshots/write-skills-3-after-apply.png" >/dev/null 2>&1
run "$S" reload >/dev/null 2>&1; sleep 10
echo "B_ME_AFTER_RELOAD: $(j "$S" me-build.js)"
echo "B_STATE_AFTER_RELOAD: $(j "$S" state.js)"
run "$S" screenshot "$D/screenshots/write-skills-4-after-reload.png" >/dev/null 2>&1
echo "B_DB:"; docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select slot,skill_id from user_skill_slots where user_id=10 order by slot;" 2>&1 | head -6

PAGE=equipment; S=lyp-r7b2-equip; run "$S" set viewport 390 844 >/dev/null 2>&1
echo "=== PAGE $PAGE ==="
$AB --session "$S" --init-script "$D/auth-init.js" open "$BASE/pages/$PAGE/$PAGE" 2>&1 | head -1
sleep 10
echo "C_AUTH: $(j "$S" diag-token.js)"
echo "C_CTA: $(j "$S" cta-params.js)"
echo "C_BEFORE_STATE: $(j "$S" state.js)"
echo "C_ME_BEFORE: $(j "$S" me-build.js)"
run "$S" screenshot "$D/screenshots/write-equipment-1-before.png" >/dev/null 2>&1
echo "C_OPEN: $(run "$S" click ".r7-btn.primary")"; sleep 2
echo "C_MODAL: $(j "$S" modal-state.js)"
run "$S" screenshot "$D/screenshots/write-equipment-2-modal.png" >/dev/null 2>&1
echo "C_CONFIRM: $(run "$S" click ".r7-compare .r7-actions .r7-btn.primary")"; sleep 5
echo "C_APPLY_MSG: $(j "$S" apply-error.js)"
echo "C_ME_AFTER: $(j "$S" me-build.js)"
run "$S" screenshot "$D/screenshots/write-equipment-3-after-apply.png" >/dev/null 2>&1
run "$S" reload >/dev/null 2>&1; sleep 10
echo "C_ME_AFTER_RELOAD: $(j "$S" me-build.js)"
echo "C_STATE_AFTER_RELOAD: $(j "$S" state.js)"
run "$S" screenshot "$D/screenshots/write-equipment-4-after-reload.png" >/dev/null 2>&1
echo "C_DB:"; docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select equipment_id,slot,equipped from user_equipment where user_id=10 order by equipment_id;" 2>&1 | head -6
echo "=== RESTORE QA STATE ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "update user_skill_slots set skill_id=4 where user_id=10 and slot=1;" 2>&1 | tail -1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "update user_equipment set equipped=(equipment_id=10) where user_id=10 and slot='gloves';" 2>&1 | tail -1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "delete from user_skills where user_id=10 and skill_id=2;" 2>&1 | tail -1
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "delete from user_equipment where user_id=10 and equipment_id=11;" 2>&1 | tail -1
echo "RESTORED_DB:"; docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select slot,skill_id from user_skill_slots where user_id=10 order by slot;" 2>&1 | head -6; docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select equipment_id,equipped from user_equipment where user_id=10 order by 1;" 2>&1 | head -6
echo PHASE_BC_DONE
