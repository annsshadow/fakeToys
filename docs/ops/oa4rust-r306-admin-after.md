# R306 管理页视觉深化改造后复查对照（PC admin 18 页）

## 1. 来源与方法

- 改造前基线：见 `docs/ops/oa4rust-r306-admin-baseline.md`（commit d17a31514），测试服产物 `index-CunnpRgh.js`，采集于改造启动前。
- 改造后复查：t53/t54/t55 三项改造全部完成并部署后，测试服产物 `index-BiyhjNmp.js`（服务 active），以**同一脚本、同一 18 路由、同一视口 1440×900、同一诊断口径**重跑（脚本 `r306-after-pc.cjs` 与基线脚本仅输出目录/完成标记不同）。
- 访问地址：`https://<YOUR_TEST_DOMAIN>/oa4rust/app/admin/<route>`（`<YOUR_TEST_DOMAIN>` 占位，真实域名不入库）。
- 登录方式：脚本运行时经 ssh 到测试服读取受保护凭据文件完成真实 UI 登录；凭据仅运行时读取，不写入脚本、证据或本文。
- 截图性质：**本地无头浏览器渲染截图，不是物理真机截图**；证据目录 `oa4rust/test-results/r306-after-pc/`（gitignore 忽略，不入库）。

## 2. 量化对照

| # | 路由 | 页面 | 源文件 | srcVar 前→后 | DOM aiPage 前→后 | loading 前→后 | errorText 前→后 | h1数 前→后 | pageerror | API失败 |
|---|------|------|--------|--------------|------------------|---------------|-----------------|------------|-----------|---------|
| 1 | `/app/admin/query` | 查询管理 | `QueryManager.vue` | 0→29 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 2 | `/app/admin/jpush` | 消息推送 | `JPushApp.vue` | 0→24 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 3 | `/app/admin/queryview` | 查询视图 | `QueryViewApp.vue` | 0→30 | 0→5 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 4 | `/app/admin/form` | 表单管理 | `FormApp.vue` | 0→26 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 5 | `/app/admin/view` | 视图管理 | `ViewApp.vue` | 0→30 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 6 | `/app/admin/fileinfo` | 文件信息 | `FileInfoApp.vue` | 0→30 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 7 | `/app/admin/query-designer` | 查询设计器 | `QueryDesigner.vue` | 0→13 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 8 | `/app/admin/portal-designer` | 门户设计器 | `PortalDesigner.vue` | 0→13 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 9 | `/app/admin/query-explorer` | 查询浏览器 | `QueryExplorerApp.vue` | 0→13 | 0→5 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 10 | `/app/admin/query-view-designer` | 视图设计器 | `QueryViewDesignerApp.vue` | 0→13 | 0→4 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 11 | `/app/admin/portal-dict-designer` | 门户字典设计器 | `PortalDictDesignerApp.vue` | 0→13 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 12 | `/app/admin/portal-page-designer` | 门户页面设计器 | `PortalPageDesignerApp.vue` | 0→13 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 13 | `/app/admin/portal-script-designer` | 门户脚本设计器 | `PortalScriptDesignerApp.vue` | 0→17 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 14 | `/app/admin/portal-widget-designer` | 门户组件设计器 | `PortalWidgetDesignerApp.vue` | 0→16 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 15 | `/app/admin/service-invoke-designer` | 服务调用设计器 | `ServiceInvokeDesignerApp.vue` | 0→16 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 16 | `/app/admin/find-designer` | 查找设计器 | `FindDesignerApp.vue` | 0→23 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 17 | `/app/admin/pdf-viewer` | PDF查看器 | `PdfViewerApp.vue` | 0→21 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |
| 18 | `/app/admin/common` | 公共组件库 | `CommonApp.vue` | 0→16 | 0→2 | 否→否 | 否→否 | 1→1 | 0 | 0 |

- 覆盖：改造前后均 18/18 捕获完成；完成标记 `R306_AFTER_PC_DONE {"total":18,"captured":18,"errors":0,"apiFailures":0}`。
- 令牌消费：DOM `aiPage=0` 由 18/18 降为 0/18；改造后 aiPage>0 共 18/18（范围 2~5）；源码 `srcVar>0` 共 18/18（范围 13~30）。**18 页全部从「双未消费」转为「源码有令牌 + DOM 有 ai-page」**。
- 功能不退化：pageerror 合计 0，API 失败合计 0，loading=true 0 页，errorText=true 0 页，h1 数变化 0 页（均保持 h1Count=1），无重定向登录。

## 3. 视觉目视确认

- `17-admin-query.png`：改造前为白底扁平工具栏；改造后工具栏区呈淡紫渐变底板、按钮改为蓝紫渐变，"新建查询"主按钮更突出，语义与列表数据未变。
- `25-admin-queryview.png`：改造后新增淡紫渐变区块与渐变按钮列，"业务列表/原有操作入口与能力完整保留"提示可见，空态仍在，说明功能入口未被替换。
- 上述截图均为本地无头浏览器渲染，非物理真机；逐页 PNG 见证据目录。

## 4. 结论

- R306 A/B/C 三组改造（t53/t54/t55）对目标 18 页的令牌消费度提升在本轮真浏览器复查中**全部得到验证**：每页 DOM 均出现 `ai-page` 类，源码均注入 13~30 处 `var(--ai-*)` 令牌。
- 改造未引入错误、API 失败、加载卡死或标题语义退化；18 页 h1 数保持 1。
- 本复查为只读实测，未修改任何源码、契约测试或清单。

## 5. 证据索引

- `oa4rust/test-results/r306-after-pc/evidence.json`：改造后 18 页逐页诊断。
- `oa4rust/test-results/r306-after-pc/` 下 19 张 PNG（18 页 + 登录流程）。
- `oa4rust/test-results/r306-after-run.log`：完成标记 `R306_AFTER_PC_DONE`。
- `oa4rust/test-results/r306-source-scan-current.json`：改造后源码令牌扫描（srcVar 13~30）。
- 改造提交：318cace72（t53 A 组）、ea1c40f99（t54 B 组）、5da60b8fe（t55 C 组）。
