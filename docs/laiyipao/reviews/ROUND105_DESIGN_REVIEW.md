# R105 独立终审设计复核：index 顶栏 768 横溢修复

- 终审任务：t293 / attempt_id=2441c698-fa69-443f-a241-ff93dbccf701；身份：same-route-reviewer（R40 独立终审）
- 审查对象：t291 实施（index.vue 顶栏 768 横向溢出修复），reviewedTaskId=t291
- 基线 HEAD：0151ce97e1216d70efd899e85d72e7ad2f93b750（全程未变，实施未提交）
- 全链：t289 R105-C 只读验证侦察（坐实真实缺陷）→ t290 规范+RED → t291 实施（曾因契约内部矛盾被判 failed，队长就地修复契约并复跑 GREEN 10/10，实施本身全绿）→ t292 QA PASS → 本轮终审
- 性质：只读审查 + 单一写域 docs/laiyipao/reviews/ROUND105_DESIGN_REVIEW.md；未改业务 vue / 契约 / QA 报告 / .gitignore / 令牌。

## 判决

**verdict = pass。六维全绿，无 blocker / high / medium；8 条 low 观察如实列出但不阻断，同意分批发布。**

---

## 维度一：RED → GREEN 真实性（pass）

- SPEC：`docs/laiyipao/specs/round105-topbar-overflow/SPEC.md`（51 行）§0 裁定与权威口径 / §1 修复目标 / §2 根因 / §3 唯一精确改点（三处）/ §4 不变量 / §5 契约与验证口径 / §6 诚实边界齐备。权威口径取 `docs/laiyipao/design/AI_VISUAL_SYSTEM.md` §7「H5 限宽 560px 居中 + P7 响应式支柱」与历史 R98–R104 零横溢必测项（全库 39 处）；纠正 A 报告称该文件不存在的错误结论。
- RED 逐字：`docs/laiyipao/specs/round105-topbar-overflow/RED-evidence.md` 与 `RED-raw.log` 一致——命令 `cd laiyipao/miniapp && npx vitest run src/styles/r105TopbarOverflow.contract.test.ts --reporter=verbose`，vitest v4.1.12，`Test Files 1 failed (1)` / `Tests 3 failed | 7 passed (10)`（RED-raw.log:65-66），Duration 319ms；三处缺口 FAIL 在 RED-raw.log:4/:6/:8（.player-info min-width:0 / ≤900px .res-row flex-wrap:wrap / .topbar max-width+box-sizing），七处不变量 PASS 在 :10-16，无 PARSE_ERROR。3 个缺口断言真失败（业务尚未改），7 个不变量全绿。
- 契约稳健性：`laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts` 真实 `fs.readFileSync` 读 `pages/index/index.vue` 与 `styles/tokens.css`（`read()` 基于 `fileURLToPath(import.meta.url)` + `srcRoot`），禁止内嵌实现副本；`cssBlock()` 以唯一 class 锚点（`.topbar {` / `.player-info {` / `.res-row {`）定位，`occurrences()!==1` 返回 null 令测试自责；`firstCssBlock()` 取媒体块之前的基础块（适用于 .res-row 基础块+媒体块同名共存）；`mediaBlock()` 花括号配平；`tagBefore()` 跨行开标签容错；全程不依赖行号。
- 独立复跑：自建后台 job `bash-ae9e629ec973416f` 复跑契约 `Tests 10 passed (10)` RC=0（vitest Duration 320ms），采信 GREEN 10/10。
- 契约矛盾修复披露：`RED-evidence.md:33` 如实记录首轮契约 9 it（令牌与零标记合为一条）RED 得 3 failed|6 passed 与 SPEC 声明 10 it 不一致，遂拆为两条独立不变量 it 复跑得 3 failed|7 passed；首轮原始输出被最终 RED-raw.log 覆盖并明示。`firstCssBlock()` 辅助函数带注释说明其用途（基础块+媒体块同名共存取前者），系队长就地修复的内部矛盾点。t291 failed 历史保留正确——本终审采信派发描述的『契约内部矛盾被判 failed、队长就地修复并复跑 GREEN 10/10、实施本身全绿』；`team_task_get(t291)` 现返回 not found（已归档），故不改写其历史 verdict。契约逻辑正确且独立复跑 10/10，披露充分，不阻断（见 low L6）。

## 维度二：实施精确性（pass）

- `git diff -- .gitignore laiyipao/miniapp/src/pages/index/index.vue` 逐行核对：业务 `index.vue` 仅 `<style scoped>` 内三改点、净 +10 行（2+1+7），与 SPEC §3 逐字吻合：
  1. `.topbar` 块（约 754 行）增 `max-width: 100%;` 与 `box-sizing: border-box;`（+2 行）；
  2. `.player-info` 块（约 781 行）增 `min-width: 0;`（+1 行）；
  3. 新增 `@media (max-width: 900px) { .res-row { flex-wrap: wrap; } }`（+7 行）。
- 保留项核实：`.topbar` 的 `display:flex` 与 `justify-content:space-between` 未动；`.res-row` 基础块 `white-space:nowrap` 未动；四胶囊插值 `store.wallet.coin/gem/energy/keys` 原样保留；`@media` 内仅 `flex-wrap:wrap` 单一声明，未静默替换为 `white-space:normal` 或缩 gap。零多余改动。
- `.gitignore` +5 行（注释 + round105-a-number-unit / round105-c-topbar / round105-topbar-overflow / round105-topbar-overflow-postdeploy 四审计 data 目录忽略），沿袭 round104 防真值入库模式，属卫生配置，非业务改动。
- 令牌三锚点 `tokens.css:14/15/16`（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）未动；业务源码零 `--r105-`（全局 grep 仅契约自身断言字符串命中 r105TopbarOverflow.contract.test.ts:8/159/160）。

## 维度三：QA 充分性（pass）

- QA 报告 `docs/laiyipao/audits/round105-topbar-overflow/REPORT.md`（§1–§8）独立通读，三视口 390×844 / 768×1024 / 1280×800 真 Chromium（agent-browser 0.41.1）。
- 核心修复坐实：768 视口 `clientWidth=768 / scrollWidth=768 / hOverflow=false`；对照 t289 改前 real-768（coin=52000）`scrollWidth=780`、溢出 12px；改后 `.res-row.right=639.44` 落在 `.topbar.right=664` 限宽容器内（改前 780.09 远超 664）。390/1280 零横溢不回退。
- computed 三改点生效：`topbar maxW=100%/boxSizing=border-box` 三视口在；`resRow flexWrap=wrap`（390/768 命中 ≤900px 媒体查询）/ `nowrap`（1280 >900px 基础块保持）；`playerInfo minW=0px` 三视口在；`.topbar display:flex/justify:space-between` 未变。断点按预期生效，基础 nowrap 未被破坏。
- 四胶囊（52000/4321/320/🔑7）三视口完整渲染无裁切；768 下 🔑7 折到第二行但仍在 resRow 内；玩家信息断行但被限制在容器内。
- 键盘不变量：三视口各 12 次真 Tab 序列一致（ai-cta → ann-all → ann-item×2 → dim-hint×5 → mg-row×3），命中元素 computed `role=button`/`tabIndex=0`/逐元素零横溢；AX 树坐实两条 ann-item `expanded=false`。
- 6 图 sha256 用 `ops_stat` revision 逐张独立重算，与 REPORT §6 表机器比对 **6/6 逐字一致、UNIQUE 6/6**：r105-index-390=deb4df460671d380474dff8953c93b611a9c4870f62b7ee1aac8dad9c2727775 / r105-index-768=1f742f71addf85b04d15dfba0daa7e44194d57797154417f2ebf1f82e7ea0e26 / r105-index-1280=7626e313663a478f398850d0dc74c0c6ab13a8dbce7e41d522a22955aa2c7eb8 / r105-topbar-390=9a0ef333df6b1690fd7942e24fc4c8c5bf4b53d0a5c5eecf9d0de3ebde5e63c7 / r105-topbar-768=c758b0693ed196ae43b41dd6bd5c99db081e8b8b028a4917cb6a856a53eb424b / r105-topbar-1280=40cd0bd5c9ab520c7f44b5d3d817d45b11fc467e329a2ec5acc91d5c6063b2b8。
- 本人亲读 2 张非登录页确认非空态：`r105-index-768.png`（768×1024，顶栏四胶囊 52000/4321/320/🔑7 完整显示，构筑评分 D30 / 五维补强卡清晰）与 `r105-index-1280.png`（1280×800，顶栏四胶囊 + AI 战术洞察 + 公告 + 雷达图非空态），与探针数据一致。
- 诚实边界合理（见维度四）。

## 维度四：诚实边界处理（pass）

QA §7 五条边界均如实披露、合理列 low 不阻断：
1. 后端 8080 未起，数据经 init-script 合成，钱包取真实量级 coin=52000（与 t289 基线同量级），令牌 r105-qa-placeholder-token 占位符仅内存注入、零真值（L1）。
2. 改前 12px 溢出结论引用 t289 真浏览器对照报告 `docs/laiyipao/recon/ROUND105_C_TOPBAR_OVERFLOW_VERIFY.md`，本轮未注入旧 CSS 重做 A/B；改后数据为本轮真浏览器实测（L3）。
3. 错误态 retry 三件套本轮未坐实（正常态 retryN=0），由 r99/r102 契约静态守住，未冒充真浏览器 PASS（L2）。
4. 截图亲读 2/6，其余以实测 sha 与探针数据为准（L4；本人另亲读 r105-index-1280，覆盖 1280 桌面态）。
5. aria-expanded 仅验证折叠态 false 与真 Tab 命中，展开态由 R101 报告覆盖（L5）。
- 768 图『游客账号』视觉断行系 `min-width:0` 允许收缩的设计预期、被限制在容器内，QA §3 已如实披露，非新缺陷（L7）；t291 failed 历史保留与契约矛盾修复披露见维度一（L6）；gates G3/G4 的 build.attacker 占位告警为测试既有 stderr 非本轮引入，QA §5 已披露（L8）。

## 维度五：门禁独立复跑（pass）

- QA `data/gates-out.txt` 汇总 GATES_DONE、各段 RC=0：G1 r105 契约 10 passed / G2 a11y 五契约 64 passed / G3 index.test.ts 27 passed / G4 全量 237 passed|1 skipped files、2185 passed|2 skipped tests / G5 type-check / G6 build:h5 / G7 git diff --check 全 RC=0。
- 本轮至少两项独立复跑且与 QA 汇总一致（均 RC=0）：
  - 契约（job `bash-ae9e629ec973416f`）：`Test Files 1 passed (1)` / `Tests 10 passed (10)`。
  - 全量 vitest（job `bash-4507bb47f3ab4e0d`）：`Test Files 237 passed | 1 skipped (238)` / `Tests 2185 passed | 2 skipped (2187)`，Duration 24.92s，与 G4 逐字一致。

## 维度六：不变量与卫生（pass）

- 键盘与语义不变量：R99 `go(d.hint)`、R102 retry / ann-all / goStage / go(aiInsight.hint) 三件套（role=button / tabindex=0 / @keydown.enter 同 handler）保留；R100/R101 `.ann-item` 三件套 + `:aria-expanded="String(expanded.has(a.id))"` 保留；四胶囊插值保留；契约 7 条不变量 it + QA 真 Tab/AX 双重坐实。
- 令牌三锚点 `tokens.css:14/15/16` 在；业务源码零 `--r105-`。
- HEAD `0151ce97e1216d70efd899e85d72e7ad2f93b750` 未变；`git status` 非 `??` 项仅 `M .gitignore` + `M laiyipao/miniapp/src/pages/index/index.vue`（t291 预期写域），tracked data 修改 0。
- auth 五路径 `git check-ignore` 全部 IGNORED（.env / laiyipao/admin/.env.local.keys / laiyipao/deploy/.local.keys / laiyipao/server/.local.keys / laiyipao/miniapp/.env.local）；`git diff --check` RC=0；报告零真值；自启 job 全部 completed（无 running）。

## low 观察表（不阻断）

| 编号 | 观察 | 处置 |
|---|---|---|
| L1 | 后端 8080 未起，数据 init-script 合成（占位令牌、零真值） | 已披露，视觉/布局验证不受影响 |
| L2 | retry 三件套未真 Tab 坐实，仅静态守 | 由 r99/r102 契约覆盖，不冒充 PASS |
| L3 | 改前 12px 对照引用 t289，未重做 A/B | t289 真浏览器证据充分，采信 |
| L4 | QA 亲读 2/6 截图 | 本人另亲读 r105-index-1280；其余以 sha+探针为准 |
| L5 | aria-expanded 仅验证 false 折叠态 | 展开态由 R101 报告覆盖 |
| L6 | t291 failed 历史因团队任务已归档不可直查；契约矛盾（9it→10it、firstCssBlock）修复细节主要在 RED-evidence.md:33 与契约注释 | 采信派发描述+代码证据，历史 verdict 不改写 |
| L7 | 768『游客账号』视觉断行 | min-width:0 收缩的设计预期，QA §3 已披露 |
| L8 | gates G3/G4 build.attacker 占位告警 | 测试既有 stderr 非本轮引入，QA §5 已披露 |

---

**一句话判决：六维全部 pass、无任何 blocker/high/medium，R105 index 顶栏 768 横溢修复真实、精确、QA 充分、门禁全绿、卫生合规，verdict=pass，同意进入分批发布。**
