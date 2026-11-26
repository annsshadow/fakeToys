# R64 竞态守卫修复独立复查报告（POSTFIX）

- 任务：t122 / attempt 0e063989-849f-4cff-b294-f50167dfe51f（独立复查 t121 修复闭环）
- 范围：只读复查；工作目录 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248，HEAD=932106f19
- 零真值：全文不含 token/口令/私人路径；鉴权仅经 gitignored `data/auth-init.js` 会话侧注入，用完即删
- 结论：**PASS（修复核心闭环成立）**。t121 在两处 await 恢复后、写 ref 前新增序号复核，真 Chromium 三视口复刻 t120 竞态时序后，旧响应不再污染辅助面板（growthCompare 保持当前展开项 B 的空态，无 A 的 stage5 10411→8888 错配）。附带 1 项非阻断残留：陈旧链仍发出一次 `/me/battles/477` 中间请求，但其响应在复核点被丢弃，未写入任何面板。

## 1. t121 修复源码核实（QA 只读）

- `laiyipao/miniapp/src/pages/battles/battles.vue:496-509` `loadGrowthCompare(d, seq?)`：`:497` 入口守卫；`:500` `await api.myBattleHistory(20)`；`:503` `await api.myBattleDetail(priorId)`；`:507` **新增复核** `if (seq !== undefined && seq !== detailPanelSeq) return // 异步恢复后复核：过期响应不写 ref`；`:508` 才写 `growthCompare.value`。
- `battles.vue:771-779` `loadR14Advice(d, seq?)`：`:772` 入口守卫；`:776` `await fetchAdviceAction(...)`；`:777` **新增复核** 同款序号判断；`:778` 才写 `r14Advice.value`。
- 契约 `laiyipao/miniapp/src/styles/r64BattlesRace.contract.test.ts`：`SEQ_GUARD` 正则 + `guardedAfterAllAwaits(block, wi)` 语义判据（校验最后一个 await 之后、写入之前存在复核），替代 t120 的 `block.slice(0, wi)` 文本顺序假绿判据。
- 不变量保留：`battles.vue:475` computed detail 仍绑 `details.value[expandedId.value]`；`:835` `if (details.value[it.id]) return` 按 id 缓存；`:282` CTA `#r14-advice-action-btn` 含 `role="button" tabindex="0" @keydown.enter` 完整保留。

## 2. 复查方法（真 Chromium，与 t120 同一时序）

- 服务：API 8080（`/api/v1/config`=200）；静态 H5 5201（bundled python http.server，根目录 laiyipao/miniapp/dist/build/h5）。旧 5174（302）已剔除。
- 延迟注入 `race-init2.js`：patch `window.XMLHttpRequest`，`window.__raceCtl.rules=[{match,ms,remaining}]` 延迟 send，`log` 记录 send/done 时戳；`race-set2.js` 设 `/me/battles?limit=20&offset=0` 与 `reaction=superconduct` 各延迟 3000ms×1。
- 操作序列：进入 `#/pages/battles/battles` → 点 A（id=378，stage5，10411）→ 0.4s 后点 B（id=41，stage40，19925）→ 等 4.2s → `r64-probe.js` 读 DOM 与 raceLog。
- 三视口：390×844 / 768×1024 / 1280×800，各用独立 session 完整重放（`race-view-driver.sh`）。

## 3. 逐条核对（队长指定必列项）

### 3.1 A 延迟恢复后的 __raceLog 是否还发 `/me/battles/477`

- **如实结论：仍发 1 次（非阻断残留）。** 三视口 `sends477=1`。以 768 完整 raceLog（`data/race-view-768.txt`）铁证：`send /me/battles?limit=20&offset=0 @...837776`（A 的 history，被延迟）→ `send /me/battles/41 @...838222`（B 详情）→ `done @...838244` → `send /me/battles/477 @...840788` 与 `done A-history @...840788` **同一毫秒** → `done 477 @...840792`。
- 归因：477 出自 A 陈旧链的第二个 await（`loadGrowthCompare` 在 A 的 history 恢复后找同关前局 477 并发起 baseline 请求）。t121 复核点位于两个 await 之后、写 ref 之前（line 507），**并未前移到发请求之前**，故请求必然发出；但该响应返回后在 line 507 被序号比对拦截，**未写入 `growthCompare.value`**，面板不受污染。
- 与 t120 对比：t120 同位置确实发出 477 且结果写回，造成 A 错配；本次请求仍在但结果被丢弃，数据正确性已恢复。若要连中间请求也消除，需在 `:500` history await 之后、找 priorId 之前再加一道序号复核——属后续可选加固，不影响本次功能判定。

### 3.2 growthCompare 归属 B 或空态

- 三视口全部：`expandedHead="胜 ★ 第 40 关 19925"`、`detailHash=da91c7bf08b63fa4`（B，R63 按 id 缓存正确）
- `growthText="AI · 养成前后（同一关可信差值） 还缺养成前那局战报，先打一局同关 · 构筑变更点未上报，不做估算。"`、`growthHashes=""` —— **B 的无同关前局空态**。
- `A_MISMATCH=false`（三视口均无 `10411`/`8888` 字样）。t120 的 `10411.0 → 8888.0`、`hash 7468252a57f6befe / 8ddc5cc01e787ec9` 错配已彻底消失。

### 3.3 r14Advice 归属

- 三视口 `superconduct` 建议 send 数 = 0：A 陈旧链的 r14 建议请求被入口守卫与调用顺序彻底拦截（A 的详情/建议看出去时 seq 已过期）。
- 唯一发出的建议请求为 B 的 `reaction=overheat`（768：`send .../advice/action?element=lightning&source=battle&reaction=overheat @...838240`，`done @...838266`）。
- DOM `r14Text` 为本样本诚实空态（「AI 暂时没有足够数据给出建议……先补齐目录再回来」），请求层归属 B 已由 raceLog 坐实，文本层无 A 污染。

### 3.4 三视口截图 sha 互异且 ≠ 旧 d5482858

| 视口 | 文件 | sha256 | hOverflow |
|---|---|---|---|
| 390×844 | screenshots/race-view-390.png | 1bfc1b52536a96bc9ba2f15374468a996cd54018d3d36770b5d25306be6b301b | false |
| 768×1024 | screenshots/race-view-768.png | f7b44cb6427e8006d1f7148b50051f24a20c4e07ccabb82da1e30df23e438704 | false |
| 1280×800 | screenshots/race-view-1280.png | dd1ee470bdfca995b8029d3f2f81c052836988f741dcc57b69a46a4c3e55faf4 | false |

- 三张 sha 两两互异，且均 ≠ t120 重复 sha `d5482858d5c63f031479bd29bddb6f28b312434cee16775919ed6da0852832b1`。
- 三视口溢出：`innerWidth == document.scrollWidth == clientWidth`（390/390/390、768/768/768、1280/1280/1280），hOverflow 全 false。
- 亲读 768 与 1280 两图：均为「我的战报」非登录页，可见 AI 防守诊断、战绩总账、战绩趋势、最近战斗记录等，展开态与空态面板正常渲染，无异常空白。

### 3.5 门禁（全 RC0）与 Go 守卫

| 门禁 | 结果 |
|---|---|
| R64 契约 r64BattlesRace.contract.test.ts | 5 passed / 5，RC=0 |
| R60 契约 r60A11yKeyboard.contract.test.ts | 6 passed / 6，RC=0（键盘不回退） |
| miniapp 全量 pnpm test | 219 files passed \| 1 skipped；2030 tests passed \| 2 skipped，RC=0 |
| vue-tsc --noEmit | RC=0 |
| pnpm run build:h5 | DONE Build complete，RC=0 |
| git diff --check | RC=0（仅 `.gitignore` LF→CRLF 提示，无错误） |
| Go TestNoInfraIdentifiersInRepo | PASS（8.22s），RC=0 |
| Go TestNoRealSecretsInRepo | **SKIP 非 PASS**，RC=0 |

- TestNoRealSecretsInRepo 原文：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。按时如实登记为 SKIP，不冒称 PASS。
- 静态回归：令牌三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 逐字在 `tokens.css`；全 src `--r64-` 业务命中 0；`placeholder-class="ph"` 17 处；`role="button"`/`tabindex="0"`/`@keydown.enter` 各 48 处一致（R55/R60 不回退）。

## 4. 诚实边界与残留

- **非阻断残留**：`/me/battles/477` 中间请求仍发 1 次（见 3.1），结果被复核点丢弃、不污染面板；如需连请求也消除，须在 history await 后、找 priorId 前补复核。
- r14Advice 归属由请求层（superconduct send=0，仅 B 的 overheat）坐实；DOM 文本为空态，无 A 污染。
- 本轮为 t121 修复的独立复查，未改任何业务源码/契约/规范/.gitignore；未做 git 写操作。
- 鉴权文件 `data/auth-init.js` 复查后已删除（ABSENT），QA 库 `battle_records` 保持 count=104 / max(id)=477，所有自启 job 已 kill。

## 5. 判定

- 硬门禁①（growthCompare 不归属 A）**通过**；②（旧响应不写 ref）**通过**，其中"不再发 477"字面未满足但结果丢弃、无害，已如实列 3.1；③（r14Advice 归属可判定/空态）**通过**；④（三图 sha 互异、亲读非登录、零溢出）**通过**；⑤（全门禁 RC0、SKIP 非 PASS）**通过**。
- 综合：**PASS**。t121 修复对 R64 目标（旧响应晚到不污染辅助面板）有效，可进入终审；上列 477 请求残留作为可选加固项移交队长判断。
