# W13/W14 仓库侧闭环审计

**审计日期**：2026-09-12  
**机器基线**：[`w13-w14-repository-closure.manifest.json`](./w13-w14-repository-closure.manifest.json)  
**口径**：只审计当前仓库代码；未修改任何主功能 Vue、计划或 CI。`implemented` 只表示 manifest `acceptance` 明示的能力已有前端消费、应用路由和后端路由代码证据，不表示整个旧组件家族完全替代。

## 结论

共裁决 **53** 项：W13 的精确 `class="crud-view"` 视图 **38** 项，W14 旧组件家族 **15** 项。分类为：

| 分类 | 数量 | 含义 |
|---|---:|---|
| `implemented` | 1 | 仅 `x_component_Forum` 的论坛主页版块/主题浏览子能力；其余 Forum 子能力独立裁决 |
| `out_of_scope` | 2 | `AppMarketV2` 云市场及其应用详情/评论/在线安装生态，书面接受排除，绝不计入已实现 |
| `still_blocked` | 50 | 38 个 W13 视图全部仍阻塞；W14 的 Minder/service 及 6 个 Forum 子家族仍阻塞 |

## 判定规则与接受边界

1. **CSS 类名不等于空壳，也不等于真实实现。** 本轮没有删 `crud-view`，而是逐文件检查路由、响应式数据流、写入 URL 和领域交互。只有代码证据满足明确验收能力时才能标 `implemented`。
2. **范围外不是完成。** `out_of_scope` 必须带 `acceptedBoundary.scope/impact/owner`，且 `replacementClaim=false`；对外不得将其计入替代完成率。
3. **集成页按能力拆分。** 一个新视图可以映射多个旧组件，但每个旧组件单独裁决。例如 BBSForum 的主页读取真实，不会因此把搜索、发帖、回复、个人页和版块管理一并判成真实。
4. **后端存在不等于前端闭环。** 后端有路由但前端未消费、方法不符或 URL 错误，仍为 `still_blocked`。

接受的唯一新增范围边界是：第三方 O2 云市场目录、账号/VIP、评论、版本详情和在线安装生态不纳入本次无历史数据的 greenfield 内网替代。影响是用户不能沿用旧 O2 市场发现/安装链路；受控程序包部署需另行验收。此边界不覆盖本仓库自有应用管理，也不允许把 ProgramCenter 的 Market 卡片宣传成 AppMarketV2 实装。

## W13：38 个 `crud-view` 精确枚举

扫描限定为 `oa4rust-web/apps/desktop/src/views/*.vue` 中精确字面量 `class="crud-view"`。当前 38 项全部已有 `/app/*` 路由，但都判为 `still_blocked`：

| 视图 | 应用路由 | 声明端点 | 主要阻塞 |
|---|---|---|---|
| AnnApp.vue | `/app/ann` | `/jaxrs/ai/assemble/control/ann/list` | 通用名称/标识薄 CRUD，不能证明 ANN 配置闭环 |
| CmsColumnApp.vue | `/app/cms-column` | `/jaxrs/cms/core/entity/column/list` | 无 CMS 列定义语义 |
| CmsColumnManagerApp.vue | `/app/cms-column-manager` | `/jaxrs/cms/core/entity/column_manager/list` | 无列管理器业务交互 |
| CmsDictDesignerApp.vue | `/app/cms-dict-designer` | `/jaxrs/cms/assemble/control/dict/list` | 无字典树/数据编辑 |
| CmsFormDesignerApp.vue | `/app/cms-form-designer` | `/jaxrs/cms/assemble/control/form/list` | 无表单设计/运行契约 |
| CmsIndexApp.vue | `/app/cms-index` | `/jaxrs/cms/core/entity/index/list` | 无索引字段/构建语义 |
| CmsModuleApp.vue | `/app/cms-module` | `/jaxrs/cms/core/entity/module/list` | 查询结果一次性复制；list URL 复用于写操作 |
| CmsScriptDesignerApp.vue | `/app/cms-script-designer` | `/jaxrs/cms/assemble/control/script/list` | 无代码编辑、补全、版本 |
| CmsViewDesignerApp.vue | `/app/cms-view-designer` | `/jaxrs/cms/assemble/control/view/list` | 无视图布局/字段设计 |
| CmsXformApp.vue | `/app/cms-xform` | `/jaxrs/cms/assemble/control/xform/list` | 无 XForm 编辑/运行时 |
| CollectApp.vue | `/app/collect` | `/jaxrs/program_center/collect/list` | 前端通用 CRUD 与后端动作模型不一致 |
| CommonApp.vue | `/app/common` | `/jaxrs/general/assemble/control/list` | 多类公共能力被误建模为单资源 CRUD |
| DeploymentApp.vue | `/app/deployment` | `/jaxrs/server/deploy/list` | 无目标、资源、执行状态 |
| FaceSetApp.vue | `/app/face-set` | `/jaxrs/personal/face/list` | 无采集/上传/人脸设置语义 |
| FtSearchApp.vue | `/app/ftsearch` | `/jaxrs/ftsearch/list` | 无索引状态、搜索、运维动作 |
| LogViewerApp.vue | `/app/log-viewer` | `/jaxrs/log/list` | 日志被误建模为可编辑名称 CRUD |
| NoteApp.vue | `/app/note` | `/jaxrs/cms/core/entity/note/list` | 无正文编辑；list URL 复用于写操作 |
| PortalDictDesignerApp.vue | `/app/portal-dict-designer` | `/jaxrs/portal/assemble/designer/dict/list` | 无字典树/数据编辑 |
| PortalPageDesignerApp.vue | `/app/portal-page-designer` | `/jaxrs/portal/assemble/designer/page/list` | 无布局、模块、预览 |
| PortalScriptDesignerApp.vue | `/app/portal-script-designer` | `/jaxrs/portal/assemble/designer/script/list` | 无代码编辑、补全、版本 |
| PortalWidgetDesignerApp.vue | `/app/portal-widget-designer` | `/jaxrs/portal/assemble/designer/widget/list` | 无组件主体设计/预览 |
| ProcessApplicationApp.vue | `/app/process-application` | `/jaxrs/program_center/application/list` | 流程应用被缩减为通用 CRUD |
| ProcessDictDesignerApp.vue | `/app/process-dict-designer` | `/jaxrs/processplatform/assemble/designer/dict/list` | 无字典树/流程数据编辑 |
| ProcessFormDesignerApp.vue | `/app/process-form-designer` | `/jaxrs/processplatform/assemble/designer/form/list` | 无表单设计/运行契约 |
| ProcessManagerApp.vue | `/app/process-manager` | `/jaxrs/processplatform/assemble/surface/process_manager/list` | 无流程状态、流转、管理动作 |
| ProcessScriptDesignerApp.vue | `/app/process-script-designer` | `/jaxrs/processplatform/assemble/designer/script/list` | 无代码编辑、补全、版本 |
| ProcessTaskCenterApp.vue | `/app/process-task-center` | `/jaxrs/processplatform/service/processing/task/list` | 无办理动作/任务语义 |
| ProcessXformApp.vue | `/app/process-xform` | `/jaxrs/processplatform/assemble/designer/xform/list` | 无 XForm 编辑/运行时 |
| QueryExplorerApp.vue | `/app/query-explorer` | `/jaxrs/query/assemble/surface/explorer/list` | 无条件输入、执行、结果浏览 |
| QueryImporterDesignerApp.vue | `/app/query-importer-designer` | `/jaxrs/query/assemble/designer/importer/list` | 无映射、校验、导入设计 |
| QueryQueryApp.vue | `/app/query-query` | `/jaxrs/query/assemble/designer/list` | 无查询结构/执行验证 |
| QueryStatDesignerApp.vue | `/app/query-stat-designer` | `/jaxrs/query/assemble/designer/stat/list` | 无维度、指标、过滤、模拟 |
| QueryTableDesignerApp.vue | `/app/query-table-designer` | `/jaxrs/query/assemble/surface/table/list` | 无列编辑/建表/DDL |
| QueryViewDesignerApp.vue | `/app/query-view-designer` | `/jaxrs/query/assemble/designer/view/list` | 无过滤、排序、分页、lookup、模拟 |
| SelectorApp.vue | `/app/selector` | `/jaxrs/organization/assemble/control/person/list` | 不是可复用组织/人员/身份选择器 |
| ServiceInvokeDesignerApp.vue | `/app/service-invoke-designer` | `/jaxrs/program_center/invoke/list` | 真实写路由在 `/invoke/{flag}`；无参数设计 |
| TemplateApp.vue | `/app/template` | `/jaxrs/templateform/list` | 无模板正文、结构、预览 |
| ThreeMemberApp.vue | `/app/three-member` | `/jaxrs/organization/assemble/control/threemember/list` | 无身份、同步、权限语义 |

共同的直接证据是各页把 TanStack Query 的 `data.value` **仅在 setup 时复制一次**到 `items.value`，没有 `watch`/`computed` 跟随异步结果；同时增改删普遍把带 `/list` 的 `ep` 直接拼接复用。`CmsModuleApp.vue` 与 `ServiceInvokeDesignerApp.vue` 已在 manifest 固化代表性证据，守卫会阻止在这些模式仍存在时把条目标成 `implemented`。

## W14：15 个旧组件家族逐项裁决

| 旧组件 | 新栈映射 | 分类 | 证据与边界 |
|---|---|---|---|
| x_component_Minder | MindApp `/app/mind` | `still_blocked` | 目录树 GET 可用；列表 URL 与后端版本路由不符，无创作入口 |
| x_component_MinderEditor | MindApp `/app/mind` | `still_blocked` | 只显示 `<pre>` JSON；后端虽有 mind/folder/version 写路由，前端无画布/保存 |
| x_component_service_AgentDesigner | ProgramCenter `/app/program` | `still_blocked` | 仅列表、名称/flag 新建、启停，无 Agent 属性设计 |
| x_component_service_DictionaryDesigner | ProgramCenter `/app/program` | `still_blocked` | 仅列表与简单新建，无后端已支持的路径数据树编辑 |
| x_component_service_InvokeDesigner | ServiceInvoke `/app/service-invoke-designer` | `still_blocked` | W13 薄壳；错误复用 `/invoke/list` 写入 |
| x_component_service_ScriptDesigner | ProgramCenter `/app/program` | `still_blocked` | 仅列出/删除/执行，无编辑、补全、版本 |
| x_component_service_ServiceManager | ProgramCenter + ServiceInvoke | `still_blocked` | 没有服务管理视图和动作/权限闭环 |
| x_component_AppMarketV2 | ProgramCenter Market tab | `out_of_scope` | 排除 O2 云市场目录、账号/VIP、在线安装生态；Market 卡片不算实现 |
| x_component_AppMarketV2_Application | ProgramCenter Market tab | `out_of_scope` | 排除旧市场详情、评论、版本、在线安装页 |
| x_component_Forum | BBSForum `/app/bbs` | `implemented` | **仅主页版块和全部/推荐主题浏览**；前端 GET 与后端同路由，已有路由级测试 |
| x_component_ForumCategory | BBSForum `/app/bbs` | `still_blocked` | 新建版块按钮无 handler，无分类管理页 |
| x_component_ForumDocument | BBSForum `/app/bbs` | `still_blocked` | 发帖用不存在的 `/subject/create`；无真实详情读取闭环 |
| x_component_ForumPerson | BBSForum `/app/bbs` | `still_blocked` | “我的”仅切换列表端点，无个人主页/回复管理 |
| x_component_ForumSearch | BBSForum `/app/bbs` | `still_blocked` | 前端 POST 短路径，后端短路径只注册 GET；真实分页搜索为 PUT 另一 URL |
| x_component_ForumSection | BBSForum `/app/bbs` | `still_blocked` | 版块列表可读，但筛选前端 POST、后端 GET；无版块管理 |

`x_component_Forum` 的 `implemented` 是本轮唯一“类名/外观之外，凭代码证据确认真实”的条目：`BBSForum.vue` 消费 `/section/list` 与 `/list/subjects/index`，`main.ts` 注册 `/app/bbs`，`bbs_assemble_control/src/routes.rs` 注册相同 GET 路由，`BBSForum.test.ts` 固化首屏查询。它的 acceptance 明确不包含其他 Forum 子能力，因此不会用一个集成页掩盖家族缺口。

## 回归守卫

`w13-w14-closure.test.ts` 将机器 manifest 作为唯一分类基线，并强制：

- 当前精确 `crud-view` 集合与 W13 manifest **完全相等**，漏项或新增未裁决页面直接失败；
- 旧仓库四类前缀发现出的 W14 目录与 manifest **完全相等**；
- 只允许三种状态，summary 必须由条目重算得到；范围外条目不得 `replacementClaim=true`；
- `implemented` 必须有 acceptance、前端/路由/后端三类可定位证据，证据字符串必须实际存在；
- W13 页面若仍命中非响应式 query copy、`/list` 写复用或通用名称/标识表单，不得伪称 `implemented`；
- W13 每个视图必须仍可在 `main.ts` 找到真实路由，防止仅列文件不核路由。

该守卫不要求删 class，也不批量修改页面；后续真正补齐某项时，应先提供代码证据，再把对应 manifest 条目改为 `implemented` 并写窄化 acceptance。
