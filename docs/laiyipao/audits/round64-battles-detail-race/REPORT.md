# R64 真浏览器 QA：战报详情辅助面板竞态复查报告

- 任务：t120 / attempt 8dcdee28-58f0-48df-ba33-2ece270ccc66
- 范围：只读复查，工作目录 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248，HEAD=932106f19
- 零真值：全文不含 token/口令/私人路径；鉴权仅在 gitignored data/ 目录会话侧注入
- 结论：**FAIL（reject）**。R64 静态契约 5/5 绿，但真 Chromium 构造「A 先发、B 后发、A 响应晚到」后，辅助面板 growthCompare 仍归属 A（stage5 10411→8888），未归属当前展开的 B（stage40 19925）。硬门禁①「展开 B 时 growthCompare/r14Advice 归属 B 非 A」未满足。

## 0. 判定依据（一句话）

守卫 `if (seq !== detailPanelSeq) return` 写在两个 loader 的**入口、await 之前**，只在进入函数那一刻校验一次；await 恢复后写 `growthCompare.value` / `r14Advice.value` 前**不再复核**，故旧响应晚到仍覆盖。契约 it1 用 `block.slice(0, wi)` 只做文本顺序检查（guard 文本位于写入前即 pass），无法检出「await 后未复核」，属静态假绿。

## 1. 实施 diff（QA 只读核验）

- `git diff --stat`：`.gitignore` 4+ / `laiyipao/miniapp/src/pages/battles/battles.vue` 13+/4-。
- `battles.vue:379-380` `let detailPanelSeq = 0`。
- `battles.vue:496-507` `async function loadGrowthCompare(d, seq?)`：`:497` 入口 seq 守卫；`:500` `const history = await api.myBattleHistory(20)`；`:503` `if (priorId) baseline = await api.myBattleDetail(priorId)`；`:507` `growthCompare.value = baseline ? normalizeGrowthCompare([baseline, d]) : normalizeGrowthCompare([])`（await 后无复核）。
- `battles.vue:770-775` `async function loadR14Advice(d, seq?)`：`:771` 入口 seq 守卫；`:775` `r14Advice.value = await fetchAdviceAction(...)`（await 后无复核）。
- `battles.vue:830-845` `loadDetail`：`const seq = ++detailPanelSeq`、按 id 缓存短路、`const d = await api.myBattleDetail(it.id)`、`await loadGrowthCompare(d, seq)`、`void loadR14Advice(d, seq)`。
- 契约 `laiyipao/miniapp/src/styles/r64BattlesRace.contract.test.ts` 88 行 5 it；`SEQ_GUARD=/\b\w*[Ss]eq\b\s*!==?\s*\b\w*[Ss]eq\b/`，it1 仅文本顺序检查。
- SPEC `docs/laiyipao/specs/round64-battles-detail-race/SPEC.md` 56 行，§2 明确要求旧响应晚到时丢弃。

## 2. 竞态构造（真 Chromium）

- 服务：API 8080（start-api.sh，`/api/v1/config`=200 version=6237e0b623219b8b）；静态 H5 5201（bundled python http.server --directory laiyipao/miniapp/dist/build/h5）；旧 5174 为 302 服务，已剔除。
- 鉴权：users.id=10 guest_token 经 POST /api/v1/auth/guest 换 token，gitignored `data/auth-init.js` 会话侧注入，不入库。
- 延迟注入 `race-init2.js`（patch window.XMLHttpRequest，rules：`{match, ms, remaining}`）；`race-set2.js` 设两条规则：`/me/battles?limit=20&offset=0` 延迟 3000ms×1、`reaction=superconduct` 延迟 3000ms×1，并记 send/done 时戳于 window.__raceLog。
- 操作序列：进入 `#/pages/battles/battles` → 点 A（378，stage5，10411）→ 0.4s 后点 B（41，stage40，19925）→ 等 4.2s → `r64-probe.js` 读 DOM。
- 三视口 390×844 / 768×1024 / 1280×800 全部复现同一错配（DOM witness：`data/race-multi-390.txt`、`data/race-final-witness.txt`、`data/race-multi-1280.txt`）。

## 3. 请求时序铁证（768，`data/race-final-witness.txt` raceLog）

| 时戳(ms) | 事件 | 说明 |
|---|---|---|
| 1795686978493 | send /me/battles/378 | A 详情 |
| 1795686978501 | send /me/battles?limit=20&offset=0 | A 的 history，被延迟 3000ms |
| 1795686978501 | done /me/battles/378 | A 详情返回 |
| 1795686978958 | send /me/battles/41 | B 详情 |
| 1795686978964 | send /me/battles?limit=20&offset=0 | B 的 history |
| 1795686978964 | done /me/battles/41 | B 详情返回 |
| 1795686978969 | send /me/advice/action?...reaction=overheat | B 的建议请求 |
| 1795686978971 | done /me/battles?limit=20&offset=0 | B 的 history（未被延迟，remaining 已耗尽） |
| 1795686978990 | done advice/action | B 建议返回 |
| **1795686981517** | **send /me/battles/477** | **A 的延迟 history 恢复后仍去找基线（477=8888）——若守卫生效该请求根本不应发出** |
| 1795686981523 | done /me/battles/477 | A 基线返回，覆盖 growthCompare |

结论：A 的旧响应在 3s 后恢复，入口 seq 守卫当时已通过（进入时 seq 尚等于 detailPanelSeq），恢复后无复核 → A 的 history/baseline 继续跑完并写入 ref。

## 4. 面板归属证据（三视口一致）

- 主详情（R63 按 id 缓存）：`expandedHead="胜 ★ 第 40 关 19925"`、`detailHash=da91c7bf08b63fa4`、kv 含「回放哈希 da91c7bf08b63fa4」——**B 归属正确，R63 无回退**。
- 辅助面板（错配）：`growthText` 含「AI · 养成前后（同一关可信差值） 同一关第 5 关 · 两次真实结算的可信差值 分数 10411.0 → 8888.0 -1523.0」；`growthHashes="hash：7468252a57f6befe / 8ddc5cc01e787ec9"`——这是 A（378/477，stage5）的配对，**不是 B（stage40）**。
- B 的 stage40 正对照本应显示「还缺养成前那局战报，先打一局同关」或 stage40 配对；实测被 A 覆盖。
- `r14Text` 为空态「AI 暂时没有足够数据给出建议 ... 去打一场战斗 →」——本样本未坐实 r14Advice 错配（B=stage40 建议请求返回后无更优候选，覆盖与否不可从文本区分）；不虚构，如实标注。
- 三视口 DOM witness 关键字段一致：390/1280 的 `expandedHead` 与 `growthText` 与 768 完全相同；竞态与视口无关。

## 5. 单场展开正对照（通过）

- `data/race-ref-i4.txt`、`data/race-paired-i4.txt`：单独展开 stage40（41，19925），`expandedHead` 正确、`detailHash=da91c7bf08b63fa4`、`growthText="还缺养成前那局战报，先打一局同关"`（诚实空态）、`r14Text` 空态。
- 说明单场展开链路正常；缺陷只在「快速切换 + 旧响应晚到」的竞态窗口。
- CTA 链路未破坏：`battles.vue:282` `<text id="r14-advice-action-btn" class="r14-advice-cta" role="button" data-r14-source="battle" @click="r14OpenCompare()" tabindex="0" @keydown.enter="r14OpenCompare()">一键替换为 {{ r14Advice.candidateName }} →</text>` 保留，R60 键盘属性在位。

## 6. 回归与不变量（通过）

- R63 主详情按 id 缓存与 computed detail 绑定保留（见 §4 detailHash 正确）。
- R60 键盘：全 src `role="button"`=48、`tabindex="0"`=48、`@keydown.enter`=48，一一对应，无回退。
- R55 placeholder：全 src `placeholder-class="ph"`=17 处保留。
- 令牌三锚点 `laiyipao/miniapp/src/styles/tokens.css:14-16` 逐字在：`--lyp-token-version: lyp-tokens-v1;` / `--lyp-r3-version: lyp-tokens-v3-r3;` / `--lyp-r6-version: lyp-tokens-r6;`；全 src `--r64-` 0 命中。
- 业务 diff 仅限 `battles.vue`（+13/-4）与 `.gitignore`（+4），QA 未改业务源码。

## 7. 门禁（全绿，但 TestNoRealSecrets 如实 SKIP）

| 门禁 | 结果 |
|---|---|
| R64 契约 `r64BattlesRace.contract.test.ts` | 5/5 passed，RC=0 |
| R60 契约 `r60A11yKeyboard.contract.test.ts` | 6/6 passed，RC=0 |
| miniapp 全量 vitest | 219 files passed / 1 skipped；2030 tests passed / 2 skipped，RC=0 |
| vue-tsc type-check | RC=0 |
| build:h5（cwd=laiyipao/miniapp） | DONE Build complete，RC=0 |
| git diff --check | RC=0（仅 .gitignore LF→CRLF warning） |
| Go TestNoInfraIdentifiersInRepo | PASS（8.22s），RC=0 |
| Go TestNoRealSecretsInRepo | **SKIP，非 PASS**：逐字「读不到 .../laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」；Go 命令整体 RC=0 |

## 8. 截图与诚实边界

- 竞态截图：`screenshots/race-768-final.png` 与 `race-768-afterB.png`，sha256 均为 `d5482858d5c63f031479bd29bddb6f28b312434cee16775919ed6da0852832b1`（**两张同 sha**，均为展开 B 后同页同状态，预期合理但**不满足「截图全互异」**），已 `read_image` 亲读：非登录页，显示「我的战报」真实业务内容（0% 防守扎实、313 总击杀、AI 防守诊断、看战报详情 CTA、战绩总账）。
- 390 / 1280 视口：DOM witness 已产出并坐实同一错配；**截图步骤因 driver 挂起未落盘**（`race-multi-driver.sh` 在截图阶段中止，job 已 kill），三视口截图互异亲读这条**未满足**，如实披露，不虚构 sha。
- r14Advice 归属：本样本 r14Text 为空态，无法区分是否被 A 覆盖；不将其列为已坐实的错配证据。
- 静态契约 5/5 绿与运行时 FAIL 并存：**契约 it1 的文本顺序判据不足以覆盖「await 后复核」语义**，这是本次失败的关键教训（tests verify intent：只测文本顺序的测试在业务逻辑变化时不会失败，是错误测试）。
- 队长审阅语「竞态窗口已坐实且守卫后无错配」与本报告 DOM 证据不一致：主详情 B 归属正确，但 growthCompare 辅助面板明确归属 A（stage5 10411→8888 / hash 7468252a57f6befe、8ddc5cc01e787ec9），因此硬门禁①判 FAIL。

## 9. 修复建议（findings，QA 不代改）

- **F1 blocker**：在 `loadGrowthCompare` 的每次 await 恢复后、写 `growthCompare.value` 前复核 `seq !== detailPanelSeq` 并 return；同理 `loadR14Advice` 在 `await fetchAdviceAction` 后、写 `r14Advice.value` 前复核。仅入口守卫不足。
- **F2 high**：契约 it1 升级为语义判据——断言 guard 同时出现在写入语句之前且位于每个 await 之后的写入路径上（或直接以可注入的 fake api + defer 做时序 RED），避免静态假绿。
- **F3 medium**：重跑真浏览器时序用例确认 B 面板归属 B（或空态），并补三视口互异截图后复验。

## 10. 收尾

- 鉴权 `data/auth-init.js` 用后删除并验证 ABSENT；QA 库 `battle_records` 回基线 count|max=104|477；自启 job 全 kill；data/ 命中 .gitignore，未入库。
