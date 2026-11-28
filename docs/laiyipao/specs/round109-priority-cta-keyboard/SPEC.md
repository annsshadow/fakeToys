# R109 规范：跨页 AI 优先级引导 CTA 键盘可达

## §0 背景与裁定

- 来源：R109-A 侦察（docs/laiyipao/recon/ROUND109_A_PRIORITY_CTA_KEYBOARD_RECON.md，155 行，四条件全中）。R109-B 无候选。
- 裁定：立 R109，纳入全部 10 处裸 view CTA。这是「真实功能不可达」——键盘用户无法执行 AI 引导卡「去补强玩法」唯一入口（真 Tab 探针坐实跳过，AX 无 button），区别于 R108-A 的语义堆砌。
- 基线 HEAD=652e237fbdf5ce38460f1f159e5b48a6ec9c7646。
- 权威：AI_VISUAL_SYSTEM.md:137（CTA 键盘可达且焦点可见 WCAG 2.4.7）、:926（§9 底线不可协商）、:66（可访问性优先）、:120（给卡内 CTA 焦点）；WCAG 2.1.1；同仓已修 text 版 9 处三件套范式。无反向权威。

## §1 写域

- 唯一写域：docs/laiyipao/specs/round109-priority-cta-keyboard/{SPEC.md,RED-evidence.md,RED-raw.log} + laiyipao/miniapp/src/styles/r109PriorityCtaKeyboard.contract.test.ts。
- 禁改：业务 vue、.gitignore、令牌、既有契约。

## §2 改点清单（10 处，逐处覆盖）

每处补 `role="button" tabindex="0" @keydown.enter="同handler"`（复用现有 @click 同 handler 同参数，不新增函数）：

| # | 文件:行 | class | handler |
|---|---|---|---|
| 1 | gems/gems.vue:18 | r12-priority-cta | r12Go() |
| 2 | skills/skills.vue:17 | r12-priority-cta | r12Go() |
| 3 | equipment/equipment.vue:17 | r12-priority-cta | r12Go() |
| 4 | bag/bag.vue:12 | r12-priority-cta | r12Go() |
| 5 | me/me.vue:61 | r3-ai-cta | r4GoMe() |
| 6 | bag/bag.vue:33 | r3-ai-cta | r4GoBag() |
| 7 | battles/battles.vue:21 | r3-ai-cta | r4GoBattle() |
| 8 | enemies/enemies.vue:17 | r3-ai-cta | r4FocusEnemy() |
| 9 | rank/rank.vue:21 | r3-ai-cta | r4GoRank() |
| 10 | skins/skins.vue:22 | r3-ai-cta | r4FocusSkin() |

## §3 契约设计

- 契约文件：laiyipao/miniapp/src/styles/r109PriorityCtaKeyboard.contract.test.ts（114 行，16 it）。
- 真实 fs.readFileSync 读 10 个 vue，按 class + @click handler 双锚点定位开标签单行（不依赖行号）；断言命中恰为 1 行，防解析歧义。
- 10 缺口 it：每处断言 role="button" + tabindex="0" + @keydown.enter="同handler" 三属性齐全。
- 6 不变量 it：
  1. 10 处 @click handler 签名未变（自指防护与导航参数不动）。
  2. r12Cta 自指防护逻辑仍在（bag/equipment/gems/skills 四处 if (r12Cta.value.self)；R48 契约 r12GrowthPriority.contract.test.ts:197-235 锁死）。
  3. 同类 text 版 r3-ai-cta 三件套仍在（9 页 >=9 处：announcements:36/equipment:127/gems:69+70/mastery:36/reactions:103/shop:60/signin:33/skills:145/stage:67）。
  4. 令牌三锚点 tokens.css:14-16（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）不变 + 10 页零 --r109-。
  5. R60/R96/R99/R100/R102/R104 既有三件套抽查仍在（index retry/ai-cta、me back-btn、bag r3-empty-action、enemies resist-toggle、announcements aria-busy）。
  6. 10 个目标 vue 均可定位且无解析歧义（ctaLine 命中恰 1 + template 存在）。

## §4 RED 验证口径

- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r109PriorityCtaKeyboard.contract.test.ts --reporter=verbose`。
- 预期 RED：exit=1，Tests 10 failed | 6 passed (16)，10 失败全为 role="button" 缺失真实缺口，6 不变量全过，无 PARSE_ERROR。
- 完整原始输出：docs/laiyipao/specs/round109-priority-cta-keyboard/RED-raw.log（198 行）；摘要与意图：RED-evidence.md。
- GREEN 阶段由实施轮补三件套后复跑，预期 16 passed。

## §5 与既有轮次不重复论证

- R96 修 `.r3-empty-action` 空态 CTA（6 页）；本轮是 `.r12-priority-cta` + `.r3-ai-cta` view 版（10 处），class 完全不同。
- R99 修 index 链接、R100 修 battle-row/ann-item/resist-toggle 等、R102 修 index CTA；本轮是跨页 AI 引导卡，改点页面与 class 均不重叠。
- R104 是 loading aria-busy（语义），本轮是键盘功能可达；R108-A 是进度条语义（判不立轮），本轮判立轮。
- 五者同属键盘/可访问性大主题但改点互不重叠，是本批次系统性遗漏的分批收口。

## §6 hygiene

- HEAD 未变、tracked diff=0（.gitignore R109 四行为上游预置）、业务 vue/令牌/既有契约零改动、零真值、自启 job 回收。
