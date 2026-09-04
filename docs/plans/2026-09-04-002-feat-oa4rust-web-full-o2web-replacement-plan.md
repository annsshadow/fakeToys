---
module: oa4rust-web
tags: [frontend, o2web-migration, parity, 100-percent, roadmap, design-editor]
problem_type: feature-gap-closure
created: 2026-09-04
status: in_progress
---

# OA4Rust Web 前端 100% 替代 o2web 完整实施计划

## 一、精确数据与现状审计

### 1.1 后端能力（已完成）

| 维度 | 状态 | 数据 |
|------|------|------|
| 后端模块覆盖 | ✅ 100% | 110 crates, 4684 路由 |
| API模块数 | ✅ 80个 | 含 `request()` 通配兜底 |
| TypeScript | ✅ 零错误 | apis + desktop 双包通过 |
| Vite构建 | ✅ 通过 | 65KB JS gzip |

### 1.2 前端视图（已完成 36/84 = 43%）

**o2web 共 84 个组件目录**（已过滤 node_modules 中的空组件）。

**已有 oa4rust-web 视图 36 个，映射关系如下：**

| # | oa4rust-web 视图 | 覆盖的 o2web 组件 | 后端API模块 |
|---|-----------------|-------------------|------------|
| 1 | Dashboard.vue | (首页聚合) | — |
| 2 | OrgViewer.vue | Org | organization_assemble_control |
| 3 | ProcessWork.vue | process_Work, process_TaskCenter, process_workcenter, process_Xform, process_Application, process_ProcessManager, process_ApplicationExplorer | processplatform_assemble_surface, processplatform_service_processing |
| 4 | IMChat.vue | IMV2 (核心聊天) | — |
| 5 | Personal.vue | Profile | personal |
| 6 | Settings.vue | Setting, systemconfig (部分) | config, sysresource |
| 7 | SSO.vue | (SSO登录) | — |
| 8 | MeetingApp.vue | Meeting | meeting_control |
| 9 | AttendanceApp.vue | Attendance, attendancev2 | attendance_control |
| 10 | QueryManager.vue | query_QueryManager, query_Query, query_TableDesigner, query_ViewDesigner, query_StatementDesigner, query_StatDesigner, query_ImporterDesigner, query_QueryExplorer | query_assemble_designer, query_assemble_surface |
| 11 | AIAssistant.vue | AI | ai_assemble_control |
| 12 | PortalApp.vue | portal_Portal, portal_PortalManager, portal_PortalExplorer | portal_assemble_surface |
| 13 | HotpicApp.vue | HotArticle | cms_assemble_control |
| 14 | JPushApp.vue | (消息推送) | — |
| 15 | AppInfoApp.vue | AppCenter, AppMarketV2, appstore, appstore_application | program_center |
| 16 | CategoryApp.vue | (分类管理) | cms_core_entity |
| 17 | CalendarApp.vue | Calendar | calendar |
| 18 | FileManager.vue | File | file_control |
| 19 | BBSForum.vue | Forum, ForumCategory, ForumDocument, ForumPerson, ForumSearch, ForumSection | bbs_assemble_control |
| 20 | MindApp.vue | Minder, MinderEditor | mind_assemble_control |
| 21 | DocumentApp.vue | cms_Document, cms_Module, cms_Column, cms_ColumnManager, cms_Index, cms_Xform, cms_FormDesigner, cms_ViewDesigner, cms_ScriptDesigner, cms_DictionaryDesigner | cms_assemble_control, cms_core_entity |
| 22 | ProgramCenterApp.vue | service_ServiceManager, service_AgentDesigner, service_ScriptDesigner, service_DictionaryDesigner, service_InvokeDesigner, AppMarketV2 | program_center |
| 23 | QueryViewApp.vue | query_ViewDesigner (独立入口) | queryview |
| 24 | RecycleApp.vue | (回收站) | — |
| 25 | ServerApp.vue | (服务器管理) | server |
| 26 | UnitApp.vue | (单元管理) | unit |
| 27 | FormApp.vue | form (表单列表) | form |
| 28 | ViewApp.vue | view (视图列表) | view |
| 29 | FileInfoApp.vue | (文件信息) | file_control |
| 30 | AIChatApp.vue | AI (聊天入口) | ai_assemble_control |
| 31 | RoleManager.vue | (角色管理) | role |
| 32 | ProcessDesigner.vue | process_ProcessDesigner, process_FormDesigner, process_DictionaryDesigner, process_ScriptDesigner | processplatform_assemble_designer |
| 33 | FormDesigner.vue | cms_FormDesigner (CMS表单设计器) | cms_assemble_control |
| 34 | QueryDesigner.vue | query_ViewDesigner, query_TableDesigner, query_StatementDesigner, query_StatDesigner, query_ImporterDesigner | query_assemble_designer |
| 35 | PortalDesigner.vue | portal_WidgetDesigner, portal_ScriptDesigner, portal_DictionaryDesigner, portal_PageDesigner | portal_assemble_designer |
| 36 | PlaceholderView.vue | (占位) | — |

### 1.3 未覆盖的 o2web 组件（48个）

| # | o2web 组件 | 对应功能 | 优先级 | 后端API路径模式 | 估计复杂度 |
|---|-----------|---------|--------|---------------|-----------|
| 1 | ANN | 神经网络AI配置 | P3 | /jaxrs/ai_assemble_control/* | 低 |
| 2 | BAM | 业务活动监控(图表+时间线) | P2 | /jaxrs/processplatform_assemble_bam/* | 中 |
| 3 | Collect | 收集管理(表单收集+数据汇总) | P2 | /jaxrs/program_center/* | 中 |
| 4 | Common | 公共组件库 | P3 | /jaxrs/general_assemble_control/* | 低 |
| 5 | ConfigDesigner | 配置设计器 | P2 | /jaxrs/config/* | 中 |
| 6 | ControlPanel | 控制面板(统计+快捷操作) | P2 | /jaxrs/sysresource/* | 中 |
| 7 | Deployment | 部署管理 | P2 | /jaxrs/server/* | 高 |
| 8 | DesignCenter | 设计中心入口(聚合导航) | P2 | 多模块 | 低 |
| 9 | Empty | 空页面占位 | P3 | — | 极低 |
| 10 | FaceSet | 人脸设置 | P3 | /jaxrs/personal/* | 中 |
| 11 | FindDesigner | 查找设计器(搜索配置) | P2 | /jaxrs/query_assemble_designer/* | 中 |
| 12 | Homepage | 首页配置(门户首页布局) | P2 | /jaxrs/portal_assemble_surface/* | 中 |
| 13 | LogViewer | 日志查看器 | P1 | /jaxrs/log/* | 低 |
| 14 | Note | 笔记管理(列表+富文本) | P2 | /jaxrs/cms_core_entity/* | 低 |
| 15 | PdfViewer | PDF在线查看 | P2 | /jaxrs/preview/* | 低 |
| 16 | Search | 全局搜索 | P2 | /jaxrs/search/* | 中 |
| 17 | Selector | 通用选择器(选人/选组织) | P1 | /jaxrs/organization_assemble_control/* | 低 |
| 18 | Template | 模板管理(列表+预览) | P2 | /jaxrs/templateform/* | 低 |
| 19 | ThreeMember | 三方成员管理 | P2 | /jaxrs/organization_assemble_control/* | 低 |
| 20 | ftsearch | 全文搜索 | P3 | /jaxrs/ftsearch/* | 中 |
| 21 | cms_Column (独立) | CMS列管理(详细字段编辑) | P1 | /jaxrs/cms_core_entity/column/* | 低 |
| 22 | cms_ColumnManager | CMS列管理器(批量管理) | P1 | /jaxrs/cms_core_entity/column_manager/* | 低 |
| 23 | cms_DictionaryDesigner | CMS字典设计器(独立入口) | P1 | /jaxrs/cms_assemble_control/dict/* | 低 |
| 24 | cms_FormDesigner (独立) | CMS表单设计器(独立入口) | P1 | /jaxrs/cms_assemble_control/form/* | 低 |
| 25 | cms_Index | CMS索引设计 | P1 | /jaxrs/cms_core_entity/index/* | 低 |
| 26 | cms_Module (独立) | CMS模块管理(独立入口) | P1 | /jaxrs/cms_core_entity/module/* | 低 |
| 27 | cms_ScriptDesigner | CMS脚本设计器(独立入口) | P1 | /jaxrs/cms_assemble_control/script/* | 低 |
| 28 | cms_ViewDesigner | CMS视图设计器(独立入口) | P1 | /jaxrs/cms_assemble_control/view/* | 低 |
| 29 | cms_Xform (独立) | CMS XForm表单(独立入口) | P1 | /jaxrs/cms_assemble_control/xform/* | 低 |
| 30 | portal_DictionaryDesigner | 门户字典设计器(独立入口) | P1 | /jaxrs/portal_assemble_designer/dict/* | 低 |
| 31 | portal_PageDesigner | 门户页面设计器(独立入口) | P1 | /jaxrs/portal_assemble_designer/page/* | 低 |
| 32 | portal_ScriptDesigner | 门户脚本设计器(独立入口) | P1 | /jaxrs/portal_assemble_designer/script/* | 低 |
| 33 | portal_WidgetDesigner | 门户组件设计器(独立入口) | P1 | /jaxrs/portal_assemble_designer/widget/* | 低 |
| 34 | process_Application (独立) | 流程应用管理(独立入口) | P1 | /jaxrs/program_center/application/* | 低 |
| 35 | process_DictionaryDesigner | 流程字典设计器(独立入口) | P1 | /jaxrs/processplatform_assemble_designer/dict/* | 低 |
| 36 | process_FormDesigner (独立) | 流程表单设计器(独立入口) | P1 | /jaxrs/processplatform_assemble_designer/form/* | 低 |
| 37 | process_ProcessManager (独立) | 流程实例管理(独立入口) | P1 | /jaxrs/processplatform_assemble_surface/process_manager/* | 低 |
| 38 | process_ScriptDesigner | 流程脚本设计器(独立入口) | P1 | /jaxrs/processplatform_assemble_designer/script/* | 低 |
| 39 | process_TaskCenter (独立) | 流程任务中心(独立入口) | P1 | /jaxrs/processplatform_service_processing/task/* | 低 |
| 40 | process_Xform (独立) | 流程XForm(独立入口) | P1 | /jaxrs/processplatform_assemble_designer/xform/* | 低 |
| 41 | query_ImporterDesigner (独立) | 查询导入设计器(独立入口) | P1 | /jaxrs/query_assemble_designer/importer/* | 低 |
| 42 | query_Query (独立) | 查询定义管理(独立入口) | P1 | /jaxrs/query_assemble_designer/query/* | 低 |
| 43 | query_QueryExplorer (独立) | 查询浏览器(独立入口) | P1 | /jaxrs/query_assemble_surface/explorer/* | 低 |
| 44 | query_StatDesigner (独立) | 统计设计器(独立入口) | P1 | /jaxrs/query_assemble_designer/stat/* | 低 |
| 45 | query_StatementDesigner (独立) | SQL语句设计器(独立入口) | P1 | /jaxrs/query_assemble_designer/statement/* | 低 |
| 46 | query_TableDesigner (独立) | 表格设计器(独立入口) | P1 | /jaxrs/query_assemble_surface/table/* | 低 |
| 47 | query_ViewDesigner (独立) | 视图设计器(独立入口) | P1 | /jaxrs/query_assemble_designer/view/* | 低 |
| 48 | service_InvokeDesigner (独立) | 服务调用设计器(独立入口) | P1 | /jaxrs/program_center/invoke/* | 低 |

### 1.4 覆盖统计

| 指标 | 目标 | 当前 | 差距 |
|------|------|------|------|
| o2web组件覆盖率 | **100%** (84/84) | 43% (36/84) | 48个缺失 |
| 核心业务操作 | 100% | 100% | ✅ |
| 设计器能力 | 100% | 47% (4/8) | 4个设计器需独立入口 |
| 系统管理 | 100% | 25% | 需补齐 |
| TypeScript错误 | 0 | 0 | ✅ |
| Vite构建 | 通过 | 通过 | ✅ |

---

## 二、实施策略

### 2.1 实现原则

1. **聚合优先**：对功能相近的组件，优先合并到单一视图中（如 DocumentApp 聚合了10个CMS组件）
2. **MVP路线**：先实现 CRUD 列表视图（满足基本操作），再迭代增强设计器交互
3. **复用模式**：新视图复用现有 UI 模式（glass-card + 深色主题 + naive-ui）
4. **独立入口**：重要设计器提供独立路由（`/app/designer-name`）
5. **占位兜底**：低频组件先实现占位视图（显示"开发中"），后续逐步完善

### 2.2 实现模式库

新视图统一使用以下模式之一：

**模式A：列表CRUD视图**（适用 60% 组件）
```
顶部搜索栏 + 新建按钮
→ 数据表格（分页+排序）
→ 行操作：编辑/删除/详情
→ 创建/编辑弹窗（表单）
```
代表：LogViewer, Note, Template, Selector, ThreeMember, FaceSet, PdfViewer, Search

**模式B：编辑器视图**（适用 25% 组件）
```
左侧：列表/目录树
右侧：JSON/YAML 配置编辑器（代码编辑器）
底部：预览面板
顶部：保存/预览/导出按钮
```
代表：ConfigDesigner, FindDesigner, cms_Index, query_ImporterDesigner

**模式C：聚合导航视图**（适用 10% 组件）
```
功能卡片网格（每个卡片链接到子功能）
```
代表：DesignCenter

**模式D：可视化展示视图**（适用 5% 组件）
```
图表/时间线/统计面板
```
代表：BAM, Homepage

### 2.3 文件组织约定

- 新视图文件：`oa4rust-web/apps/desktop/src/views/[Name]App.vue`
- 路由注册：`oa4rust-web/apps/desktop/src/main.ts`
- API模块：`oa4rust-web/packages/apis/src/index.ts`（如有需要）

---

## 三、实施阶段规划

### Phase 0：基础设施确认（已完成 ✅）

- [x] 后端API层100%覆盖（80个API模块）
- [x] 核心业务视图36个
- [x] TypeScript零错误，Vite构建通过
- [x] 4个设计器（流程/表单/查询/门户）
- [x] AI聊天 + 角色管理

### Phase 1：高频缺失组件（P1，预计 2 天）

**目标**：补齐用户日常操作最频繁但尚未实现的组件

| # | o2web组件 | oa4rust路由 | 实现方式 | 视图名 |
|---|-----------|------------|---------|--------|
| 1 | LogViewer | /app/log-viewer | /jaxrs/log/* | LogViewerApp.vue |
| 2 | Selector | /app/selector | /jaxrs/organization_assemble_control/* | SelectorApp.vue |
| 3 | query_Query (独立) | /app/query-query | /jaxrs/query_assemble_designer/query/* | QueryQueryApp.vue |
| 4 | query_QueryExplorer (独立) | /app/query-explorer | /jaxrs/query_assemble_surface/explorer/* | QueryExplorerApp.vue |
| 5 | query_TableDesigner (独立) | /app/query-table-designer | /jaxrs/query_assemble_surface/table/* | QueryTableDesignerApp.vue |
| 6 | query_ViewDesigner (独立) | /app/query-view-designer | /jaxrs/query_assemble_designer/view/* | QueryViewDesignerApp.vue |
| 7 | query_StatementDesigner (独立) | /app/query-statement-designer | /jaxrs/query_assemble_designer/statement/* | QueryStatementDesignerApp.vue |
| 8 | query_StatDesigner (独立) | /app/query-stat-designer | /jaxrs/query_assemble_designer/stat/* | QueryStatDesignerApp.vue |
| 9 | query_ImporterDesigner (独立) | /app/query-importer-designer | /jaxrs/query_assemble_designer/importer/* | QueryImporterDesignerApp.vue |
| 10 | cms_Column (独立) | /app/cms-column | /jaxrs/cms_core_entity/column/* | CmsColumnApp.vue |
| 11 | cms_ColumnManager | /app/cms-column-manager | /jaxrs/cms_core_entity/column_manager/* | CmsColumnManagerApp.vue |
| 12 | cms_Index | /app/cms-index | /jaxrs/cms_core_entity/index/* | CmsIndexApp.vue |
| 13 | process_TaskCenter (独立) | /app/process-task-center | /jaxrs/processplatform_service_processing/task/* | ProcessTaskCenterApp.vue |
| 14 | process_ProcessManager (独立) | /app/process-manager | /jaxrs/processplatform_assemble_surface/process_manager/* | ProcessManagerApp.vue |

**策略**：全部采用模式A（列表CRUD）或模式B（编辑器），代码模式统一，快速批量生成

### Phase 2：CMS独立入口（P1，预计 1 天）

**目标**：为CMS模块提供独立的详细编辑入口

| # | o2web组件 | oa4rust路由 | 实现方式 | 视图名 |
|---|-----------|------------|---------|--------|
| 1 | cms_DictionaryDesigner | /app/cms-dict-designer | 键值对配置 | CmsDictDesignerApp.vue |
| 2 | cms_FormDesigner (独立) | /app/cms-form-designer | JSON表单编辑器 | CmsFormDesignerApp.vue |
| 3 | cms_ViewDesigner | /app/cms-view-designer | JSON视图配置 | CmsViewDesignerApp.vue |
| 4 | cms_ScriptDesigner | /app/cms-script-designer | 脚本编辑器 | CmsScriptDesignerApp.vue |
| 5 | cms_Xform (独立) | /app/cms-xform | XForm编辑器 | CmsXformApp.vue |
| 6 | cms_Module (独立) | /app/cms-module | 模块管理 | CmsModuleApp.vue |
| 7 | portal_DictionaryDesigner | /app/portal-dict-designer | 键值对配置 | PortalDictDesignerApp.vue |
| 8 | portal_PageDesigner | /app/portal-page-designer | JSON页面配置 | PortalPageDesignerApp.vue |
| 9 | portal_ScriptDesigner | /app/portal-script-designer | 脚本编辑器 | PortalScriptDesignerApp.vue |
| 10 | portal_WidgetDesigner | /app/portal-widget-designer | 组件配置器 | PortalWidgetDesignerApp.vue |
| 11 | process_FormDesigner (独立) | /app/process-form-designer | 流程表单设计 | ProcessFormDesignerApp.vue |
| 12 | process_DictionaryDesigner | /app/process-dict-designer | 流程字典配置 | ProcessDictDesignerApp.vue |
| 13 | process_ScriptDesigner | /app/process-script-designer | 流程脚本编辑 | ProcessScriptDesignerApp.vue |
| 14 | process_Xform (独立) | /app/process-xform | 流程XForm | ProcessXformApp.vue |
| 15 | process_Application (独立) | /app/process-application | 流程应用管理 | ProcessApplicationApp.vue |
| 16 | service_InvokeDesigner (独立) | /app/service-invoke-designer | 服务调用配置 | ServiceInvokeDesignerApp.vue |

### Phase 3：系统管理与工具（P2，预计 1 天）

| # | o2web组件 | oa4rust路由 | 实现方式 | 视图名 |
|---|-----------|------------|---------|--------|
| 1 | DesignCenter | /app/design-center | 聚合导航 | DesignCenterApp.vue |
| 2 | ControlPanel | /app/control-panel | 统计面板 | ControlPanelApp.vue |
| 3 | ConfigDesigner | /app/config-designer | KV编辑器 | ConfigDesignerApp.vue |
| 4 | FindDesigner | /app/find-designer | 搜索配置 | FindDesignerApp.vue |
| 5 | Homepage | /app/homepage | 首页布局配置 | HomepageApp.vue |
| 6 | BAM | /app/bam | 监控图表 | BamApp.vue |
| 7 | Collect | /app/collect | 收集管理 | CollectApp.vue |
| 8 | Note | /app/note | 笔记管理 | NoteApp.vue |
| 9 | Template | /app/template | 模板管理 | TemplateApp.vue |
| 10 | Search | /app/search | 全局搜索 | SearchApp.vue |
| 11 | PdfViewer | /app/pdf-viewer | PDF查看 | PdfViewerApp.vue |
| 12 | Deployment | /app/deployment | 部署管理 | DeploymentApp.vue |
| 13 | ThreeMember | /app/three-member | 三方成员 | ThreeMemberApp.vue |
| 14 | FaceSet | /app/face-set | 人脸设置 | FaceSetApp.vue |

### Phase 4：低频扩展功能（P3，预计 1 天）

| # | o2web组件 | oa4rust路由 | 实现方式 | 视图名 |
|---|-----------|------------|---------|--------|
| 1 | ANN | /app/ann | AI配置 | AnnApp.vue |
| 2 | Common | /app/common | 组件库 | CommonApp.vue |
| 3 | ftsearch | /app/ftsearch | 全文搜索 | FtSearchApp.vue |
| 4 | Empty | /app/empty | 占位 | EmptyApp.vue |

### Phase 5：补漏与优化（P3，预计 1 天）

- 检查 o2web 中剩余未覆盖组件
- 验证所有路由正确注册
- 确保 TypeScript 零错误
- 验证 Vite 构建通过

---

## 四、实施节奏与里程碑

```
Day 1 (Phase 1):   14个高频缺失组件（LogViewer, Selector, query系列, cms系列, process系列）
Day 2 (Phase 2):   16个CMS/Portal/Service独立入口组件
Day 3 (Phase 3):   14个系统管理与工具组件
Day 4 (Phase 4):   4个低频扩展功能组件
Day 5 (Phase 5):   补漏、验证、TypeScript检查、构建验证

总计：5个工作日，48个新视图，100% o2web 组件覆盖率
```

---

## 五、验收标准

### 功能性验收（100%）

| 指标 | 目标值 | 验收方法 |
|------|--------|---------|
| o2web组件覆盖率 | **100%** (84/84) | 对照 o2web/source/ 下的84个 x_component_* 目录逐一确认 |
| 前端视图总数 | 84个（含占位） | `ls oa4rust-web/apps/desktop/src/views/*.vue \| wc -l` |
| 前端路由总数 | ≥84条 | `grep "path:" oa4rust-web/apps/desktop/src/main.ts \| wc -l` |
| TypeScript错误 | 0 | `pnpm --filter @oa4rust/desktop exec tsc --noEmit` |
| Vite构建 | 通过 | `pnpm --filter @oa4rust/desktop build` |
| 端到端可操作 | 100% | 每个视图可通过路由访问，API调用返回正常数据 |

### 代码质量验收

- 每个视图 ≤ 400 行（保持可读性）
- 所有API调用使用类型化参数（禁止 `any`）
- 新增视图遵循现有UI规范（glass-card + 深色主题）
- 无遗留 console.log
- 路由名称与 o2web 组件名称保持一致命名（如 `LogViewer` → `/app/log-viewer`）

### 交付物清单

| 类别 | 数量 | 说明 |
|------|------|------|
| 新增 Vue 视图文件 | 48 个 | `apps/desktop/src/views/*.vue` |
| 更新路由注册 | 48 条 | `apps/desktop/src/main.ts` |
| API模块更新 | 如有 | `packages/apis/src/index.ts` |
| 计划文档更新 | 1 份 | 本计划文档 |
| Commit | 5 个 | 按Phase分阶段提交 |

---

## 六、风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| 编辑器复杂度超预期 | Phase延期 | 先用JSON编辑器MVP，可视化拖拽后续迭代 |
| 后端API路径不确定 | 调试成本增加 | 先用 `request()` 通配兜底，逐步精确化 |
| 视图数量膨胀导致构建变慢 | 开发体验下降 | 代码分割（lazy import），按需加载 |
| 与o2web功能对比遗漏 | 覆盖率虚高 | 每个Phase完成后手工对照o2web目录 |
| o2web组件功能不对等 | 某些组件无需单独视图 | Empty/App占位即可，无需完整实现 |

---

## 七、快速启动命令

```bash
# 查看当前状态
cd D:/WORKSPACE/fakeToys
grep -c "^export const.*Api = {" oa4rust-web/packages/apis/src/index.ts      # API模块数
ls oa4rust-web/apps/desktop/src/views/*.vue | wc -l                          # 视图数
grep "path:" oa4rust-web/apps/desktop/src/main.ts | wc -l                     # 路由数
ls oa/o2web/source/x_component_*/ 2>/dev/null | grep -c "x_component_"       # o2web组件数

# 开发调试
cd oa4rust-web
pnpm --filter @oa4rust/desktop dev

# 类型检查
pnpm --filter @oa4rust/apis exec tsc --noEmit
pnpm --filter @oa4rust/desktop exec tsc --noEmit

# 构建
pnpm --filter @oa4rust/desktop build
```

---

## 八、与旧版计划对比

| 对比项 | 旧版计划（80%目标） | 新版计划（100%目标） |
|--------|-------------------|-------------------|
| 覆盖目标 | 69/86 (80%) | **84/84 (100%)** |
| 缺失组件数 | 49个 | **48个** |
| 实施周期 | 4周 | **5天** |
| Phase数量 | 7个 | 5个（合并优化） |
| CMS组件 | 10个独立视图 | **26个**（含独立入口+聚合覆盖） |
| 新增组件 | 49个 | **48个** |
| 低频组件 | 未包含 | **ANN, Common, ftsearch, Empty 明确纳入** |

> 注：新版计划基于精确审计（o2web共84个有效组件），剔除了之前估算中重复/不存在的组件，同时补充了低频率但必要的组件（ANN, FaceSet, ThreeMember, Deployment等），实现真正意义上的100%替代。
