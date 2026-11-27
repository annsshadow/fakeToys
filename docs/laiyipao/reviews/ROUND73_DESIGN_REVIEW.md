# R73 战场图例弹层焦点管理 · 独立设计终审

- 审阅角色：same-route-reviewer（独立终审，agent-29）
- 审阅对象：t153 实施（reviewed task=t158 链）；本轮 t160，attempt=1，attempt_id=64a419e4-d61c-4241-96aa-fdf36c1faecb
- 基线 HEAD：def111dd8
- 审阅性质：纯只读六维核验 + 门禁独立复跑，未改任何业务源码/契约/规范/.gitignore，未写真值、未写 git
- 核心变更：battle.vue 战场图例弹层补齐 WAI-ARIA dialog 语义 + 焦点移入/归焦 + ESC 关闭 + Tab 陷阱 + 键盘可达入口，复用 R60 键盘范式（role=button + tabindex=0 + @keydown.enter）
- 结论：**verdict=pass**，六维全绿，无 blocker/high/medium finding，可发布。

## 维度一 · test-first 真实性

- 契约 laiyipao/miniapp/src/styles/r73LegendFocus.contract.test.ts（101 行 / 4953B / 6 it）亲读：真实 `node:fs readFileSync(join(__dirname, '..'))` 读 battle.vue 与 tokens.css，无内嵌副本；`blockAfter` 大括号配对提取函数块，`viewTagBlock` 按 class 定位开标签块，均为源码语义判据而非文本顺序硬编码。
- 六条判据：it1 overlay 含 role=dialog + aria-modal=true；it2 help-chip 与 legend-close 均 role=button + tabindex=0 + @click 复用 + @keydown.enter 复用各自 handler；it3 openHelp 含 nextTick + focus、closeHelp 含 focus；it4 ESC 守卫 + Tab 陷阱守卫；it5 doPause/doResume/helpPausedByPage + closeReactionTimeline + 令牌三锚点；it6 battle+tokens 零 --r73-。
- RED 侧：SPEC §5 明文预期 exit=1、4 failed（it1–it4）/ 2 passed（it5/it6），缺口恰为四处新能力断言，it5 不变量与 it6 零令牌在 RED 阶段即通过——符合「新行为失败、既有不变量通过」的正确 RED 形态。原始 RED 日志未留存，按 SPEC §5 + 契约结构 + GREEN 独立复跑三重采信，不伪造日志。
- GREEN 侧：本轮独立复跑 RV-G1 契约 6/6 passed（RC=0），确认 RED→GREEN 语义闭环成立。
- 判定：通过。

## 维度二 · 实施正确性

- git diff HEAD --stat：仅 `.gitignore`(+6) 与 `laiyipao/miniapp/src/pages/battle/battle.vue`(+42/-3)；`git diff --name-only` 仅此两文件，写域精确。
- help-chip（约 :112–120）：加 `ref=helpChipRef` / `role=button` / `tabindex=0` / `@keydown.enter=openHelp`，保留原 `:class` 与 `@click` 与 aria-label——R60 逐字范式。
- legend-overlay（约 :177–185）：开标签加 `ref=legendOverlayRef` / `role=dialog` / `aria-modal=true` / `aria-label=战场图例` / `@keydown.esc=closeHelp` / `@keydown.tab=trapLegendTab`，保留 `v-if` 与 `@click`。
- legend-close（约 :185）：加 `ref=legendCloseRef` / `role=button` / `tabindex=0` / `@keydown.enter=closeHelp`，保留 `@click` 与 × 文案。
- 脚本区（:482 起）import nextTick；:980 附近新增三个 `ref<any>(null)`，注释明示 H5 端 view ref 取组件实例 $el 为原生节点的兜底。
- openHelp 末 `nextTick(() => { const el=legendCloseRef.value; const node=(el&&el.$el)||el; if(node&&typeof node.focus===function) node.focus(); })`；closeHelp 末以 helpChipRef 同样方式归焦；新增 trapLegendTab(e) 用 legendOverlayRef 取 root 的 `[tabindex=0], [role=button]` 首尾，Shift+Tab 在首/外部跳末、Tab 在末/外部跳首并 preventDefault。函数块 :975–1045 亲读与 diff 完全一致。
- 判定：通过。焦点移入/归焦用 nextTick + $el 兜底是 H5 uni view ref 的正确取法，焦点陷阱首尾闭环且 preventedDefault，不会横跳。

## 维度三 · 不变量与外科写域

- doPause / doResume / helpPausedByPage 记账逐字未动；openHelp 先 `closeReactionTimeline()` 的 R29 互斥未动；helpSeen 记忆未动；closeHelp 的 doResume 未动。helpPausedByPage 仍为 let 普通变量，doPause 在 `!paused.value && !settled.value` 时调用并置 true——手动暂停分支可正确恢复。
- 令牌三锚点 tokens.css:14/15/16（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）逐字原样。
- 全 `laiyipao/miniapp/src` 扫描 `--r73-` 仅命中契约 r73LegendFocus.contract.test.ts:99 的断言 1 处，业务源码 0——未新增视觉令牌。
- 保护域 engine/replay/hash/score/settle/loot/tokens 与后端零改；契约/SPEC/.gitignore 中 .gitignore 仅本轮既有 +6（历史轮次同款规则），非本轮新增视觉令牌。
- 判定：通过。

## 维度四 · QA 证据可信

- QA 报告 docs/laiyipao/audits/round73-legend-focus-trap/REPORT.md（t159 attempt 18efeab1-ba5a-438c-8a46-434f98323768，判定 PASS）：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800，真实点击 + 真实 press。注意 QA 的 hOverflow 探针 `true=无溢出`（沿历史命名，勿反读）。
- 三视口核心行为：打开前焦点 BODY、弹层关；点 help-chip 后焦点 legend-close（UNI-VIEW 文本 × 在弹层内）、弹层开、role=dialog、aria-modal=true、暂停 true、无横溢；ESC 后焦点 help-chip、弹层关、暂停 false。Tab 与 Shift+Tab 各视口 6 次，activeInOverlay / overlayOpen / paused 恒 true。真键盘 Enter 双通道开-暂停-关-恢复；手动暂停分支开图例再 ESC 后 paused 仍 true。
- R29 互斥：点 .r28-feed-cta 后 overlayOpen true→false 且焦点归 help-chip；诚实边界注明未捕获抽屉展开态 DOM（drawerOpen=false），故不宣称抽屉已展开——不夸大。
- 7 图 sha256 亲算与报告逐字一致，UNIQUE=7/7：390-open=3d9c053f…、390-closed=ebe1606a…、768-open=0093c0eb…、768-closed=bea370f6…、1280-open=61b386cb…、1280-closed=752421e2…、trial-open-768=dcdd3a5d…（首轮额外证据非三视口正式组）。
- 亲读 4/7 张（均非登录页，三视口弹层渲染正确）：r73-768-open（战场图例标题 + 关闭× + 热量条/技能槽说明完整）、r73-1280-closed（战场主界面波次/技能/AI诊断/分数512/热量0/100）、r73-390-open（窄屏弹层完整可见无裁切）、r73-1280-open（宽屏弹层居中完整）。其余 3 张仅列 sha，未逐张亲读。
- 首轮 dispatchEvent(KeyboardEvent) 未触发 Vue .enter 已作废重 press——QA 如实披露。
- 判定：通过。行为细节、sha、诚实边界三者自洽。

## 维度五 · 门禁独立复跑

- 独立脚本 docs/laiyipao/audits/round73-legend-focus-trap/data/review-gates.sh（data/ 命中 .gitignore:592，未入库），以 ROOT 绝对路径切 cwd，输出 review-gates.log（764210B）。后台 job bash-14fadf68a0314136 已 completed，无 running 残留。
- RV-G1 契约 r73LegendFocus：6 passed，RC=0。
- RV-G2 battle.test.ts：61 passed，RC=0。
- RV-G3 全量 vitest：224 files passed | 1 skipped (225)，2055 tests passed | 2 skipped (2057)，RC=0。
- RV-G4 vue-tsc --noEmit：RC=0。
- RV-G5 build:h5：DONE Build complete，RC=0。
- RV-G6 git diff --check：RC=0。
- RV-G7 Go TestNoInfraIdentifiersInRepo：--- PASS (11.75s)，RC=0。
- RV-G8 Go TestNoRealSecretsInRepo：RC=0 但 **--- SKIP 非 PASS**，逐字「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」，如实标注，不冒认通过。
- 判定：通过（含诚实边界）。

## 维度六 · 保护面零回退与 hygiene

- R60 键盘范式：全 `**/*.vue` role=button 51 / tabindex=0 51 / @keydown.enter 50；battle.vue help-chip:117/118/120、legend-close:185 齐备（:1030 为 trapLegendTab 内 querySelector 字符串被 grep 计入，非模板元素）。QA 报 48 为基线口径，差异系统计范围不同，非回退。
- R61 aria：verify.vue v-aria-native / aria-label 保留；R55 placeholder-class=ph 保留；R71 fmtVerifyTime 2 命中；R72 search-empty-text 9 文件在；R66 normalizeGemGain 4 文件在；R69 nicknameSubmitting 1 文件在；R48 battles.vue #r14-advice-action-btn 与 applyAdviceAction 未动——均无回退。
- hygiene：本轮 round73-legend-focus-trap 目录内 auth-init.js / admin-auth-init.js glob=[] ABSENT；git status 非 ?? 项=2（.gitignore 与 battle.vue 为本轮实施 tracked 改动，预期）；QA 库 docker lyp-qa-pg 只读查询 battle_records count|max=104|477；git diff --check RC=0；零真值入库回显。
- 判定：通过。

## 非阻断观察

1. RED 原始日志未留存：依 SPEC §5 预期 + 契约结构 + GREEN 独立复跑三重采信，不伪造。
2. 7 张截图仅亲读 4 张；其余 3 张仅核 sha，未逐张目视。
3. Go TestNoRealSecrets SKIP 非 PASS：真值扫描守卫未执行，非通过。
4. R29 互斥复查未捕获抽屉展开态 DOM（drawerOpen=false），不宣称抽屉已展开。
5. QA gates-out.txt 中大量契约测试名含「RED」字样系历史轮次测试命名，非本轮失败。

## 结论

六维全绿，无 blocker/high/medium finding。R73 战场图例弹层焦点管理实现符合 SPEC 口径，test-first 真实、外科写域精确、不变量完整、QA 证据可信、门禁独立复跑全绿（TestNoRealSecrets SKIP 如实标注）、保护面零回退。**verdict=pass，可发布。**
