# R44 规范：R22 协同预判 CTA 文案—行为一致性（保守派对齐只读）

> 目标：让 R22 协同预判区的「行动号召」文案与真实交互一致——把按钮措辞改为只读操作指引，不改任何行为。
> 本轮只交付规范与契约测试；battle.vue 的文案落地留实施轮（test-first RED）。零 engine / replay_hash / score / settle / loot / tokens 改动。

## 0 缺口与权威证据（亲读源码，带行号）

- docs/laiyipao/specs/round22-card-reaction-synergy/SPEC.md:55 原逐字：『4. 可执行 CTA：去装备/选它（点选该卡即装备，或引导去装备页）。』——SPEC 明确承诺「可执行」。
- laiyipao/miniapp/src/pages/battle/battle.vue:364 实际模板：`<text class="r22-ai-cta">去装备/选它</text>`——无 @click。
- laiyipao/miniapp/src/pages/battle/battle.vue:2756 `.r22-ai-cta { color: var(--accent); font-size: 20rpx; font-weight: 700; }`——无 pointer-events:auto。
- 结论：该行是不可点的纯文本，却用按钮措辞诱导点击；玩家把全局解读区这行字误当按钮却点不动。
- 相邻的权威事实（证明「点卡即装备」已由卡牌本体承担，无需此行代劳）：
  - battle.vue:340 注释逐字：『R22 选卡反应协同预判：纯展示派生，不改卡牌点击/弃牌行为』。
  - battle.vue:314-321 卡牌本体已整卡 `@click="takeCard(c.id)"`（点卡即装备）。
  - battle.vue:286 已有提示『这一波给你 3 张卡，点一张就装备上』。
- 测试盲区：battle_r22_synergy.test.ts:196-209 与 battle_r22_synergy.contract.test.ts:56-68 只测文案字样/SPEC 字样，不校验措辞是否承诺可执行动作。

## 1 保守派决策与理由（队长裁定）

- 决策：选**保守派**——文案与 SPEC 对齐「只读操作指引」现实，不改任何行为。
- 理由一：行动派（给该行加 @click 自动选卡）会替玩家做选卡决策，改变核心策略判断，触碰 gameplay；本项目硬约束禁改 engine / replay_hash / score / settle / loot / tokens。
- 理由二：R43 已建立「CTA 退化为只读」先例（R27/R24 的 openHelp 错配改为纯提示），本轮沿用同一口径，不发明新形态。
- 理由三：「点卡即装备」本就由卡牌本体 takeCard 承担（battle.vue:314-321/:286），该行只需做方向指引，无需承接点击。

## 2 逐字文案与判定口径

- 新文案（逐字）：`↑ 点上方卡牌即可装备它 →`
  - 不得包含旧按钮措辞 `去装备/选它`。
  - 必须明确指向「上方卡牌」「即可装备」，即引导玩家点选卡牌本体（takeCard 即装备）。
  - `.r22-ai-cta` 行不得出现 `@click`，CSS 不得出现 `pointer-events: auto` / `pointer-events:auto`。
- AI 四判据第 4 条改写为「只读操作指引」：文案引导玩家点上方卡牌；本提示只读、不可点、不自动选卡。R22 SPEC.md:55 就地修正为同一口径。
- 其余三段（可见 AI / 真实数据结论 / 逐字依据行）与诚实红线逐字不变，继续沿用 §3。

## 3 契约边界与零行为改动

- 本轮写域：本 SPEC + R22 SPEC.md:55 就地修订 + 两个契约测试文件。禁改 battle.vue 实现本体（文案落地留实施轮）。
- 不改 takeCard / discardCard / skipCards / hand / showCards / elementName 任何逻辑。
- 不改 REACTION_PAIRS / lookupReaction / 派生算法；不进 replay_hash、不改结算、不写权威字段。
- 令牌三锚点保留：`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`；禁新增 `--r44-` 令牌。

## 4 诚实红线与零真值

- 辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准（逐字保留）。
- 只读指引不自动选卡、不替玩家决策；玩家的策略判断由点选卡牌本体完成。
- 零真值：不得出现口令、令牌、真实域名/IP、个人路径与本机环境信息；示例值不硬编码进展示。

## 5 契约测试与 RED 门禁

- battle_r22_synergy.contract.test.ts 追加 R44 守卫：读真实 battle.vue，定位 `class="r22-ai-cta"` 行，断言 not.toContain('@click')、not.toContain('去装备/选它')、toContain(新指引措辞)；断言 `.r22-ai-cta` CSS 无 pointer-events:auto；读 R22 SPEC 断言 not.toContain('可执行 CTA')。
- battle_r22_synergy.test.ts 追加同源源码守卫（复用 R43 的 ctaLine / cssRule 范式）。
- 禁止内嵌参考实现：必须从真实文件逐行读取。
- RED 门禁（battle.vue 仍为旧文案，必红）：在 laiyipao/miniapp 执行
  `pnpm exec vitest run src/pages/battle/battle_r22_synergy.test.ts src/pages/battle/battle_r22_synergy.contract.test.ts --config vitest.config.ts --reporter=dot`
  期望 exit=1，失败来自新增 CTA 措辞断言；不得是解析/模块缺失/超时。
- GREEN（实施轮）：仅把 battle.vue:364 文案替换为 §2 逐字新文案；不动任何其他行。

## 6 修订轨迹

- R22 原 SPEC.md:55「可执行 CTA：去装备/选它（点选该卡即装备，或引导去装备页）」→ 本轮就地改为「只读操作指引」口径：文案引导玩家点上方卡牌即可装备（takeCard 即装备），本提示只读、不可点、不自动选卡，避免改变玩家策略判断；不再使用「去装备/选它」按钮措辞。
- R44 新增本 SPEC 固化保守派决策与理由、逐字文案、契约边界、RED 门禁。
- 既有 AI 四判据前三段、诚实红线、选择器契约、三端不回退锚点一律不动。

## 7 实施分工（下一轮）

- 实施：仅替换 battle.vue:364 的 `.r22-ai-cta` 文案为 §2 逐字新文案；零行为改动。
- 验证：真浏览器打开选卡界面，确认该行为只读提示、点上方卡牌即装备、无暂停/自动选卡副作用；RED→GREEN 后两测试全绿。

## 8 三端不回退锚点

- miniapp/src/styles/tokens.css 三锚点保留，禁新增 --r44-。
- battle.vue 选卡基线（hand/showCards/elementName/takeCard/discardCard）与 REACTION_PAIRS import 保留，本轮只改一行文案措辞。
- R15–R43 既有契约（含 R27/R24 CTA 只读先例）一律不动。
