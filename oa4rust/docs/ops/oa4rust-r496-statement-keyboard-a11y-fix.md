# R496 SQL语句设计器语句项键盘可达性修复（WCAG 2.1.1）

## 1. 概况

- 轮次：R496（台账 §491 据实立项），真父提交 `364fdfcf861c8a6df2d46da8342c3ba32d6dd552`（R495 甄别 commit）；实际 HEAD 为台账 commit `cccf3ed42`，QueryStatementDesigner.vue blob 与真父一致（`756b9cde`），已 git rev-parse 核实。
- 缺陷：R495 甄别判 1 项 P2——QueryStatementDesigner.vue:53-55 `.sb-item` 语句项的『选中并载入编辑器』仅鼠标 `@click=selectStatement(s)` 一条路径；div 无 `tabindex`、无 `role`、无 `@keydown`，AX role=generic、可聚焦=false；键盘用户仅能 Tab 到行内 `✏ 编辑` / `🗑 删除`（非同功能），无法把已存语句载入 SQL 编辑区。
- 修复：该 div 开标签新增 `role=button` + `tabindex=0` + `:aria-pressed=currentStatement?.id===s.id` + `@keydown.enter=selectStatement(s)` + `@keydown.space.prevent=selectStatement(s)`，复用既有 `selectStatement`（:1019-1022 仅设 currentStatement + sql，无副作用）。
- 测试服：受控测试环境（域名 `<YOUR_TEST_DOMAIN>`、主机 `<TEST_HOST>` 占位）；凭据运行时从受控凭据源提取注入，真值不入库。

## 2. 根因与修复

- 静态锚点：QueryStatementDesigner.vue:53-55 `div v-for=s in filtered :key=s.id class=sb-item :class={active:currentStatement?.id===s.id} @click=selectStatement(s)`；行内两按钮 :62-63 `@click.stop=editStatement(s)` / `@click.stop=deleteStatement(s)`；`selectStatement` :1019-1022 = `currentStatement.value=s; sql.value=s.sql??''`。
- 修复选择：仅在该 div 开标签追加 5 个键盘可达属性，复用 selectStatement；不新增函数、不改 SQL 加载、不改行内按钮。
- 硬纪律：原 `@click`、`v-for`、`:key`、`:class active`、行内 `@click.stop`、class、`si-icon`/`si-name`/`si-meta` 结构、data-testid、样式、selectStatement 函数体全部零改；不改 MeetingApp.vue / modalFocus.ts / formErrorA11y.ts / main.ts。

## 3. 契约 RED→GREEN

- 契约：`oa4rust/frontend/apps/desktop/src/views/R496.statementKeyboardA11y.contract.test.ts`（源码字符串断言，readFileSync + `sbItemTag` 开标签提取）。
- 5 it：①核心 role=button + tabindex=0 + :aria-pressed + @keydown.enter/space 复用 selectStatement ②原 @click/v-for/:key/:class active 保留 ③行内 edit/delete @click.stop 全保留且无一调 selectStatement ④ selectStatement 函数体保留 ⑤ class/图标/信息结构零改。
- 真父 364fdfcf 定点 RED：Tests 1 failed | 4 passed（5），失败正为核心 `/role="button"/` 断言，4 项业务守卫父版全 pass 无桩错；修复版 GREEN：Tests 5 passed（5）。

## 4. 三 verify（全 exit 0）

- V1 定点契约：`cd oa4rust && pnpm vitest run frontend/apps/desktop/src/views/R496.statementKeyboardA11y.contract.test.ts` → 1 passed (5 tests)，exit 0。
- V2 全量测试：`cd oa4rust && pnpm test` → 328 files / 3188 tests passed，0 failed，exit 0。
- V3 类型检查：`cd oa4rust && pnpm typecheck` → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：入口 `index-Dxl2Lhqw.js`；部署 `deploy-pc-r496.sh`（tar -czf → scp → 同一归档 sha256 比对 → 远端解压 web.incoming-r496 → grep 入口 → 原子替换）`SHA_OK=6c7047bca7b33b7902c23a6c46c767c58f8956449d66dae602625ffe3897c4ff`，`DEPLOY_R496_PC_FULL_OK`。

## 5. 真浏览器 E2E

- 脚本 `test-results/r496-statement-keyboard-check.cjs` + runner（gitignored），Playwright Chromium headless 非物理真机 PC 1440×900 DPR1，light/dark 双主题；CDP DOM + Accessibility.getPartialAXTree + focus/Enter + CodeMirror/`.stmt-name` 检测器。
- 终轮 `R496_DONE checks=20 passed=20 failed=[] shots=2 errors=0`，0 pageerror。
- 语义与聚焦实证（双主题一致）：`.sb-item` 共 14 条，首条 DOM `role=button`、`tabindex=0`、`aria-pressed=false`；CDP AXTree `axRole=button`、axIgnored=false、axName 含语句名与图标；`.focus()` 后 becameActive=true、tabIndex=0；行内编辑/删除各 14 个全保留。
- 键盘激活实证（双主题一致）：聚焦第 2 条语句后真实 `press(Enter)` 即触发选中（`realEnterChanged=true`，`.stmt-name` 由空写入 `R343Q313711424-查询`）；说明本次 Vue `@keydown.enter` 在 headless 下真实按键即生效（R494 曾观察到 press(Enter) 未触发，本轮未见该现象，据实记录）。
- CodeMirror 内容判据（据实披露）：本轮测试服前 8 条语句的 SQL 正文均为空（`contentIdx=-1`，`.tb-info` 显示『1 行 · 0 字符』），故 `cmLen>0` 判据不适用、按 `contentIdx<0` 豁免；键盘激活的可感知效果由 `.stmt-name` 从空写入语句名这一决定性证据坐实（与 R495 补采鼠标对照同型检测器）。
- 截图来源如实标注：来自 Playwright Chromium headless 非物理真机，未冒充物理真机；2 图 read_image 目视 SQL 语句设计器页，语句项选中蓝框、`.stmt-name` 写入语句名，light/dark 主题正确。

## 6. 证据与 sha256（命令行 sha256sum 原始值）

- pc-statement-keyboard-light-r496.png daf568a6436d857ae876c55d5cb636adb5eec7c794071c1fb2dc39757034db2e
- pc-statement-keyboard-dark-r496.png 761fd668662a4041e73d64a799ac85ec1354ad6b60a412dd9497e07f2f76e883
- report.json 3cbb2a5e8f1ca5c1e26a2ab7d0d7820fcaed17e63ccaca7a0fcbec0d7ded2270
- 证据目录（gitignored）：`oa4rust/test-results/r496-statement-keyboard/`（2 PNG + report.json + 脚本）。

## 7. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 `<YOUR_TEST_DOMAIN>`、主机用 `<TEST_HOST>` 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 源码 diff 仅 QueryStatementDesigner.vue 开标签 6 行键盘增强属性；业务处理器、SQL 加载、样式、data-testid、全局设施零改。
