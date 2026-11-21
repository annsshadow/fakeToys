# R329 PC 文档中心（DocumentApp.vue）只读深度审计

- 轮次：R329（首次覆盖该页，R300–R328 未审计）。
- 审计对象：`oa4rust/frontend/apps/desktop/src/views/DocumentApp.vue`（路由 `/app/admin/document`，系统管理 / 内容管理 / 文档管理）。
- 方式：真浏览器 Playwright Chromium headless，视口 1440×900，**本地无头浏览器实测，非物理真机**；截图与 evidence 落 gitignored 目录 `oa4rust/test-results/r329-pc-document-audit/`。
- 环境：测试服从既有 gitignored 部署脚本 / `.local.keys` 运行期读取，域名/主机/凭据均不入库；本文统一用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，真值 0 处。
- 判定纪律：真缺陷（真 UI 可复现 + 真请求/响应 + DOM 证据 + 源码定位）/ 取证限制 / 产品语义差异三分，禁臆造。

## 一、结论摘要

**判定 1 项真缺陷（P1）：文档中心全部业务操作入口未渲染** —— 新建、搜索、字段/批量状态、管理视图、孪生端点 A/B、数据·文档深度 C、密文文档列表，以及每行的 15 个行操作（推荐/置顶/发布/撤发/密文发布/阅读数/可见人/通知/记录浏览/日志/点赞/详情/删除）在真实页面上**全部不存在**。
根因：`DocumentApp.vue` 把这些按钮写在 `AdminToolbar` / `AdminTable` 的**默认插槽**里，而这两个 R298 标准封装组件**只渲染具名插槽**（`AdminToolbar` 只有 `#filters` / `#actions`，`AdminTable` 只有 `#cell-*`），默认插槽内容被 Vue 静默丢弃。页面无报错（pageerror=0），属静默功能失效。

| 项 | 判定 |
| --- | --- |
| ① 列表加载与空态 | 真缺陷（同一根因：数据加载成功但呈现断裂） |
| ② 新建 → 列表回显 | 真缺陷（入口缺失导致功能不可用） |
| ③ 删除 → 列表刷新 | 真缺陷（入口缺失） |
| ④ 发布/撤发布状态回显 | 真缺陷（入口缺失） |
| ⑤ 密文相关端点 | 取证限制（破坏性写未授权/入口缺失，未伪造） |
| ⑥ 加载失败 500 兜底与空态可区分 | PASS（此部分组件正常） |
| ⑦ `/api/data/document/0/0...` 探针 | 产品语义差异（可用性探测，非缺陷；但入口缺失实际点不到） |
| ⑧ 契约回归与 0 pageerror | pageerror=0；核心 UI 契约不成立（真缺陷） |

## 二、真请求 / 真响应 / DOM 三件套

- 登录：`POST /oa4rust-api/api/authentication/login` → 200；`GET /oa4rust-api/api/authentication/who` → 200。
- 列表：`POST /oa4rust-api/api/document/filter/list/1/size/50/manager` → 200，响应 `data` 含 1 条 draft 文档（`create_time=2026-11-21 11:59:40...`，`title` 为空，`status=draft`）。
- 真 DOM 按钮全集（v2 evidence `dom.allButtons`）：`["☾深色模式","☰","⌘K","🔔","已发布","草稿","刷新","标准","紧凑"]` —— 无任何业务操作按钮。
- 真 DOM 计数：`btnCreate=0`、`btnSearch=0`、`btnAct=0`、`btnDel=0`、`itemTable=0`、`tableRow=0`（源码自定义 `.table-row` 结构未渲染）、`toolbarBtn=1`（仅内置"刷新"）、`adminTableRow=1`（通用表格行）。
- 真 `toolbarHTML`：`input + <div class="admin-toolbar-filters"></div> + <div class="admin-toolbar-actions"><button>刷新</button><!----></div>` —— 默认插槽内容位置只有注释占位 `<!---->`，证明插槽内容被丢弃。
- 真 `tableHTML`：通用 `admin-table-grid`，行内 4 个 `td` 显示 `"" / "draft" / "—" / "—"`；标题空、创建时间"—"、操作列"—"，无 `statusLabel` 的"草稿"中文映射、无 `documentColumns` 的 actions 按钮。
- `tableText`：`表格密度 标准 紧凑 标题 状态 创建时间 操作 draft — —`。
- 证据：`oa4rust/test-results/r329-pc-document-audit/T1-list.png`、`V2-DOM-list.png`、`evidence.json`、`evidence-v2.json`（均为本地 headless 截图/数据，非物理真机）。

## 三、源码定位

- `DocumentApp.vue:12–23`：`<AdminToolbar ...>` 内的搜索框、`+ 新建文档`（`:15`）、字段/批量状态、管理视图、孪生端点、孪生端点B、`数据·文档深度C`、密文文档列表 —— 均为默认插槽内容。
- `DocumentApp.vue:41–57`：每行 15 个 `btn-act` + `btn-del`（`:57`）—— 写在 `<AdminTable ...>` 默认插槽。
- `AdminToolbar.vue:17` `<slot name="filters" />`、`:20` `<slot name="actions" />`、`:21` 内置"刷新"、`:22` 仅当传 `createLabel` 才渲染内置新建按钮 —— **无默认 `<slot/>`**；DocumentApp 未传 `createLabel`，故连内置新建按钮也不出现。
- `AdminTable.vue:34` `<tr ... class="admin-table-row">`、`:36` `<slot :name="'cell-' + col.key" ...>` —— **只有 `cell-*` 具名插槽，无默认 `<slot/>`**；故走通用 `row[col.key]` 渲染，操作列显示原始值或"—"。
- `DocumentApp.vue:105–110` `documentColumns` 定义含 `title/status/createdAt/actions` 四列，但调用方未提供对应 `#cell-*` 插槽，列渲染退化为通用值。

## 四、各项判定明细

### ① 列表加载与空态（真缺陷，同一根因）
真请求 200 且返回 1 条文档，但真 DOM 未呈现标题/中文状态/创建时间/操作按钮，仅显示通用表格行的原始 `draft`。数据层正常、呈现层断裂，是默认插槽缺陷的直接表现。

### ②③④ 新建/删除/发布 闭环（真缺陷：入口缺失）
真 DOM 中 `+ 新建文档`、行内"发布"/"撤发"/"删除"按钮计数均为 0，真 UI 无入口，闭环无法开始。这不是无头 picker 之类的取证限制，而是按钮根本未被渲染。

### ⑤ 密文相关端点（取证限制）
`onCipherPublish`（`DocumentApp.vue:170–179`，先 `prompt()` 输入密文再 `PUT /api/document/cipher/publish/content`）与 `onViewRecord`（`:363`，`POST /api/document/cipher/{id}/persist/view/record` 属写操作）均为破坏性写；且入口按钮同样缺失。按纪律未伪造密文、未触发写，**如实标取证限制**，不判缺陷。

### ⑥ 加载失败 500 兜底（PASS）
以 `page.route` 将 `**/api/document/filter/list/**` fulfill 为 500 后重载：真 DOM 渲染 `ListLoadError`（`mode=empty`），文案"文档列表加载失败：HTTP 500: Internal Server Error"，`retry` 按钮 1 个；与正常空态 `.empty`（"暂无文档数据"）可区分。此路径组件工作正常（源码 `DocumentApp.vue:225–228` / `ListLoadError.vue`）。证据 `V2-fail500.png`、`evidence-v2.json > verdicts.fail500*`。

### ⑦ `/api/data/document/0/0...` 探针（产品语义差异，非缺陷）
`loadDocTwin3`（`DocumentApp.vue:494–539`）用 `s(p)=p.catch(()=>null)` 包装约 35 条深段族 + 匿名筛选 + 评论探针，`Promise.all` 提交后无条件 `toast.success("数据·文档深度C N 条已提交")`。错误被有意吞掉，属可用性路由探测而非用户数据操作，按源码注释（rev485 "arity 已校验；path 段全填 0 避影子"）判为**产品语义差异，非真缺陷**。注意：因该按钮本身未渲染（默认插槽问题），真 UI 实际无法点击，本次未触发这些探针。

### ⑧ 契约回归与 pageerror
全程 `pageerror=0`。但核心 UI 契约（新建/行操作按钮存在）不成立，见真缺陷。

## 五、影响面与严重度

- 严重度：**P1**。文档管理是系统管理核心功能页，所有写操作（新建/删除/发布/撤发/密文/通知/推荐/置顶）入口全部消失，用户无法使用；且静默失败无报错，常规巡检难以发现。
- 影响面：直接为 `DocumentApp.vue` 单页。延伸建议（本次审计范围外，未验证）：排查是否还有其他页面把内容放进这两个 R298 组件的默认插槽，避免同类静默失效。
- 非数据/权限风险：列表读接口本身 200 正常，问题在 UI 呈现。

## 六、修复方向（仅建议，本次未改代码）

- 方案 A（最小、推荐）：改造 `DocumentApp.vue` 使用具名插槽 —— 工具栏按钮放进 `<template #actions>`（或 `<AdminToolbar #actions>`），表格四列分别提供 `#cell-title` / `#cell-status` / `#cell-createdAt` / `#cell-actions` 插槽渲染标题、中文状态标签、创建时间、15 个操作按钮；删除弹窗逻辑保持。
- 方案 B（改组件，需评估回归）：给 `AdminToolbar` / `AdminTable` 增加默认 `<slot/>` 渲染以兼容现有写法；因这两个组件被多页复用，须逐一回归，风险高于方案 A。
- 护栏：补一条渲染级契约测试 —— 打开文档页后断言出现"新建文档"按钮、每行 `.btn-del` 与"发布"按钮、`toolbarHTML` 含新建入口，杜绝静默失效回归。

## 七、验证方式（修复后）

1440×900 真浏览器重跑本审计 E2E（`r329-pc-document*.cjs`）：断言 `btnCreate>=1`、每行 `btnAct>=15`、`btnDel>=1`、`toolbarHTML` 含"+ 新建文档"、行操作列含"发布"/"删除"；并走通"新建 STAMP 文档 → 列表回显 → 发布 → 删除 → 列表刷新消失"闭环，保持 pageerror=0 与 500 兜底 PASS。

## 八、纪律声明

本轮为只读审计：未改任何源码 / 契约 / 后端 / 配置，未部署，未执行破坏性写。所有结论基于真浏览器真请求与 DOM 证据。项目持续迭代（≥1000 轮），本文不表示项目完成。
