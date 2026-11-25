# R521 展开态 aria-expanded 修复（WCAG 2.1 AA / 4.1.2）

- 真父：`672d3d9bb5e03eb05bab24cb32e5008bdd36070b`（台账 §513）
- 范围：R519 判定的两处 P2「新建」disclosure 按钮折叠/展开态缺失 `aria-expanded`，AXTree 无 expanded 属性。
- 修复：仅给两按钮各加一行 `:aria-expanded` 绑定，对齐正面锚点 AttendanceApp.vue:13 / CalendarApp.vue:51；业务零改。

## 改动文件（精确 4 文件）

| 文件 | 改动 |
| --- | --- |
| `oa4rust/frontend/apps/desktop/src/views/OrgDirectoryApp.vue` | :72 按钮加 `:aria-expanded="showUnitCreate"` |
| `oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue` | :128 按钮加 `:aria-expanded="showGroupCreate"` |
| `oa4rust/frontend/apps/desktop/src/views/R521.expandedStateA11y.contract.test.ts` | 新增契约测试：2 核心断言（RED→GREEN）+ 6 守卫断言（@click/v-if/文案/testid 零改） |
| `oa4rust/docs/ops/oa4rust-r521-expanded-state-fix.md` | 本文档 |

## RED → GREEN

- 真父（未修复源码）：定点 vitest exit 1，核心 2 条 fail（`AssertionError: expected '<button type="button" class="org-crea…' to match /:aria-expanded="showUnitCreate"/`），守卫 6 条 pass。
- 修复后：定点 vitest 8/8 pass；全量 `pnpm test` 334 文件 3221 用例全 pass（exit 0）；`pnpm typecheck` 6 项目全 Done（exit 0）。

## 真浏览器实证（本地 dev server 加载工作区修复源码）

- 环境：Playwright Chromium headless，1440×900，DPR 1，light/dark 双主题；本地 Vite dev server `127.0.0.1:5199` 加载含修复的工作区源码（Vite 转换产物含 `aria-expanded": $setup.showUnitCreate`）。
- 会话：CDP route 拦截 `/oa4rust-api/api/authentication/who`，按 `ApiResponse<T>` 信封返回 `{success:true,data:{unique,name,roles:['admin'],roleList:['admin'],permissionLevel:9}}`，绕过远端测试服静态部署旧代码问题；mock 值均占位，无真值。
- 取证：CDP `DOM.getAttributes` + `Accessibility.getPartialAXTree`，`:org-create-unit-btn` / `:org-create-group-btn` 折叠→点击→展开两态；正面对照 Attendance/Calendar；每态 1440×900 截图 16 张。
- 结果（light/dark 对称）：两目标按钮折叠 `aria-expandedAttr="false"` / prop `"false"` / AXTree `expanded=false`，展开 `"true"` / `"true"` / `expanded=true`，AXTree name 分别 `新建部门/收起`、`新建组织组/收起`，面板 `panelVis C/E=false/true`；Attendance/Calendar 同样 false→true 且 name 正确，零回归；`pageerrors=[]`。
- 远端静态服对照：先前 `bash test-results/r521-run.sh` 在远端测试服上目标 attr/prop 均 null（与 R519 旧结果一致）、正面对照正常、pageerrors=[]，证明远端部署不含本地修复，非修复无效；故改用本地 dev server 实证。
- 证据（gitignored `oa4rust/test-results/r521-expanded-state-fix/`）：`report.json` sha256 `30f5d8e5d9e02fbad88ef3887f39ca2dee12ce8c63aacdaa2f50ce50122f516c`，16 张 PNG 截图与逐节点 DOM/AXTree 属性；runner 脚本 `r521-e2e.cjs`、`r521-run.sh`、诊断 `r521-diag.cjs` 位于 gitignored `oa4rust/test-results/r521-*`。

## 脱敏声明

- 全文 0 真值：无密钥/口令/个人邮箱/手机号/真实域名或 IP；本地回环地址与占位 mock 身份仅用于可复现验证，不含任何个人信息或环境真值。
