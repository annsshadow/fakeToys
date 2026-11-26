# R9 战斗层缺口 1+3 实测与修复（t91）

> 环境：本地回环 H5 真浏览器（agent-browser / Chromium，会话 `lyp-gp-t91`，视口 1258×588）。
> **非物理真机**；所有截图均为本地回环页面，不含真实域名、IP、口令或私钥。

## 1. 任务与依据

Round9 Top3 缺口①③在战斗层落地：t88 已提供最近 3 条反应记录和下一关入口，但每条只有名称与伤害；选卡/跳过后玩家看不到可归因反馈。t90 设计规范 §1H 要求：反应记录须含反应名称、触发元素组合、伤害与 AI 因果；选卡反馈以 `card_picks` 为权威来源，收益须可归因、禁止虚构。

本轮采用 test-first RED→GREEN，并在真浏览器连续实战第 1～3 关复测同一流程。

## 2. 真实缺陷与 RED

1. 反应因果不可读：8× 下 canvas 粒子一闪而过，HUD 最近记录只显示「蒸汽爆发 -1234」，玩家看不出是哪两种元素触发，也看不出收益依据。
2. 选卡取舍不可感知：波次间取卡/弃牌/跳过只改变内部状态，结算前没有「我的选择带来了什么」的反馈。

RED 命令：

```bash
cd laiyipao/miniapp
npx vitest run --config vitest.config.ts src/game/feedback.test.ts src/pages/battle/battle.test.ts --reporter=verbose
```

RED 结果：exit 1，2 failed files / 3 failed tests；逐字错误包括 `Cannot find module './feedback'`、`expected '蒸汽爆发抗性削减 -88过热 -1234' to contain '焰+电 → 过热'`、`expected false to be true`。

## 3. GREEN 实现

- `laiyipao/miniapp/src/game/engine.ts`：`BattleEvent` 的 `reaction` 事件增加可选 `existingElement` / `incomingElement`；在 `applyElementStacks` 前读取 `def.dominantElement()`，emit 时填入真实元素。只用于展示，不改 record、replay_hash 或服务端协议。
- `laiyipao/miniapp/src/game/feedback.ts`（新增）：`summarizeCardPicks(picks)`、`damageShareFromEvents(events)`、`reactionCue(reaction, existing, incoming, damage)`。本地复刻 `decodeDiscardCode` 的编码契约并由单测逐码比对；伤害占比以 `hit.damage` 为分母、`reaction.damage` 为分子，避免重复计数。
- `laiyipao/miniapp/src/pages/battle/battle.vue`：HUD 每条反应显示「元素组合 → 反应名 / 伤害 / AI 因果（依据 + CTA）」；选卡 sheet 与结算卡显示取/弃/跳与反应次数、伤害占比、依据与下一局建议。元素组合缺失时写「元素组合未上报」，无事件时不编造占比。沿用现有颜色与排版，无新增视觉令牌、无 `--r9-*`。
- 反应记录保留最近 3 条，每局 `setup()` 清空；浮层 `pointer-events: none`，不遮挡按钮与战场操作，与 R7 星级差额、t82 钳分胶囊并列而不重叠。

GREEN 结果：定向 2 files / 55 tests pass；`battle.test.ts` 48/48；双类型检查 exit 0。

## 4. 真浏览器连续三关复测（非物理真机 / 本地回环）

流程：首页 CTA → 选关 → 开始战斗 → 8× → 真实点击卡牌 → 逐波真实点击「全部不要了，直接下一波」 → 结算 CTA 进入下一关，重复三关。浏览器 `errors --json` 全程 `errors=[]`。

| 关卡 | HUD 因果链实例 | 选卡反馈 | 结算反馈 | 结算数据 |
| --- | --- | --- | --- | --- |
| 1 | 电+焰 → 过热 -286；电+冰 → 超导 -185 | 取卡 1、跳过 0，反应 8 次 | 取卡 1、跳过 3，反应 51 次，反应伤害占比 44.5% | 通关 C；39 杀 / 0 漏；8255 分；hash `39f022a838d64340` |
| 2 | 电+焰 → 过热 -216；冰+电 → 超导 -133 | 取卡 1、跳过 0，反应 12 次 | 取卡 1、跳过 3，反应 81 次，反应伤害占比 37.3% | 通关 C；38 杀 / 0 漏；9702 分；hash `73efdf2068c04349` |
| 3 | 焰+冰 → 蒸汽爆发 -235；冰+焰 → 蒸汽爆发 -160 | 取卡 1、跳过 0，反应 6→17 次 | 取卡 1、跳过 3，反应 99 次，反应伤害占比 44.3% | 通关 B；32 杀 / 0 漏；11066 分；hash `4b039106d3f7708d` |

三关结算均保留 R7 星级差额建议与 t82/t64 钳分归因，示例（第 3 关）：「73s 内得分上限 16309，实得 11066；距 2 星门槛 13917 还差 2851；漏怪 0；建议缩短清波时间或少漏怪、多触发反应提分。」

## 5. 截图证据

所有图片位于 `docs/laiyipao/audits/round9-combat/screenshots/`，均为 1258×588 PNG：

- `r9c-01-level1-start.png`：第 1 关开局 HUD（星级差额 11941、钳分倒计时 131s）。
- `r9c-02-reaction-causality.png`：第 1 关因果链 HUD（元素组合→反应名→伤害→AI 依据/CTA）与选卡反馈同屏。
- `r9c-03-card-picked.png`：真实点卡后反馈更新为「已取卡 1 张」。
- `r9c-04-level1-result.png`：第 1 关结算反馈（取卡 1 / 跳过 3 / 占比 44.5%）。
- `r9c-05-level2-causality.png`：第 2 关因果链 HUD。
- `r9c-06-level2-card-feedback.png`：第 2 关真实点卡后的反馈。
- `r9c-07-level2-result.png`：第 2 关结算反馈。
- `r9c-08-level3-causality.png`：第 3 关因果链 HUD（目视确认不遮挡按钮）。
- `r9c-09-level3-card-feedback.png`：第 3 关真实点卡后的反馈。
- `r9c-10-level3-result.png`：第 3 关结算反馈（目视确认 B / 1 星 / 占比 44.3%）。

## 6. 验证命令与结果

```bash
cd laiyipao/miniapp
npx vitest run --config vitest.config.ts src/game/feedback.test.ts src/pages/battle/battle.test.ts --reporter=verbose   # 55 passed
npm test                                                                                                           # 178 files passed | 1 skipped (179); 1698 passed | 2 skipped (1700)
npm run type-check                                                                                                 # exit 0
npm run type-check:test                                                                                            # exit 0
```

## 7. 诚实边界与未做事项

- 反应元素来自引擎事件新增字段；旧路径/缺字段时写「元素组合未上报」，不猜元素。
- 反应伤害占比来自本局 `hit` / `reaction` 事件；没有事件时不做数值外推。`card_picks` 统计只解码权威编码，不读取 UI 临时状态作为结算依据。
- 不修改 admin / 后端 / stage.vue；未新增 CSS 令牌；未执行任何 git 操作；未在仓库、日志或对话中写入真实凭据/域名/私钥。
- 三关均为本地回环真浏览器实战，**非物理真机**；长局（≥20 关）与更多构筑路线的收益对比仍属后续玩法缺口。