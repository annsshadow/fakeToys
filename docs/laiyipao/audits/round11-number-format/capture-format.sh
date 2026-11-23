#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r11
D=docs/laiyipao/audits/round11-number-format/screenshots
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
"$AB" --session "$S" route "**/me/recommendations?context=skills" --body '{"schema_version":"rec-v1","generated_at":"2026-11-22T05:00:00Z","context":"skills","subject":{"user_key":"hash","level":3,"power":1234},"input_snapshot":{"battle_count":3,"window_days":7,"config_version":"cfg-abc"},"evidence":[],"insights":[{"id":"skill-rec-1","headline":"AI 推荐使用重炮弹","basis":["近 7 天 3 场","星级概率无可信模型，未做推测"],"cta":{"label":"应用推荐","route":"","params":{"id":4,"replace":1}},"confidence":"rule_based","model":{"name":"rules","version":"rec-v1"}}],"empty_reason":null,"model":{"name":"rules","version":"rec-v1","kind":"rule_based"}}' 2>&1 | head -3
"$AB" --session "$S" route "**/me/recommendations/impact?*" --body '{"schema_version":"rec-v1","generated_at":"2026-11-22T05:00:00Z","context":"skill","subject":{"user_key":"hash","level":3,"power":1234},"input_snapshot":{"battle_count":3,"window_days":7,"config_version":"cfg-abc"},"evidence":[],"insights":[{"id":"impact-skill-1","headline":"换上后每波伤害提升","basis":["近 7 天 3 场","星级概率无可信模型，未做推测"],"cta":{"label":"去换技能","route":"","params":{"id":4,"replace":1}},"confidence":"rule_based","model":{"name":"rules","version":"rec-v1"}}],"empty_reason":null,"model":{"name":"rules","version":"rec-v1","kind":"rule_based"},"impact":{"kind":"skill","candidate_id":4,"replaced_id":1,"candidate_name":"重炮弹","replaced_name":"燃烧弹","baseline":{"attack_permille":1000,"armor_permille":0,"estimated_damage_per_wave":5.400000000000001,"power":100,"rating_total":40},"candidate":{"attack_permille":1120,"armor_permille":0,"estimated_damage_per_wave":9.69,"power":112,"rating_total":44},"changes":{"power_delta":0.30000000000000004,"rating_total_delta":4,"attack_permille_delta":120,"armor_permille_delta":0,"damage_per_wave_delta":4.299999999999997,"damage_per_wave_pct":79.62962962962963,"star_rate_observed":33.333,"star_probability_estimate":null},"method":"命中/波×技能底伤(等级)×(1000+攻击)/1000","star_rate_note":"只给历史三星率；星级概率无可信模型，未做推测"}}' 2>&1 | head -3
echo ROUTES_SET
"$AB" --session "$S" reload >/dev/null 2>&1
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 4
echo "MOBILE_TEXT:"
ev "(()=>{const e=document.querySelector('.r7-impact');return e?e.innerText.slice(0,700):'NO_IMPACT'})()" > docs/laiyipao/audits/round11-number-format/mobile-impact-text.txt 2>&1
cat docs/laiyipao/audits/round11-number-format/mobile-impact-text.txt
echo "MOBILE_LONGTAIL=$(ev "document.body.innerText.includes('5.400000000000001') || document.body.innerText.includes('79.62962962962963')")"
"$AB" --session "$S" screenshot "$D/after-skills-mobile.png" >/dev/null 2>&1
echo SHOT_MOBILE
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
sleep 2
ev "(()=>{const e=document.querySelector('.r7-impact');return e?e.innerText.slice(0,700):'NO_IMPACT'})()" > docs/laiyipao/audits/round11-number-format/desktop-impact-text.txt 2>&1
cat docs/laiyipao/audits/round11-number-format/desktop-impact-text.txt
"$AB" --session "$S" screenshot "$D/after-skills-desktop.png" >/dev/null 2>&1
echo SHOT_DESKTOP
echo CAPTURE_DONE
