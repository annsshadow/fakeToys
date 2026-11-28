# R110 搜索空态清除搜索键盘契约 RED 证据

- 任务：t320 / attempt_id=5abc04e9-a5eb-4366-84d8-953f7ed6b40c
- 基线 HEAD：`f11a1eae323d834433e6feaa749a609e94d95b79`（全程未变）
- 完整原始 stdout：docs/laiyipao/specs/round110-clear-search-keyboard/RED-raw.log

## 命令

cd laiyipao/miniapp && npx vitest run src/styles/r110ClearSearchKeyboard.contract.test.ts --reporter=verbose

## 结果（实测）

- exit=1（vitest 退出码，命令重定向捕获 RED_RC=1）
- Test Files 1 failed (1)
- Tests 6 failed | 6 passed (12)
- Duration 303ms (transform 27ms, setup 0ms, import 40ms, tests 25ms, environment 0ms)

## 6 failed（全为真实缺口：role="button" 缺失）

1. × 缺口 pages/enemies/enemies.vue clear-search 三件套齐全 — AssertionError: 须 role=button
2. × 缺口 pages/equipment/equipment.vue — 同上
3. × 缺口 pages/gems/gems.vue — 同上
4. × 缺口 pages/reactions/reactions.vue — 同上
5. × 缺口 pages/skills/skills.vue — 同上
6. × 缺口 pages/skins/skins.vue — 同上

## 6 passed（不变量全绿）

1. ✓ 不变量1 announcements.vue 已修 .link 三件套仍在（R99_A）
2. ✓ 不变量2 r72SearchEmpty 空态结构保留
3. ✓ 不变量3 令牌三锚点 + 7 页零 --r110-
4. ✓ 不变量4 R99/R100/R102/R104/R109 既有三件套抽查无回退
5. ✓ 不变量5 6 处 @click handler + 双锚点唯一定位
6. ✓ 不变量6 common.css .clear-search 视觉样式未动

## 无解析错误

- RED-raw.log 无 PARSE_ERROR / SyntaxError / Transformation error / Cannot find 命中。

## 自纠记录（诚实披露）

- 首轮 RED 曾得 7 failed | 5 passed：不变量4 误把 battle.vue:77 锚点路径写成 battles.vue，导致一条不变量失败（非目标缺口）。经 grep 定位真实锚点在 pages/battle/battle.vue:77，修正路径后复跑得干净的 6 failed | 6 passed。
- 首轮写契约时模板字符串把转义单引号求值成裸单引号，致第 90 行语法错误；用 edit_file 以 String.fromCharCode(39) 拼接修复。两处自纠均在最终 RED 前完成。
- 首次 RED 重定向因 specs 目录不存在失败（无日志），mkdir -p 后重跑成功。

## Hygiene

- HEAD 未变；产出全 UNTRACKED；业务 vue / .gitignore / 令牌 / 既有契约零 diff；零真值。
