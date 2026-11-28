# R109 RED 证据

## 意图

- 契约 r109PriorityCtaKeyboard.contract.test.ts 以 10 缺口 it + 6 不变量 it 锁定 10 处裸 view CTA 的键盘三件套。
- RED 期望：恰好 10 个缺口 it 失败（role="button" 缺失）、6 个不变量 it 通过、无 PARSE_ERROR，证明契约能精确捕获缺口且不误伤既有保护面。

## RED 命令

- `cd laiyipao/miniapp && npx vitest run src/styles/r109PriorityCtaKeyboard.contract.test.ts --reporter=verbose`
- 首跑：exit=1，Tests 10 failed | 6 passed (16)，Test Files 1 failed (1)，Duration 341ms。
- 复跑（RED-raw.log）：exit=1，Tests 10 failed | 6 passed (16)，Duration 310ms，一致。

## 失败摘要（逐字摘录，完整输出见 RED-raw.log）

- × 缺口 pages/gems/gems.vue r12-priority-cta r12Go() → pages/gems/gems.vue 须 role=button
- × 缺口 pages/skills/skills.vue r12-priority-cta r12Go() → 须 role=button
- × 缺口 pages/equipment/equipment.vue r12-priority-cta r12Go() → 须 role=button
- × 缺口 pages/bag/bag.vue r12-priority-cta r12Go() → 须 role=button
- × 缺口 pages/me/me.vue r3-ai-cta r4GoMe() → 须 role=button
- × 缺口 pages/bag/bag.vue r3-ai-cta r4GoBag() → 须 role=button
- × 缺口 pages/battles/battles.vue r3-ai-cta r4GoBattle() → 须 role=button
- × 缺口 pages/enemies/enemies.vue r3-ai-cta r4FocusEnemy() → 须 role=button
- × 缺口 pages/rank/rank.vue r3-ai-cta r4GoRank() → 须 role=button
- × 缺口 pages/skins/skins.vue r3-ai-cta r4FocusSkin() → 须 role=button
- ✓ 不变量1：10 处 @click handler 签名未变
- ✓ 不变量2：r12Cta 自指防护逻辑仍在（R48）
- ✓ 不变量3：同类 text 版 r3-ai-cta 三件套仍在（9 页）
- ✓ 不变量4：令牌三锚点未漂移 + 10 页零 --r109-
- ✓ 不变量5：R60/R96/R99/R100/R102/R104 既有三件套抽查仍在
- ✓ 不变量6：10 个目标 vue 均可定位且无解析歧义

## 结论

- 10 失败全为真实缺口（裸 view 无 role），6 不变量全绿，RED 干净；可作为实施轮 GREEN 的基线。
- 无 PARSE_ERROR、无定位歧义（每处 ctaLine 恰命中 1 行）。
