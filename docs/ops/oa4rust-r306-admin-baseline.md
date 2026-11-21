# R306 管理页视觉深化改造前基线实测（PC admin 18 页）

## 1. 来源与方法

- 数据来源：部署在测试服的当前 HEAD 产物（`index-CunnpRgh.js`），Playwright Chromium headless 真浏览器逐页访问，视口 1440×900。
- 访问地址：`https://<YOUR_TEST_DOMAIN>/oa4rust/app/admin/<route>`（真实域名以 `<YOUR_TEST_DOMAIN>` 占位，不入库）。
- 登录方式：脚本运行时经 ssh 到测试服读取受保护凭据文件完成真实 UI 登录；凭据仅在运行时读取，不写入脚本、证据或本文。
- 截图性质：**本地无头浏览器渲染截图，不是物理真机截图**；证据目录 `oa4rust/test-results/r306-baseline-pc/`（gitignore 忽略，不入库）。
- 诊断口径：DOM `aiPage` = 命中 `[class*="ai-page"],[class*="aiPage"]` 的元素数；`srcVar` = 源文件 `var(--ai-*` 出现次数；`loading`/`errorText`/`emptyState` 为正文文本匹配；`pageerror`/`apiFailures` 为 Playwright 事件监听。
- 页面集合：R305 基线识别的 18 个「源码零令牌 + DOM 零 ai-page」管理页，正是 R306（t53/t54/t55）视觉深化的目标页。

## 2. 量化结果

| # | 路由 | 页面 | 源码文件 | srcVar | DOM aiPage | loading | errorText | pageerror | API失败 | h1数 | bodyLen |
|---|------|------|----------|-------:|-----------:|---------|-----------|----------|---------|-----:|--------:|
| 1 | `/app/admin/query` | 查询管理 | `QueryManager.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 1296 |
| 2 | `/app/admin/jpush` | 消息推送 | `JPushApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 918 |
| 3 | `/app/admin/queryview` | 查询视图 | `QueryViewApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 834 |
| 4 | `/app/admin/form` | 表单管理 | `FormApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 1449 |
| 5 | `/app/admin/view` | 视图管理 | `ViewApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 832 |
| 6 | `/app/admin/fileinfo` | 文件信息 | `FileInfoApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 830 |
| 7 | `/app/admin/query-designer` | 查询设计器 | `QueryDesigner.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 1311 |
| 8 | `/app/admin/portal-designer` | 门户设计器 | `PortalDesigner.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 1274 |
| 9 | `/app/admin/query-explorer` | 查询浏览器 | `QueryExplorerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 836 |
| 10 | `/app/admin/query-view-designer` | 视图设计器 | `QueryViewDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 836 |
| 11 | `/app/admin/portal-dict-designer` | 门户字典设计器 | `PortalDictDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 836 |
| 12 | `/app/admin/portal-page-designer` | 门户页面设计器 | `PortalPageDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 914 |
| 13 | `/app/admin/portal-script-designer` | 门户脚本设计器 | `PortalScriptDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 840 |
| 14 | `/app/admin/portal-widget-designer` | 门户组件设计器 | `PortalWidgetDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 836 |
| 15 | `/app/admin/service-invoke-designer` | 服务调用设计器 | `ServiceInvokeDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 838 |
| 16 | `/app/admin/find-designer` | 查找设计器 | `FindDesignerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 1164 |
| 17 | `/app/admin/pdf-viewer` | PDF查看器 | `PdfViewerApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 871 |
| 18 | `/app/admin/common` | 公共组件库 | `CommonApp.vue` | 0 | 0 | 否 | 否 | 0 | 0 | 1 | 834 |

- 捕获：18/18 页成功；pageerror 合计 0，API 失败（4xx/5xx）合计 0。
- 令牌消费：DOM `aiPage=0` 共 18/18 页；源码 `srcVar=0` 共 18/18 页；**18 页全部「源码无 AI 令牌 + DOM 无 ai-page」双未消费**，与 R305 基线对该 18 页的判定一致。
- 健康度：loading=true 共 0 页，errorText=true 共 0 页；无重定向到登录、无 h1 重复（均 h1Count=1）。

## 3. 逐页截图证据

截图位于 `oa4rust/test-results/r306-baseline-pc/<name>.png`（`name` 见上表序号对应文件名，例如 `17-admin-query.png`），另有 `00-login-flow.png` 为真实登录流程留证。

## 4. 结论与改造目标

- 改造前 18 页均处于「零 AI 令牌、零 ai-page、无装饰性玻璃/栅格/骨架」的原始管理页状态；功能可访问、无加载或错误异常。
- R306 改造目标：为这 18 页注入设计令牌与 AI 风格类（`ai-page` 等），保持 h1 语义、路由、端点功能不变；改造后复查以同口径脚本比对 `aiPage` 由 0 提升，且 pageerror/apiFailures 维持 0、无新增 loading/errorText。
- 本基线为只读实测，未修改任何源码、契约测试或清单；复查将在 t53/t54/t55 全部完成后以 `r306-after-pc.cjs` 同口径执行。

## 5. 证据索引

- `oa4rust/test-results/r306-baseline-pc/evidence.json`：18 页逐页诊断与事件采集。
- `oa4rust/test-results/r306-baseline-pc/` 下 19 张 PNG（18 页 + 登录流程）。
- `oa4rust/test-results/r306-baseline-run.log`：本次运行日志，完成标记 `R306_BASELINE_PC_DONE {"total":18,"captured":18,"errors":0,"apiFailures":0}`。
- `oa4rust/test-results/r306-source-scan-current.json`：当前工作树 18 页源码令牌扫描（srcVar 全 0）。
