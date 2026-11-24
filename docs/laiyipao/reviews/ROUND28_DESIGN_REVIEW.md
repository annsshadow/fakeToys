# R28 设计终审报告：实时反应归因（live reaction attribution）

- 轮次：R28（round28-live-reaction-attribution）
- 任务：t220「R28 终审：实时反应归因独立复核」（Attempt2）
- 角色：design-reviewer-wb（设计终审 / workbuddy 路由）
- 基线 HEAD：4f1a704593d3a116fd64c6821c2529f351bfe3e8
- 日期：2026-11-24
- verdict = **pass**

本报告为**独立终审**：不采信成员/QA 自述，全部结论来自本人亲读 SPEC/契约/实现/证据 JSON/截图、独立复跑测试与 type-check/build、独立 sha256 复算与 git 基线对照。

---

## 一、终审范围与六门禁口径

R28 在 R9「最近反应 feed」每条反应上追加波次/敌人归因，并增补 AI 说明与 CTA。六门禁：
1. 规范与契约一致（SPEC ↔ contract ↔ 真模块导入，RED 可失败）
2. 纯派生不改结算权威（不碰 emit/record/step/replay_hash/score/settle/loot）
3. 归因对账与诚实性（timeline 末 3 条逐字段吻合、AI 分母真实、非法/截断分支诚实标注）
4. AI 四判据 + 移动端布局（可见 AI + 真实结论 + 依据行 + CTA；溢出归属明确）
5. 真浏览器有效证据（5 图 SHA 各异亲读非登录页 + CTA WAIT 态三探针 + settle/hash 未污染 + 真实错误 0）
6. 真值零泄漏 + R1–R27 不回退 + 审计卫生

## 二、门禁① 规范与契约一致 —— PASS

- 亲读 `docs/laiyipao/specs/round28-live-reaction-attribution/SPEC.md`（65 行）：§0 缺口（R15 旁路 timeline 有 wave/enemy_uid，R9 feed 未用）；§1 三个纯函数签名 + 三逐字常量；§2 尾部游标逆序消费、命中即消费（cursor=hit-1）再校验合法性、非法游标消费 RED 用例；§3 选择器 + 逐字文案 + CSS 约束；§4 AI 四判据；§5 只读红线；§6 契约真实导入禁内嵌；§7 令牌锚点不回退。
- 亲读契约 `laiyipao/miniapp/src/pages/battle/battle_r28_live_reaction_attribution.contract.test.ts`（193 行）：从 `./liveReactionFeed` **具名真实导入** attributeLiveReactions/attributionText/buildLiveFeedAi，无内嵌实现；4 个 describe 约 21 用例覆盖匹配/不匹配/空 timeline/truncated/非法 wave+uid/非法游标消费/逐字文案/源码守卫/令牌锚点/零真值。
- 独立复跑：定向 `vitest run battle_r28_live_reaction_attribution.contract.test.ts battle.test.ts` → **2 files / 76 tests 全通过**（契约 21 + battle 55），exit 0。契约可失败性由真实模块导入 + 源码正则守卫保证（引擎侧无该函数）。

## 三、门禁② 纯派生不改结算权威 —— PASS

- 亲读实现 `laiyipao/miniapp/src/pages/battle/liveReactionFeed.ts`（126 行）：纯函数，无副作用；`attributeLiveReactions` 只读消费传入的 events/timeline，返回新 rows 数组；`isIntAtLeastOne` 校验 wave/enemy_uid 为 ≥1 整数；命中 reaction+damage 严格相等即 `cursor = hit - 1`（即使记录非法也占住游标，防重复占用），随后才校验合法性；`buildLiveFeedAi` 分母来自 `result.rows.length`（真实显示行数）。
- `engine.ts` grep：R28 无新增写权威语句；归因逻辑完全在 UI 层，不进 replay_hash。battle.vue 用量：:461 import、:1064 调用 attributeLiveReactions、:1083 attributionText、:1098/:1227 buildLiveFeedAi，均为 computed/ref 派生。
- settle 动态证据：`data/final.json` 与 `data/detail.json` 亲读 → settle status=200 / battle_id=478 / replay_hash=`6fffb426d9ee629b`，与 detail.replay_hash 一致，未污染。

## 四、门禁③ 归因对账与诚实性 —— PASS

- `data/detail.json` 亲读：id=478 / level=5 / reactions=72 / reactions_used={overheat:18, steam_burst:20, superconduct:34} / timeline.events=72 条 / truncated=false / total=72。
- timeline 末 3 条（旧→新）：tick1741 overheat enemy_uid147 damage283；tick1756 steam_burst enemy_uid147 damage302；tick1805 overheat enemy_uid148 damage439。
- samples 尾帧 feed（新→旧，逆序对应）：`第5波 · #148 / 焰+电→过热 -439`、`第5波 · #147 / 焰+冰→蒸汽爆发 -302`、`第5波 · #147 / 焰+电→过热 -283`。**逐字段一一吻合**（report §三表格 5/1741/283、5/1756/302、5/1805/439 一致）。
- AI 分母：114/114 帧 `aiConcl=已归因 3/3 条最近反应`；首帧（战斗 WAIT）与尾帧（SETTLED）均 3/3，分母为真实 DOM 行数，非硬编码。
- 诚实分支：契约覆盖 truncated→全不可用、非法 wave/enemy_uid→不可用逐字「归因不可用」、非法游标消费（最新 wave=0 非法/较早 wave=1 uid=5 合法不串位），实现 §2 逻辑与 SPEC 一致。
- 独立全量测试：`pnpm test` → **210 passed | 1 skipped files，1945 passed | 2 skipped (1947)**，exit 0。

## 五、门禁④ AI 四判据 + 移动端布局 —— PASS（含基线溢出诚实记录）

- battle.vue 模板 :62–79 亲读：每条反应 `<text class="r28-attribution" :data-r28-wave=... :data-r28-enemy=...>{{ r.attribution }}</text>`；AI 块 `.r28-feed-ai` 含标题「AI · 反应归因」、结论 `{{ r28Ai.conclusion }}`、依据行逐字、红线逐字「辅助归因非权威判据，权威以 replay_hash 为准」、CTA `查看完整反应时间线 →` `@click.stop="openHelp"`。
- 四判据：①可见汉字「AI」（标题 + cue，截图亲读可见）②真实数据结论（已归因 N/M 来自真实行数）③依据行逐字 ④可执行 CTA（见门禁⑤ WAIT 态真点击证据）。
- CSS：`.r28-attribution` font-size 18rpx nowrap（≤20rpx）；`.r28-feed-ai` max-width:100% / pointer-events:none 可换行；`.r28-feed-cta` pointer-events:auto。R24/R26/R27 选择器共存（samples 全程 hasR24=true、hasR27=true）。
- **6px 溢出裁定（队长质疑 2，明确结论：非 R28 引入，非回归，放行但如实记录）**：
  - 现象真实：feed right=380/scrollWidth=386/clientWidth=369；3 条 `.reaction-line` right=**396**（超 390 视口 6px）、width=386、max-width=none。docOverflow=false，无页面级横向滚动。
  - 归属证据：`git show HEAD:...battle.vue` 的 HEAD 基线 CSS（2181–2235）已含 `.reaction-line{display:flex;flex-direction:row;gap:12rpx;...}` 与媒体查询块（.reaction-line width:100%）；`git diff` 仅 53 insertions / 6 deletions，新增 CSS 仅 `.r28-attribution`/`.r28-feed-ai`/`.r28-feed-cta`，**未改 `.reaction-line`/`.reaction-feed`/`.reaction-name`/`.reaction-damage`/`.reaction-cue` 任何宽度规则**。
  - 判定：该几何为 HEAD 基线既有，R28 未引入也未加剧（R28 归因 text 用 nowrap 18rpx 但受父容器约束，肉眼截图无截断）；按非阻塞既有问题记录，建议后续单独修复，不作为 R28 回归缺陷。

## 六、门禁⑤ 真浏览器有效证据 —— PASS

- 5 图 SHA 本人 `sha256sum` 亲算，与 REPORT §七逐字一致、两两不同：
  - r28-01-attribution-multi.png = `9216925c7f1277f4cac7c39bb309f8f2ac1dd6e370c876c77fc14e0d05ea303f`
  - r28-02-feed-ai.png = `dc02d8b6d0532d6acf1597d3d5191a2c7c0f81fde89fdd427833bb5adc84c9f2`
  - r28-03-cta-help.png = `b6fd9ccc266bf8f9fa1086af9490b569dede6e2e0934cfe7892b9dfcc1774f8e`
  - r28-04-settle.png = `321b20955f87d1828660a14c74f68c12ba67cb13828c773fdcdc9441bf10ad8e`
  - r28-05-cta-open.png = `dc6a59cc8921d4be1c51914d802cc8ef664e8b284b606e6e8d20aad7a7de39b1`
- 亲读 r28-01 / r28-02 / r28-05：均为 390×844 战斗页（顶部 HUD「第5关·边缘站·5」+ 反应 feed 逐条「第1波 · #1」归因 + AI 反应归因块），非登录页；r28-05 为真实「战场图例」覆盖层（热量条/技能槽/元素反应说明），非结算页。r28-03 是结算态 CTA 时机错误的**失败样本**，QA 如实保留未冒充通过。
- CTA WAIT 态三探针（队长质疑 1，**结论：CTA 有效**）：独立会话 `lyp-r28-cta` 在战斗 WAIT 态 CLI 真点击 `.r28-feed-cta` → `data/cta-pre.json`(helpVisible=false) → `data/cta-post.json`(true) → 点 `.legend-close` → `data/cta-closed.json`(false)，三探针 state 均 WAIT / lineCount=3 / 归因行一致；measure-eval.js 量测 CTA 命中 SPAN 文本节点，CLI click 有效。主 E2E 结算态点击 pre/post helpVisible 恒 false 属测试时机错误，已排除为假阴性。
- 真实错误：`data/final.json` pe=51 全为 autoplay-interrupted 噪声（探针过滤后 peReal=0），ce=0 / af=0 / fr=0。114 帧 samples 全 lineCount=3、hasUndefined=false、hasNaN=false、docOverflow=false。
- 权威哈希：settle 200 / battle_id=478 / replay_hash=`6fffb426d9ee629b`，与 detail 一致，未污染。
- 诚实边界：agent-browser 无头、本地回环 127.0.0.1，非物理真机。

## 七、门禁⑥ 真值零泄漏 + R1–R27 不回退 —— PASS

- 零真值 OPS 扫描审计目录：JWT（eyJ 长串）0、DSN（postgres://）0、私钥（PRIVATE KEY）0、root@IP 0；Bearer 2 处命中均为**变量/描述**（`Authorization: Bearer $TOKEN` 变量名，HEAD 基线 round14/19/20 审计脚本已有同模式；REPORT:107 为「0 命中」描述文字），非真值。
- auth-init：仅 `auth-init.tpl.js` 占位模板（第 2 行 `localStorage.setItem('lyp_token','__LYP_TOKEN__')`），含真 JWT 的 auth-init.js 已删（ops_glob 查无）。
- `.gitignore:339-340` 命中 data/ 与 auth-init.js；报告称 QA 库 pg_restore 还原 battle_records 104/max_id=477（测试前一致）——属 QA 操作，本终审未改业务源码未 git。
- 令牌锚点 `tokens.css:14-16`：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 保留，无 `--r28-`（grep 0 命中）。
- R15/R24/R26/R27 共存契约未回退（定向 + 全量 1945 测试绿佐证）。

## 八、独立复跑汇总

| 项 | 命令 | 结果 |
|---|---|---|
| 定向契约+battle | pnpm exec vitest run battle_r28_live_reaction_attribution.contract.test.ts battle.test.ts --config vitest.config.ts | 2 files / 76 tests 全绿，exit 0 |
| 全量测试 | pnpm test | 210 passed / 1 skipped files；1945 passed / 2 skipped (1947)，exit 0 |
| 类型检查 | pnpm type-check（vue-tsc --noEmit） | exit 0 |
| H5 构建 | pnpm build:h5（uni build） | DONE Build complete，exit 0 |
| Go 真值守卫 | go test ./internal/domain/... -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo' | TestNoInfra PASS(8.82s)；TestNoRealSecrets 因本机无 deploy/.local.keys 诚实 SKIP（守卫自述「没有执行，不是通过了」），exit 0 |
| 五图 SHA | sha256sum ×5 | 与 REPORT §七逐字一致、两两不同 |
| git 基线 | git show HEAD:battle.vue + git diff | 53+/6-，未改 reaction 行宽度规则 |

## 九、结论与保留事项

六门禁全部 PASS，判定 **verdict = pass**。两项队长质疑均获决定性证据裁定：①CTA 在战斗 WAIT 态三探针 false→true→false + 覆盖层截图有效，主 E2E 结算态点击属时机错误已排除；②feed 6px 溢出为 HEAD 基线既有、R28 未改宽度规则，非回归，按诚实原则记录并建议后续单独修复。

保留事项（非阻塞、不影响 verdict）：
- 基线 6px `.reaction-line` 右溢出（HEAD 既有，建议后续轮次单独修）。
- Go `TestNoRealSecretsInRepo` 因本机缺 `.local.keys` 未执行（诚实 SKIP），以 OPS 扫描 + TestNoInfra PASS 补强；不宣称该守卫「通过」。
- agent-browser 无头、本地回环，非物理真机。
- 工作树 R28 相关文件未 git（按 QA 只读边界）。
