# R482 WCAG 1.3.1 模拟表格语义结构甄别（只读）

## 1. 概况

- 轮次：R482（台账 §477 据实立项），任务 t355，design-lead 甄别，只读不改码。
- 真父提交：d11c601af3b52e1fbe30331f25cf9b498241f5fe；甄别前基线 HEAD=UP 同值，AB=0 0，工作树干净。
- 方法来源：R481（WCAG 2.4.4）技术观察——考勤记录表使用 div.tr 模拟表格、非语义行容器，属 1.3.1 维度，本轮独立甄别。
- 真机声明：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，非物理真机；真域名仅存在于 gitignored runner，本文档使用占位符 <TEST_HOST> / <YOUR_TEST_DOMAIN>。
- 证据目录：oa4rust/test-results/r482-simulated-table/（4 PNG + report.json + ax-subtree.json + 分析/探针脚本，全部 gitignored）。

## 2. 静态侦察（真实信号）

- oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue:155：表头 div.pt 内含 6 个 span.th（姓名/日期/上班/下班/工时/状态），无 role=row / role=columnheader。
- AttendanceApp.vue:159：数据行 div.tr（v-for records，含 late 类），无 role=row；:160-166：7 个 span.td（姓名/日期/上下班/工时/状态徽章/操作按钮），无 role=cell。
- AttendanceApp.vue:1865：.tr{display:grid;grid-template-columns:80px 80px 90px 90px 60px 70px 1fr;...}；:1867：.td{color:var(--text-primary)}——纯视觉网格布局，无任何 ARIA 表格语义。
- oa4rust/frontend/apps/desktop/src/utils/tableA11y.ts:18-42：installTableA11y() 仅 querySelectorAll("table")，为 thead th 补 scope=col、tbody 行首 th 补 scope=row；MutationObserver 同样只遍历 table。不覆盖 div.tr 模拟表格。
- oa4rust/frontend/apps/desktop/src/main.ts:20 导入、:616 调用 installTableA11y()——全局增强器已安装但该模拟表格不在其作用域。
- 静态结论只作为候选信号；本轮全部判定以真浏览器 CDP AXTree 实测为准，不凭源码 div.tr 预判。

## 3. 方法

- 脚本 r482-e2e.cjs：先登录注入主题（localStorage oa4rust_theme + data-theme），轮询等待考勤记录 .tr 出现（waitTries=1 即数据已到达，排除加载完成态误判）。
- 对考勤页模拟表格：DOM 标记容器/6 表头/1 行/7 单元格为 data-r482-id，CDP DOM.enable + Accessibility.enable 后逐节点 getPartialAXTree(fetchRelatives:false) 取 axRole/axName/axIgnored；另对容器 getPartialAXTree(fetchRelatives:true) 取完整子树。
- 对文档管理页原生表格（AdminTable.vue:13 table.admin-table-grid）同法采 table/thead th/tbody tr/td 作对照组，坐实差异源于源码实现而非采集工具限制。
- 脚本 r482-axsubtree.cjs：独立真浏览器补采容器完整 AX 子树（fetchRelatives=true），落盘 ax-subtree.json。
- runner r482-run.sh / r482-axsubtree-run.sh：从已验证范式复制，凭据 ssh 运行时从受控凭据源提取注入 OA_TEST_USER/OA_TEST_PASS，OA_BASE_URL 真值仅存在于 gitignored runner。

## 4. 实测数据（CDP AXTree）

### 4.1 模拟表格（考勤页，light/dark 完全一致）

| 项 | 实测值 |
| --- | --- |
| 容器 role | generic（axIgnored=false） |
| 表头 span.th ×6 | 全部 axRole=none、axIgnored=true、axName 空 |
| 数据行 div.tr ×1 | axRole=none、axIgnored=true |
| 单元格 span.td ×7 | 全部 axRole=none、axIgnored=true |
| 节点 role 分布（逐标记 15 节点） | generic 1 + none 14 |
| 表格语义角色（table/row/columnheader/cell） | 0 个 |
| 双主题一致 | light/dark roleDist 完全相同 |

### 4.2 原生对照表格（文档管理页，light/dark 完全一致）

| 项 | 实测值 |
| --- | --- |
| table | axRole=table、axIgnored=false |
| thead th ×4 | axRole=columnheader、scope=col，axName 为 标题/状态/创建时间/操作 |
| tbody tr ×2 | axRole=row、axIgnored=false |
| td ×8 | axRole=cell、axIgnored=false，axName 携带行数据 |
| 节点 role 分布（逐标记 15 节点） | table 1 + columnheader 4 + row 2 + cell 8 |

### 4.3 容器完整 AX 子树（ax-subtree.json，light）

- 容器标记命中（marked.ok=true），containerNodeId=215，getPartialAXTree(fetchRelatives:true) 返回 29 节点。
- roleDist：generic 3 / none 14 / StaticText 6 / button 4 / main 1 / RootWebArea 1——表格语义角色（table/row/columnheader/cell）合计 0。
- 6 个 StaticText 为孤立数据值（—、11/20、—、—、—、—），无列标题关联；4 个 button 为 详情/分析/归档/删除；RootWebArea name=考勤管理 · OA4Rust。
- 6 个 span.th 表头文本在 AX 子树中无对应 StaticText 节点（逐节点查询均 role=none/ignored=true），读屏器无法获得列标题语义。

### 4.4 运行健康

- R482_DONE groups=4 errors=0；0 pageerror。
- 页面视觉正常：考勤页 6 列表头与 1 行数据在 grid 布局下可见，文档管理页原生表格列头与行数据完整，双主题均无布局崩坏。

## 5. 三分判定

- 可达真缺陷：1 项（P2）。考勤记录表（AttendanceApp.vue:155-166）用 div.pt/span.th + div.tr + span.td 模拟表格，CDP AXTree 实证 0 表格语义角色：容器 generic、6 表头全 ignored、数据仅孤立 StaticText。读屏用户无法通过表格导航命令（列/行关联、列标题播报）理解数据结构，与同环境原生对照表格（完整 table/columnheader/row/cell）形成确定性差异；installTableA11y 源码坐实仅覆盖真实 table，不会为该模拟表格补语义。
- 定级 P2 依据：影响辅助技术用户对数据结构的理解，属程序化语义缺失（WCAG 1.3.1 核心）；不阻断主流视觉用户，主内容（数据值 + 按钮名）仍可读；当前记录仅 1 行且多为占位值，实际影响面有限；修复为低成本语义补齐。据此未判 P1。
- 退化实证口径说明：本维度为纯程序化语义，视觉层本就正常，故"可听退化"以 CDP AXTree（屏幕阅读器的数据源）为客观代理；未使用真实屏幕阅读器做人工听感验证，据实声明此边界。
- 不可达 L1：0。所有模拟表格节点均可达、可见（visible=15/15），axIgnored 的表头节点恰为缺陷证据本身，不作不可达排除；考勤页不在 R306C 硬排除清单。
- 正常规范预期：文档管理页原生表格（table/columnheader/row/cell 完整 + scope=col 增强）符合 1.3.1；考勤页非表格内容（统计卡、请假申请、打卡地点、排班配置等）本轮不涉及。

## 6. 修复建议（供后续 RED→GREEN 轮，本轮不改码）

- 方案 A（语义补齐，最小改动）：AttendanceApp.vue:155 表头 div.pt 加 role="row"，6 个 span.th 加 role="columnheader"；数据容器 section 或新增 wrapper 加 role="table"；:159 div.tr 加 role="row"；:160-166 每个 span.td 加 role="cell"。
- 方案 B（结构化改写）：将模拟表格改为原生 <table>/<thead>/<th scope="col">/<tbody>/<tr>/<td>，自动获得语义并纳入 installTableA11y 增强范围，长期更稳。
- 验收：修复后真浏览器 CDP AXTree 应出现 table/row/columnheader×6/cell 语义角色，且列标题 axName 可被读屏器播报。

## 7. 证据 sha256（命令行原始值）

| 文件 | sha256 |
| --- | --- |
| pc-attendance-simulated-light-r482.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-simulated-dark-r482.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-document-native-light-r482.png | d04912d4e66442967f89a2ef283eccbdf94cd129d42a495fcf95a4d2061bed1f |
| pc-document-native-dark-r482.png | 46ee4ce775e3e01314c2c3ab6d2518e5f2e7849d38526bf0774c0c20fe96b69d |
| report.json | b3bf668d45a7d724bc74ff174e0ee90bb034cc4c8518a5bf26155b861b38b3d6 |
| ax-subtree.json | 0986bae881e1d965073288f0fc1a763b04a8e552b3da236cc2a302b489dda305 |

- 4 张截图已 read_image 目视：light/dark 考勤页与文档页均 1440×900、真 OA4RUST admin 登录态、内容名实相符，非物理真机。
- 截图 sha256 经命令行 sha256sum 核验，与 report.json 内 sha256 逐一吻合（shaCheck ok=true ×4）。

## 8. 真值纪律与写域

- 唯一写域：本文档 oa4rust/docs/ops/oa4rust-r482-simulated-table-a11y-audit.md，精确 git add 单文档，禁 -A / git add .，零源码改动。
- 真值脱敏：无域名/IP/口令/私钥/个人路径入库；真域名与凭据仅存在于 gitignored runner，运行时读环境；文档使用 <TEST_HOST> / <YOUR_TEST_DOMAIN> 占位符。
- 测试库业务标识仅以截断前缀呈现（如 R330SLOTS_1795286985…），非凭据。
- 本轮为只读甄别，据实判缺陷不臆造、不凑轮次；修复由后续 RED→GREEN 轮次承接。
