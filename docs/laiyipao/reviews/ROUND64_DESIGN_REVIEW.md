# ROUND64 设计终审报告：战报详情辅助面板竞态修复

- 评审任务：t123（kind=review，attempt=2）｜被评审任务：t121（R64 修复）
- 评审角色：独立终审（same-route-reviewer）｜评审结论：**PASS，可发布**
- 评审口径：六维全绿方可发布；全程只读，不改业务源码/契约/规范/.gitignore，不写 git。
- 落盘位置：`docs/laiyipao/reviews/ROUND64_DESIGN_REVIEW.md`（本文件，唯一落盘产物）。

## 一、结论与范围

R64 针对「我的战报」详情辅助面板（growthCompare 成长对比、r14Advice 建议）在快速切换战报时的竞态覆盖缺陷：旧详情 A 的延迟异步响应晚于新详情 B 到达，把 A 的数据写入已切到 B 的面板，造成 growthText 错配（A 的 `10411.0→8888.0` 出现在 B 的面板）。

本次终审对 t121 修复做独立复核，覆盖：test-first 真实性、修复正确性、保护面零改、QA 真浏览器铁证、门禁独立复跑、诚实边界六维。**六维全部达标，判定 PASS，可发布。**

必审材料：`docs/laiyipao/specs/round64-battles-detail-race/SPEC.md`（56 行）、`laiyipao/miniapp/src/styles/r64BattlesRace.contract.test.ts`（103 行/5 it）、实施 diff（`battles.vue`）、QA 首轮 `REPORT.md` 与独立复查 `POSTFIX_REPORT.md`（同在 `docs/laiyipao/audits/round64-battles-detail-race/`）。

## 二、维度一：test-first 真实性

- SPEC §6 记录 RED 实测：`exit=1` / 1 failed file / **3 failed | 2 passed** / 338 ms。三处失败全部是缺口断言：`:56` growthCompare 无写前复核、`:67` r14Advice 无写前复核、`:77` loadDetail 首个 await 前无序号递增；两条不变量断言通过；无 Transform / 模块加载 / 超时等假错。
- t121 契约升级对照：将实现故意回退到旧写法得到 RED **2 failed | 3 passed**，恢复修复后 GREEN **5/5**。RED→GREEN 与缺口一一对应，非文本顺序假绿。
- 契约真实读源码：`readFileSync` + `fileURLToPath` + `dirname/join` 读取 `battles.vue`，无内嵌实现字符串；`funcBlock()` 以大括号配对提取函数块。
- 语义判据自检：本轮另写 `data/semantic-check.js`，用与契约相同的 `SEQ_GUARD` 与 `guardedAfterAllAwaits()` 对「await 后有复核」与「仅入口守卫」两段求值，输出 `GOOD_expected_TRUE=true`、`BAD_expected_FALSE=false`、`VERDICT=SEMANTIC_GUARD_VALID`——**实证判据能检出 t120 的假绿**。
- 诚实说明：RED 原始日志未随任务留存，采信 SPEC §6 记录 + 契约结构 + GREEN/回退对照 + 语义自检，**不虚构日志**。
- 判定：**达标**。

## 三、维度二：修复正确性

- 实施仅在 `battles.vue` 内改动：`let detailPanelSeq=0`（:379-380）；`loadDetail` 在首个 await 前 `const seq=++detailPanelSeq`（:831），并按 id 缓存详情（:835）；`loadGrowthCompare(d,seq?)` 保留入口守卫（:497），在两处 await（:500 myBattleHistory、:503 myBattleDetail priorId）恢复后、写 `growthCompare.value` 前新增序号复核（:507）；`loadR14Advice(d,seq?)` 保留入口守卫（:772），在 await fetchAdviceAction 恢复后、写 `r14Advice.value` 前新增复核（:777）。
- 语义：每次 await 恢复后都重新核对 `seq !== detailPanelSeq`，过期响应直接丢弃，不写任何面板 ref；最新请求的响应才落盘。修复点位于「写 ref 之前」，覆盖全部异步分支。
- QA 复查 DOM 铁证：三视口复刻竞态时序后，growthCompare 保持 B 的空态「还缺养成前那局战报」、`growthHashes` 为空、A_MISMATCH=false，无 `10411/8888` 错配；主详情 detailHash=da91c7bf 仍属 B（R63 按 id 缓存无回退）；r14Advice 归属 B（superconduct 建议 send=0，仅 B 的 overheat）。
- 判定：**达标**。

## 四、维度三：保护面零改

- `git diff HEAD --stat` 仅 2 文件：`.gitignore`（+4）与 `laiyipao/miniapp/src/pages/battles/battles.vue`（+13/-4）。
- 逐字确认未改：`details.value[it.id]=d` 按 id 缓存、computed `detail` 读 `details[expandedId.value]`（:475）、`applyAdviceAction`、CTA `#r14-advice-action-btn`（:282，含 `role=button tabindex=0 @keydown.enter`，R60 键盘契约不回退）。
- 保护面：`engine`/`replay`/`hash`/`score`/`settle`/`loot`/`tokens` 与后端零改；令牌三锚点 `tokens.css:14-16` 原样（`lyp-tokens-v1` / `lyp-tokens-v3-r3` / `lyp-tokens-r6`）；全 `src` 检索 `--r64-` 零命中（仅契约否定断言，业务 0）。
- R60 键盘三计数复核：`role=button` / `tabindex="0"` / `@keydown.enter` 各 48，一一对应，分布 11 页面一致。
- 判定：**达标**。

## 五、维度四：QA 真浏览器铁证

- t120 首轮 FAIL 确属真实缺口：竞态铁证 `data/race-final-witness.txt` 时序清晰（A 378 stage5 10411 → A history 延迟 → B 41 stage40 19925 → A 延迟恢复仍覆盖 growthCompare）；DOM 错配 `growthText` 与 `growthHashes` 均属 A。队长初判有误已由 QA 纠正，本轮以铁证为准。
- t122 独立复查 PASS：真 Chromium 三视口复刻时序，旧响应不污染面板；三视口 `hOverflow` 全 false。
- 三图 sha256 本轮独立复算与 `POSTFIX_REPORT.md` 逐行一致且互异、均不等于 t120 的 `d5482858`：
  - `race-view-390.png` = `1bfc1b52536a96bc9ba2f15374468a996cd54018d3d36770b5d25306be6b301b`
  - `race-view-768.png` = `f7b44cb6427e8006d1f7148b50051f24a20c4e07ccabb82da1e30df23e438704`
  - `race-view-1280.png` = `dd1ee470bdfca995b8029d3f2f81c052836988f741dcc57b69a46a4c3e55faf4`
- 三图均亲读，非登录页、「我的战报」正常渲染（390×844/113290B、768×1024/242251B、1280×800/131343B），未见 A 数据错配。
- 判定：**达标**。

## 六、维度五：门禁独立复跑（本轮亲跑）

| 门禁 | 命令（摘要） | 结果 | RC |
| --- | --- | --- | --- |
| R64+R60 契约 | `npx vitest run` 两契约 `--reporter=dot` | 2 files passed / 11 tests passed | 0 |
| 全量测试 | `pnpm test` | 219 files passed \| 1 skipped、2030 tests passed \| 2 skipped | 0 |
| 类型检查 | `vue-tsc --noEmit -p tsconfig.json` | 无输出 | 0 |
| H5 构建 | `pnpm run build:h5` | DONE / Build complete | 0 |
| diff 检查 | `git diff --check` | 仅 `.gitignore` LF→CRLF 既有 warning | 0 |
| Go 基础设施守卫 | `TestNoInfraIdentifiersInRepo` | PASS（8.22s） | 整体 0 |
| Go 真值扫描守卫 | `TestNoRealSecretsInRepo` | **SKIP（非 PASS）** | 整体 0 |

- 真值扫描守卫因读不到 `laiyipao/deploy/.local.keys` 而 SKIP，逐字提示「真值扫描这条守卫**没有执行**，不是「通过了」」。本终审如实标注：**该守卫未执行，不计为通过**。
- 门禁脚本 stderr 有 `tee: ../data/*.log: No such file or directory` 提示，仅为日志落盘目录不存在，不影响命令本身 RC 与结论。
- 判定：**达标**（SKIP 项如实标注，不作 PASS 计）。

## 七、维度六：诚实边界与安全自检

- auth 四路径全 ABSENT：`docs/laiyipao/audits/round64-battles-detail-race/data/auth-init.js`、同目录 `admin-auth-init.js`、`laiyipao/miniapp/auth-init.js`、根 `auth-init.js`。
- `git check-ignore` 命中 `.gitignore:562`（审计 data/）；QA 库只读，`battle_records=104|477` 保持。
- 零真值入库：全树无密钥/口令/令牌/真实主机地址；本终审所有路径为仓库相对路径。
- 非阻塞残留：A 的延迟 history 恢复后仍会发 1 次 `/me/battles/477` 中间请求，但响应在 `:507` 复核点被丢弃、未写 ref，不污染面板。若需连请求也消除，可在 history await 后、找 priorId 前补一次复核（可选加固，移交队长，非本次发布阻断项）。
- 本终审未改业务源码/契约/规范/.gitignore，未写 git；唯一落盘为本报告。
- 判定：**达标**。

## 八、终审结论与非阻塞观察

**VERDICT：PASS（可发布）。** 六维全部达标：test-first 真实（RED 缺口断言 + 回退对照 + 语义自检）、修复正确（每次 await 恢复后写 ref 前复核 seq，过期丢弃）、保护面零改（仅 2 文件、锚点与 CTA 逐字不动）、QA 铁证（t120 FAIL 属实、t122 PASS 独立复现、三图互异亲读）、门禁全绿（SKIP 如实标注）、诚实边界充分（477 中间请求残留如实披露）。

非阻塞观察（不阻断发布，供队长后续排期）：
1. 真值扫描守卫 `TestNoRealSecretsInRepo` 因缺 `.local.keys` SKIP——需在具备该文件的环境补跑，本仓库不计为通过。
2. `477` 中间请求残留：可选在 history await 后补复核以连请求一并消除，当前不影响面板正确性。
3. RED 原始日志未随任务留存：依 SPEC 记录 + 契约结构 + GREEN/回退对照 + 语义自检采信，建议后续任务保留原始 RED 日志。
4. 门禁脚本 `tee` 日志路径不存在提示：仅日志落盘失败，不影响 RC；建议脚本内先 `mkdir -p`。