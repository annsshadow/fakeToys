# R483 考勤模拟表格 ARIA 语义补齐修复（RED→GREEN）

## 1. 概况

- 轮次：R483（台账 §478 据实立项），任务 t357，page-core 实现，真父提交 2e2643e961db946ec1086d78416e656d5e25e453。
- 修复对象：R482 甄别（甄别 commit 4d2a54f2e8ed2d5d6d58c60be0749be7e3d3264d，文档 oa4rust-r482-simulated-table-a11y-audit.md）判定的 1 项可达真缺陷 P2——WCAG 1.3.1 考勤模拟表格无 ARIA 表格语义。
- 真机声明：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，非物理真机；真域名与凭据仅存在于 gitignored runner，文档使用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位符，真值 0 入库。
- 证据目录：oa4rust/test-results/r483-simulated-table-a11y/（4 PNG + report.json + 脚本，全部 gitignored）。

## 2. 缺陷根因

- AttendanceApp.vue:155 表头 div.pt 内 6 个 span.th（姓名/日期/上班/下班/工时/状态）无 role；:159 数据行 div.tr（v-for records，含 late 类）无 role；:160-166 共 7 个 span.td（姓名/日期/上下班/工时/状态徽章/操作按钮）无 role。
- CSS :1863 .pt / :1864 .th / :1865 .tr{display:grid;grid-template-columns:80px 80px 90px 90px 60px 70px 1fr} / :1867 .td 为纯视觉网格，无程序化表格语义。
- R482 真浏览器 CDP AXTree 实测：容器 generic、6 表头全 axRole=none + axIgnored=true、7 单元格 none，表格语义角色 0 个；原生对照文档页 table/columnheader×4/row×2/cell×8 完整。
- tableA11y.ts:18-42 全局增强器仅 querySelectorAll("table")，不覆盖 div.tr 模拟表格。

## 3. 修复方案（最小语义补齐）

- 仅新增 role 属性，共 10 行模板改动：
  - 考勤记录 section.content-panel 加 role="table"（表头与数据行共同 wrapper）。
  - 表头 div.pt 加 role="row"，6 个 span.th 各加 role="columnheader"。
  - 数据行 div.tr 加 role="row"，7 个 span.td 各加 role="cell"。
- 硬纪律：现有 class / data-testid / v-for records / @click 处理器 / :class 动态绑定 / display:grid 布局 / loading 与 empty 分支 / 全部业务视觉零改；不改 tableA11y.ts、main.ts 全局设施；git diff 仅 AttendanceApp.vue 10+/10-（每行原样仅追加 role 属性）。

## 4. 契约 RED→GREEN

- 新契约 oa4rust/frontend/apps/desktop/src/views/R483.simulatedTableA11y.contract.test.ts（5 it，源码字符串真断言非桩）：表头行 role=row+6 列头 role=columnheader；wrapper role=table；数据行 role=row；7 单元格 role=cell；业务零改守卫（data-testid / v-for / @click 四处理器 / :class 动态绑定 / display:grid 列宽保留）。
- 真父 2e2643e96 定点 RED：1 failed 文件，Tests 4 failed | 1 passed（5），4 项失败全为 role 语义真实断言，守卫通过，0 桩错（日志 test-results/r483-red.log）。
- 修复版 GREEN：Tests 5 passed（5）（日志 test-results/r483-v1.log）。

## 5. 三 verify（全 exit 0）

- V1：cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R483.simulatedTableA11y.contract.test.ts → 1 passed / 5 tests，exit 0。
- V2：cd oa4rust && pnpm test → 323 files / 3163 tests passed / 0 failed，exit 0。
- V3：cd oa4rust && pnpm typecheck → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：cd oa4rust && pnpm run build exit 0，入口 index-1gVYczRV.js / CSS index-COmAqEyf.css；部署脚本 test-results/deploy-pc-r483.sh 整目录 tar→scp→sha256 比对→远端 /usr/local/oa4rust 建 web.incoming-r483→mv 原子替换，DEPLOY_R483_PC_FULL_OK。

## 6. 真浏览器双主题 CDP AXTree 实证

- E2E oa4rust/test-results/r483-sim-table-a11y-check.cjs + runner r483-sim-table-a11y-run.sh：1440×900 DPR1 headless 非物理真机，light/dark 双主题，登录一次逐主题 goto 考勤页与原生对照文档页；CDP DOM.enable + Accessibility.enable 后逐标记节点 getPartialAXTree(fetchRelatives:false) 取 axRole/axName/axIgnored。
- 终轮 R483_DONE checks=14 passed=14 failed=[] shots=4 errors=0；0 pageerror。
- 考勤模拟表格（light/dark 完全一致）：container axRole=table（axIgnored=false）；表头 hrow axRole=row；6 列头 axRole=columnheader 且 axName 分别 姓名/日期/上班/下班/工时/状态（可播报，axIgnored=false）；数据行 axRole=row；7 单元格 axRole=cell（axName 依次为 —/11/20/—/—/—/—/详情 分析 归档 删除，axIgnored=false）。
- 原生对照（文档管理页，light/dark）：table axRole=table、thead th×4 axRole=columnheader（标题/状态/创建时间/操作）、tbody tr×2 axRole=row，零回归。
- 视觉零回归：4 张截图 read_image 目视确认真 OA4RUST admin 页 1440×900，light/dark 表格网格与操作按钮布局完整，与 R482 基线一致；此维度为纯程序化语义补齐，视觉层不变属预期。

## 7. 证据 sha256（命令行原始值）

| 文件 | sha256 |
| --- | --- |
| pc-attendance-sim-light-r483.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-sim-dark-r483.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-document-native-light-r483.png | d04912d4e66442967f89a2ef283eccbdf94cd129d42a495fcf95a4d2061bed1f |
| pc-document-native-dark-r483.png | 46ee4ce775e3e01314c2c3ab6d2518e5f2e7849d38526bf0774c0c20fe96b69d |
| report.json | f1648fc107c814fb8b4b34a5942ca7f79db2c63ab182d1e8aaaf4fa982a5292b |

## 8. 真值纪律与写域

- 精确写域 3 文件：AttendanceApp.vue、R483.simulatedTableA11y.contract.test.ts、本文档；git add 精确带 oa4rust/ 前缀，禁 -A / git add .，单提交。
- 无域名/IP/口令/私钥/个人路径入库；真域名与凭据仅存在于 gitignored runner 且运行时读环境；占位符 <YOUR_TEST_DOMAIN> / <TEST_HOST> 真值 0。
- 边界声明：AXTree 是屏幕阅读器数据源的客观代理；未使用真实屏幕阅读器做人工听感验证。
