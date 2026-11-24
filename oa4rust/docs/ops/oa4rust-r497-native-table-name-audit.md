# R497 WCAG 1.3.1 原生数据表格可访问名称 / caption 甄别报告

- 轮次：R497；任务：t388；类型：只读甄别（不改业务代码）。
- 真父基线：`9d3d4d72800bc3eeaa94f699211ea2801023e976`（台账 §492 据实立项）。分支 `opt1000/oa4rust-pc-ai-refactor`；基线 HEAD=UP，AB=0/0，工作树干净。
- 测试来源：测试服真实页面，Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，**非物理真机**（截图为浏览器真页面渲染，不代表物理设备）。采集方式：CDP DOM + Accessibility.getPartialAXTree。
- 证据：`test-results/r497-native-table/`（gitignored），含 `report.json`、`supplement.json`、原始 PNG。测试地址与凭据仅从环境变量 / SSH 运行时读取，本报告不写真值（占位 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`）。

## 1. 判据与反预判纪律

1. 本轮只甄别 WCAG 1.3.1 中「原生 HTML 数据表格可访问名称 / caption」这一子维度，不扩张到模拟表格、语义角色或列头 scope。
2. **反预判铁律**：源码无 `<caption>` 不必然构成缺陷；周边 `panel-title` / heading / 文档标题可能提供等价上下文。必须在真实浏览器里以 CDP AXTree 同时坐实 table 的 `axRole/axName` 与周边标题的 `axRole/axName/ignored`。
3. 三分判定：
   - 可达真缺陷：表格可访问名称缺失且同页无等价上下文，并有实证影响读屏理解。
   - 不可达 L1：页面无表格实体（空态等）或该维度在当前数据下不可达，不臆造缺陷。
   - 正常 / 规范预期：表格虽 `axName` 为空，但同页语义标题 + 列头提供等价上下文，且每页仅一张目标表格。

## 2. 静态侦察范围（不重复预判）

- 全 views 下原生 `<table class="data-table">` 命中 22 处：BamApp.vue:56、CmsIndexApp.vue:122、CommonApp.vue:18、DeploymentApp.vue:22、LogViewerApp.vue:18、NoteApp.vue:25、PortalDictDesignerApp.vue:18、PortalPageDesignerApp.vue:27、PortalWidgetDesignerApp.vue:18、ProcessApplicationApp.vue:32、ProcessDictDesignerApp.vue:23、ProcessFormDesignerApp.vue:31、ProcessManagerApp.vue:154、ProcessTaskCenterApp.vue:61、ProcessXformApp.vue:23、QueryImporterDesignerApp.vue:23、QueryExplorerApp.vue:21、QueryQueryApp.vue:25/:64、QueryStatDesignerApp.vue:23、ServiceInvokeDesignerApp.vue:18、ThreeMemberApp.vue:19；全库 `<caption>` 仅 EChartsView.vue:8 一处且非 data-table。
- `frontend/apps/desktop/src/utils/tableA11y.ts`（R202）：`installTableA11y(root=document)` 只给 thead th 补 `scope=col`、tbody 行首 th 补 `scope=row`，由 MutationObserver 驱动；不补 caption / aria-label；`main.ts:616` 调用。R482 仅修 AttendanceApp 模拟 div.tr，未触及原生 table caption。**本子维度未被既有增强器覆盖，因此必须实测而非引用既有结论。**
- 抽测路由（`main.ts`）：bam `113 → /app/bam/`、note `114 → /app/note/`、log-viewer `248 → /app/admin/log-viewer/`、cms-index `302 → /app/admin/cms-index/`（另 `484`/`502` 为 redirect）。抽测覆盖空态页与有表页两类，其余候选页不预判。

## 3. 主轮实测程序化结果（report.json）

主轮 job EXIT=0，`R497_DONE groups=8 fatals=0`，8 组 `pageerrors=[]`。

| 组 | 主题 | 路径 | tableCount | dataTableCount | emptyState | 表格 AX | 截图 sha256（前 12 位） |
|---|---|---|---:|---:|---:|---|---|
| bam | light | /app/bam/ | 0 | 0 | 1 | 无表格 | db9f61789cd8 |
| bam | dark | /app/bam/ | 0 | 0 | 1 | 无表格 | 803f39ce7193 |
| log-viewer | light | /app/admin/log-viewer/ | 0 | 0 | 1 | 无表格 | 8d90da1a9692 |
| log-viewer | dark | /app/admin/log-viewer/ | 0 | 0 | 1 | 无表格 | 23399bb5d4dd |
| note | light | /app/note/ | 1 | 1 | 0 | axRole=table / axName="" / ignored=false | 3073d4e69a41 |
| note | dark | /app/note/ | 1 | 1 | 0 | axRole=table / axName="" / ignored=false | 5b7c9fb60998 |
| cms-index | light | /app/admin/cms-index/ | 1 | 1 | 0 | axRole=table / axName="" / ignored=false | 8789d54a3653 |
| cms-index | dark | /app/admin/cms-index/ | 1 | 1 | 0 | axRole=table / axName="" / ignored=false | bf3c842ffd24 |

> 空态页 bam / log-viewer 双主题均 `tableCount=0/emptyState=1`，无表格实体可甄别，据实归 L1，不臆造 caption 缺陷。

## 4. 逐表结构证据（与 report.json 逐字段一致）

| 字段 | note 表格（light/dark 同） | cms-index 表格（light/dark 同） |
|---|---|---|
| cls | data-table | data-table |
| captionCount / captionText | 0 / "" | 0 / "" |
| ariaLabel / ariaLabelledby | "" / "" | "" / "" |
| headers | 标题、内容摘要、成员、更新时间、操作（5） | 名称、目标、排序、描述、更新时间、操作（6） |
| rowCount | 3 | 1 |
| firstRowText | R337V95893485-笔记改R321 审计自动填写的内容，用于验证真实创建闭环。团队成员编辑删除 | R388R388-NOSUCH-17953617959340—编辑删除 |
| viewHeader | 笔记管理沉淀团队知识与待办线索+ 新建 | CMS 索引设计+ 新建 |
| documentTitle | 笔记管理 · OA4Rust | CMS索引设计 · OA4Rust |
| AX | `{axRole:"table",axName:"",axDescription:"",axIgnored:false}` | `{axRole:"table",axName:"",axDescription:"",axIgnored:false}` |

两页在 report.json 中 `panelTitle=""` / `nearestHeading=""`，故不能仅凭该字段下结论；补采直接对 heading-like 元素取 AXTree，见下节。

## 5. 周边标题 AX 补采（supplement.json）

补采 job EXIT=0，`R497_SUP_DONE groups=4 fatals=0`，4 组均 `pageerrors=[]`；补采截图与主轮同页同主题 sha 完全一致（说明未改变页面状态）。

| 组 | 表格位置 / 父容器 | 同页语义标题 AX |
|---|---|---|
| note / light | top=262 bottom=432；parentCls=content-panel glass-card modern-section modern-card | `<h1>笔记管理` → axRole=heading / axName="笔记管理" / ignored=false（top=114–144）；Logo span "OA4RUST" → axRole=none / ignored=true |
| note / dark | 同上 | 同上（标题与 ignored 状态一致） |
| cms-index / light | top=477 bottom=559；parentCls=content-panel glass-card | `<h1>CMS索引设计` → heading / ignored=false（118–148）；`<h3>业务工作台` → heading（272–295）；`<h2>CMS 索引设计` → heading（347–377）；Logo span ignored=true |
| cms-index / dark | 同上 | 同上（标题与 ignored 状态一致） |

4 组表格自身 AX 均为 `{axRole:"table",axName:"",axIgnored:false}`，tableCount=1，即每页仅一张目标表格；语义标题均为非忽略的 heading，位于表格上方的页面内容区，且与 `viewHeader` / `documentTitle` 用语一致。

## 6. 三分判定

| 分类 | 计数 | 实测依据 |
|---|---:|---|
| 可达真缺陷 | 0 | 4 张表格虽 `axName=""` 且无 caption，但同页存在非忽略语义 heading（笔记管理 / CMS索引设计），每页仅一张 data-table，列头提供字段语义；读屏用户可从标题 + 列头获得表格上下文。WCAG 1.3.1 并未要求表格必须有可访问名称，本轮未发现缺失等价上下文的实证。 |
| 不可达 L1 | 2 类（4 组） | bam、log-viewer light/dark 空态 `tableCount=0/emptyState=1`，无表格实体；不把「没有表格」判成「表格名称缺陷」。 |
| 正常 / 规范预期 | 4 项（2 页 × 双主题） | note、cms-index 的 data-table 具 table 角色 + 5/6 个列头；语义 heading 提供等价上下文；tableA11y 已补 th scope（R202 既有覆盖，本轮不重复计为新增）。 |

**反证边界**：静态 22 处 `<table class="data-table">` 仅抽测 4 页（覆盖空态与有表两类），其余 18 处不在本轮实测范围，**不据源码预判为缺陷也不预判为正常**；后续轮次可沿用本轮同一 E2E 判据续作。

## 7. 截图 sha256 命令行核验

`sha256sum` 对证据目录 12 个 PNG 逐一核验，与 `report.json.screenshotSha` / `supplement.json.shotSha` 全一致；补采 4 张与主轮同页同态 sha 相同（正常，页面状态未变）。`SHA_ALL_OK=true`。

| 截图 | sha256 |
|---|---|
| pc-bam-light-r497.png | db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7 |
| pc-bam-dark-r497.png | 803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4 |
| pc-log-viewer-light-r497.png | 8d90da1a969200c0f37d8b50a98c37cfd5d273473081b76eaf5122462be9b2fe |
| pc-log-viewer-dark-r497.png | 23399bb5d4dd5ceabe692026d4fdad54d4a8cc8daa724610dd90eefc5f03ddb0 |
| pc-note-light-r497.png（同 -light-sup） | 3073d4e69a41e8a9713c65923d08c8af13fb0a2b0911df0f6abc568be55bd8c0 |
| pc-note-dark-r497.png（同 -dark-sup） | 5b7c9fb6099896699a23f5222eaac51ce596ca7a26638f3de9a2283ee45ebf6b |
| pc-cms-index-light-r497.png（同 -light-sup） | 8789d54a3653b6916d2d95bfc43a33a9cdf1ab0b6202cf4ff3c1652c118da6bf |
| pc-cms-index-dark-r497.png（同 -dark-sup） | bf3c842ffd244a485abab64e7a36f1d5a17456c28a86b6915481670a0b888c16 |

目视抽验 3 张（1440×900 非物理真机）：note/light 为笔记管理页 3 行表格 + 5 列头，note/dark 为同页深态，cms-index/light 为 CMS 索引设计页 1 行表格 + 6 列头与左侧系统管理侧栏；与程序化数据名实相符。

## 8. 结论

R497 子维度在抽测的 4 页 × 双主题内：**可达真缺陷 0 项；不可达 L1 2 类（空态无表格）；正常若干（4 项有标题等价上下文的表格）**。不建议因此轮改动业务代码或全局增强器；若后续轮次扩测剩余 18 处 data-table，须坚持同一「AXTree 实证 + 等价上下文」判据，禁止仅凭缺 caption 预判。项目处于 ≥1000 轮持续迭代中，本轮仅为其中一环，非终态。
