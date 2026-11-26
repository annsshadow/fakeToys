#!/usr/bin/env bash
cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
echo '=== TOKENS THREE ANCHORS =='
grep -nE -- "--lyp-token-version|--lyp-r3-version|--lyp-r6-version" laiyipao/miniapp/src/styles/tokens.css | head -5
echo '=== R60 KEYBOARD COUNTS =='
echo -n "role_button="; grep -r --include="*.vue" -h -o "role="button"" laiyipao/miniapp/src | wc -l
echo -n "tabindex0="; grep -r --include="*.vue" -h -o "tabindex="0"" laiyipao/miniapp/src | wc -l
echo -n "keydown_enter="; grep -r --include="*.vue" -h -o "@keydown.enter" laiyipao/miniapp/src | wc -l
echo '=== R55 PLACEHOLDER =='
echo -n "placeholder_class_ph="; grep -r --include="*.vue" -h -o "placeholder-class="ph"" laiyipao/miniapp/src | wc -l
echo '=== NO --r72- IN BUSINESS =='
echo -n "r72_business_hits="; grep -r --include="*.vue" --include="*.ts" --include="*.css" -h -o -- "--r72-" laiyipao/miniapp/src | wc -l
echo '=== VERIFY R71 ANCHORS =='
grep -nE -- "fmtVerifyTime\(replayInfo.created_at\)|pretty\(replayInfo.expected_hash\)|buildSnapshotCard" laiyipao/miniapp/src/pages/verify/verify.vue | head -8
echo '=== GEM CACHE GUARD R66 ==='
grep -nE -- "normalizeGemGain|佩戴收益未上报" laiyipao/miniapp/src/pages/gems/gems.vue laiyipao/miniapp/src/pages/_shared/r10GemGain.ts | head -6
echo '=== NICKNAME BUSY R69 =='
grep -nE -- "nicknameSubmitting|提交中" laiyipao/miniapp/src/pages/me/me.vue | head -8
echo '=== DIFF NAMES =='
git diff --name-only HEAD
echo STATIC_DONE