#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r110patch
D=docs/laiyipao/audits/round11-number-format-patch/screenshots
mkdir -p "$D"
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
# 打开首页（页面需先加载 uni 运行时）
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 3
# 顶层 evidence 注入长浮点 + 字符串型配置版本；insights 提供合法卡片
REC="{\"schema_version\":\"rec-v1\",\"generated_at\":\"2026-11-23T05:00:00Z\",\"context\":\"skills\",\"subject\":{\"user_key\":\"hash\",\"level\":3,\"power\":1234},\"input_snapshot\":{\"battle_count\":3,\"window_days\":7,\"config_version\":\"cfg-abc\"},\"evidence\":[{\"metric\":\"胜率\",\"value\":79.62962962962963,\"unit\":\"%\",\"source\":\"battle_records\",\"window\":\"近7天\"},{\"metric\":\"战力增益\",\"value\":0.30000000000000004,\"unit\":\"\",\"source\":\"battle_records\",\"window\":\"近7天\"},{\"metric\":\"配置版本\",\"value\":\"cfg-sha-abc123\",\"unit\":\"\",\"source\":\"config\",\"window\":\"now\"}],\"insights\":[{\"id\":\"rec-1\",\"headline\":\"AI 推荐使用重炮弹\",\"basis\":[\"近 7 天 3 场\"],\"cta\":{\"label\":\"应用推荐\",\"route\":\"\",\"params\":{\"id\":4,\"replace\":1}},\"confidence\":\"rule_based\",\"model\":{\"name\":\"rules\",\"version\":\"rec-v1\"}}],\"empty_reason\":null,\"model\":{\"name\":\"rules\",\"version\":\"rec-v1\",\"kind\":\"rule_based\"}}"
IMP="{\"schema_version\":\"rec-v1\",\"generated_at\":\"2026-11-23T05:00:00Z\",\"context\":\"skills\",\"subject\":{\"user_key\":\"hash\",\"level\":3,\"power\":1234},\"input_snapshot\":{\"battle_count\":3,\"window_days\":7,\"config_version\":\"cfg-abc\"},\"evidence\":[{\"metric\":\"三星率\",\"value\":79.62962962962963,\"unit\":\"%\",\"source\":\"battle_records\",\"window\":\"近7天\"},{\"metric\":\"增益\",\"value\":0.30000000000000004,\"unit\":\"\",\"source\":\"battle_records\",\"window\":\"近7天\"},{\"metric\":\"版本\",\"value\":\"cfg-sha-abc123\",\"unit\":\"\",\"source\":\"config\",\"window\":\"now\"}],\"insights\":[{\"id\":\"imp-1\",\"headline\":\"换上后每波伤害提升\",\"basis\":[\"近 7 天 3 场\"],\"cta\":{\"label\":\"去换\",\"route\":\"\",\"params\":{\"id\":4,\"replace\":1}},\"confidence\":\"rule_based\",\"model\":{\"name\":\"rules\",\"version\":\"rec-v1\"}}],\"empty_reason\":null,\"model\":{\"name\":\"rules\",\"version\":\"rec-v1\",\"kind\":\"rule_based\"},\"impact\":{\"kind\":\"skill\",\"candidate_id\":4,\"replaced_id\":1,\"candidate_name\":\"重炮弹\",\"replaced_name\":\"燃烧弹\",\"baseline\":{\"attack_permille\":1000,\"armor_permille\":0,\"estimated_damage_per_wave\":5.400000000000001,\"power\":100,\"rating_total\":40},\"candidate\":{\"attack_permille\":1120,\"armor_permille\":0,\"estimated_damage_per_wave\":9.7,\"power\":112,\"rating_total\":44},\"changes\":{\"power_delta\":0.30000000000000004,\"rating_total_delta\":4,\"attack_permille_delta\":120,\"armor_permille_delta\":0,\"damage_per_wave_delta\":4.299999999999997,\"damage_per_wave_pct\":79.62962962962963,\"star_rate_observed\":33.333,\"star_probability_estimate\":null},\"method\":\"命中/波×技能底伤\",\"star_rate_note\":\"只给历史三星率\"}}"
"$AB" --session "$S" network route "**/recommendations*" --body "$REC" --allow-origin http://127.0.0.1:5174 >/dev/null 2>&1
"$AB" --session "$S" network route "**/recommendations/impact*" --body "$IMP" --allow-origin http://127.0.0.1:5174 >/dev/null 2>&1
echo ROUTES_SET
capture() {
  pg=$1; name=$2
  "$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/$pg/$pg" >/dev/null 2>&1
  "$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
  sleep 3
  LONG=$(ev "document.body.innerText.includes('79.62962962962963')||document.body.innerText.includes('0.30000000000000004')||document.body.innerText.includes('5.400000000000001')||document.body.innerText.includes('4.299999999999997')" 2>/dev/null)
  EVT=$(ev "Array.from(document.querySelectorAll('.r6-ev-val')).map(e=>e.innerText).join(' | ')" 2>/dev/null)
  echo "$name MOBILE LONGTAIL=$LONG EV=[$EVT]"
  "$AB" --session "$S" screenshot "$D/after-$name-mobile.png" >/dev/null 2>&1
  "$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
  sleep 2
  LONG2=$(ev "document.body.innerText.includes('79.62962962962963')||document.body.innerText.includes('0.30000000000000004')" 2>/dev/null)
  echo "$name DESKTOP LONGTAIL=$LONG2"
  "$AB" --session "$S" screenshot "$D/after-$name-desktop.png" >/dev/null 2>&1
}
capture skills skills
capture equipment equipment
capture reactions reactions
echo CAPTURE_DONE