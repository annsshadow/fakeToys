---
module: oa4rust-web
tags: [frontend, o2web-migration, parity, design-editor, roadmap]
problem_type: feature-gap-closure
created: 2026-09-04
status: in_progress
---

# OA4Rust Web 前端 100% 替代 o2web 完整实施计划

## 一、当前状态审计

### 已有能力

| 维度 | 状态 | 数据 |
|------|------|------|
| 后端模块覆盖 | ✅ 100% | 110 crates, 4684 路由 |
| API模块数 | ✅ 80个 | 含 `request()` 通配兜底 |
| Vue视图数 | ✅ 36个 | 全部接入真实API |
| 前端路由数 | ✅ 35条 | 覆盖核心业务 |
| TypeScript | ✅ 零错误 | apis + desktop 双包通过 |
| Vite构建 | ✅ 通过 | 65KB JS gzip |

### 缺口分析（o2web 86个组件 vs oa4rust-web 36个视图）

| 类别 | o2web组件 | 已覆盖 | 缺失 | 优先级 | 后端endpoint数 |
|------|-----------|--------|------|--------|---------------|
| **日常办公** | 12 | 12 | 0 | — | — |
| **CMS内容设计** | 10 | 0 | 10 | P0 | 450 |
| **服务设计** | 5 | 0 | 5 | P0 | 405 |
| **流程深化** | 5 | 2 | 3 | P1 | 344 |
| **查询深化** | 6 | 2 | 4 | P1 | 288 |
| **门户深化** | 5 | 2 | 3 | P1 | 150 |
| **系统管理** | 6 | 0 | 6 | P2 | ~10 |
| **扩展功能** | 19 | 1 | 18 | P3 | ~50 |
| **合计** | **68** | **19** | **49** | — | **~1700** |

> 注：o2web共86个组件，其中18个为子组件/工具类（如 ForumCategory、ForumDocument 等），与 oa4rust-web 的聚合视图不对应，不计入替代目标。

---

## 二、实施阶段规划

### Phase 0：基础确认（已完成）

- [x] 后端API层100%覆盖（80个API模块）
- [x] 核心业务视图36个
- [x] TypeScript零错误，Vite构建通过
- [x] 4个基础设计器（流程/表单/查询/门户）

### Phase 1：CMS内容管理系统（P0，预计3天）

**目标**：实现o2web `cms_*` 系列组件的等价功能

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | cms_Document | /app/cms-document | cms_assemble_control | 450 | CmsDocumentApp.vue |
| 2 | cms_FormDesigner | /app/cms-form-designer | cms_assemble_control | (含在450中) | CmsFormDesigner.vue |
| 3 | cms_ViewDesigner | /app/cms-view-designer | cms_assemble_control | (含在450中) | CmsViewDesigner.vue |
| 4 | cms_ScriptDesigner | /app/cms-script-designer | cms_assemble_control | (含在450中) | CmsScriptDesigner.vue |
| 5 | cms_Column | /app/cms-column | cms_core_entity | ~20 | CmsColumnApp.vue |
| 6 | cms_ColumnManager | /app/cms-column-manager | cms_core_entity | ~15 | CmsColumnManager.vue |
| 7 | cms_DictionaryDesigner | /app/cms-dict-designer | cms_assemble_control | (含在450中) | CmsDictDesigner.vue |
| 8 | cms_Index | /app/cms-index | cms_core_entity | ~10 | CmsIndexApp.vue |
| 9 | cms_Module | /app/cms-module | cms_core_entity | ~15 | CmsModuleApp.vue |
| 10 | cms_Xform | /app/cms-xform | cms_assemble_control | ~30 | CmsXformApp.vue |

**实现策略**：
- cms_Document / cms_Module / cms_Column：列表+CRUD视图（参考 DocumentApp.vue 模式）
- cms_FormDesigner / cms_ViewDesigner / cms_ScriptDesigner：编辑器视图，复用 QueryDesigner.vue 的JSON编辑模式
- cms_Index / cms_DictionaryDesigner：配置管理视图

### Phase 2：服务管理中心（P0，预计2天）

**目标**：实现o2web `service_*` 和 `program_center` 相关组件

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | service_ServiceManager | /app/service-manager | program_center | 405 | ServiceManagerApp.vue |
| 2 | service_AgentDesigner | /app/agent-designer | program_center | (含在405中) | AgentDesigner.vue |
| 3 | service_InvokeDesigner | /app/invoke-designer | program_center | (含在405中) | InvokeDesigner.vue |
| 4 | service_ScriptDesigner | /app/service-script-designer | program_center | (含在405中) | ServiceScriptDesigner.vue |
| 5 | service_DictionaryDesigner | /app/service-dict-designer | program_center | (含在405中) | ServiceDictDesigner.vue |
| 6 | AppCenter | /app/app-center | program_center | (含在405中) | AppCenterApp.vue |
| 7 | AppMarketV2 | /app/app-market | program_center | (含在405中) | AppMarketApp.vue |
| 8 | Deployment | /app/deployment | program_center | ~20 | DeploymentApp.vue |

**实现策略**：
- ServiceManager：Agent列表+CRUD（参考 ProgramCenterApp.vue，已有 agentList/agentCreate 等API）
- AgentDesigner：Agent配置编辑器（复用 JSON editor 模式）
- InvokeDesigner：服务调用配置器（表单+测试面板）
- AppCenter/AppMarket：应用列表+详情（参考 AppInfoApp.vue）

### Phase 3：流程管理深化（P1，预计2天）

**目标**：补齐流程管理缺少的视图

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | process_ProcessManager | /app/process-manager | processplatform_assemble_surface | 963 | ProcessManagerApp.vue |
| 2 | process_TaskCenter | /app/task-center | processplatform_service_processing | 222 | TaskCenterApp.vue |
| 3 | process_ApplicationExplorer | /app/app-explorer | program_center | (含在405中) | AppExplorerApp.vue |
| 4 | process_ScriptDesigner | /app/process-script-designer | program_center | (含在405中) | ProcessScriptDesigner.vue |
| 5 | process_DictionaryDesigner | /app/process-dict-designer | processplatform_assemble_designer | 122 | ProcessDictDesigner.vue |
| 6 | process_workcenter | /app/work-center | processplatform_service_processing | (含在222中) | WorkCenterApp.vue |

**实现策略**：
- ProcessManager：流程实例管理列表（参考 ProcessWork.vue，加管理Tab）
- TaskCenter：任务调度中心（定时任务列表+启停，参考 ServerApp.vue）
- AppExplorer：应用浏览器（树形结构，参考 OrgViewer.vue）

### Phase 4：查询设计深化（P1，预计2天）

**目标**：补齐查询设计的缺失视图

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | query_ViewDesigner | /app/query-view-designer | queryview | 119 | QueryViewDesigner.vue |
| 2 | query_TableDesigner | /app/query-table-designer | query_assemble_surface | 131 | QueryTableDesigner.vue |
| 3 | query_StatementDesigner | /app/query-statement-designer | query_assemble_designer | 157 | QueryStatementDesigner.vue |
| 4 | query_StatDesigner | /app/query-stat-designer | query_assemble_designer | (含在157中) | QueryStatDesigner.vue |
| 5 | query_ImporterDesigner | /app/query-import-designer | query_assemble_designer | (含在157中) | QueryImporterDesigner.vue |
| 6 | query_QueryExplorer | /app/query-explorer | query_assemble_surface | (含在131中) | QueryExplorerApp.vue |

**实现策略**：
- QueryViewDesigner：视图配置编辑器（参考 QueryManager.vue + QueryViewApp.vue）
- QueryTableDesigner：表格设计器（列配置+排序+筛选）
- QueryStatementDesigner：SQL语句编辑器（代码编辑器+执行结果）
- QueryStatDesigner：统计设计器（维度+指标配置）
- QueryImporterDesigner：导入配置器（字段映射+数据预览）
- QueryExplorer：查询浏览器（目录树+结果预览）

### Phase 5：门户设计深化（P1，预计2天）

**目标**：补齐门户设计的缺失视图

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | portal_WidgetDesigner | /app/widget-designer | portal_assemble_surface | 72 | WidgetDesigner.vue |
| 2 | portal_ScriptDesigner | /app/portal-script-designer | portal_assemble_designer | 63 | PortalScriptDesigner.vue |
| 3 | portal_PortalManager | /app/portal-manager | portal_assemble_surface | (含在72中) | PortalManagerApp.vue |
| 4 | portal_PortalExplorer | /app/portal-explorer | portal_assemble_surface | (含在72中) | PortalExplorerApp.vue |
| 5 | portal_DictionaryDesigner | /app/portal-dict-designer | portal_assemble_designer | (含在63中) | PortalDictDesigner.vue |

**实现策略**：
- WidgetDesigner：组件配置编辑器（参考 PortalDesigner.vue）
- PortalScriptDesigner：门户脚本编辑器（代码编辑器）
- PortalManager：门户管理列表（参考 PortalApp.vue）
- PortalExplorer：门户浏览器（树形导航）
- PortalDictDesigner：门户字典配置（键值对编辑器）

### Phase 6：系统管理工具（P2，预计1天）

**目标**：补齐系统管理必需的视图

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | LogViewer | /app/log-viewer | log | ~8 | LogViewerApp.vue |
| 2 | systemconfig | /app/system-config | config | ~2 | SystemConfigApp.vue |
| 3 | ConfigDesigner | /app/config-designer | config | (含在上) | ConfigDesignerApp.vue |
| 4 | ControlPanel | /app/control-panel | sysresource | ~7 | ControlPanelApp.vue |
| 5 | DesignCenter | /app/design-center | cms_assemble_control | (含在450中) | DesignCenterApp.vue |
| 6 | FindDesigner | /app/find-designer | query_assemble_designer | ~10 | FindDesignerApp.vue |

**实现策略**：
- LogViewer：日志列表+搜索+过滤（表格+搜索框，参考 QueryManager.vue）
- SystemConfig：系统配置KV编辑器（键值对表格）
- ConfigDesigner：配置设计器（参考 FormDesigner.vue）
- ControlPanel：控制面板（统计卡片+快捷操作，参考 Dashboard.vue）
- DesignCenter：设计中心入口（聚合各设计器的导航页）
- FindDesigner：查找设计器（搜索配置）

### Phase 7：扩展功能（P3，预计2天）

**目标**：补齐用户高频使用的扩展功能

| # | o2web组件 | oa4rust路由 | 后端模块 | Endpoint数 | 视图名 |
|---|-----------|------------|---------|-----------|--------|
| 1 | PdfViewer | /app/pdf-viewer | preview | 2 | PdfViewerApp.vue |
| 2 | Note | /app/note | cms_core_entity | ~5 | NoteApp.vue |
| 3 | Template | /app/template | templateform | ~7 | TemplateApp.vue |
| 4 | Search | /app/search | search | ~10 | SearchApp.vue |
| 5 | Selector | /app/selector | organization_assemble_control | ~20 | SelectorApp.vue |
| 6 | BAM | /app/bam | processplatform_assemble_bam | 91 | BamApp.vue |
| 7 | Collect | /app/collect | program_center | ~15 | CollectApp.vue |
| 8 | ThreeMember | /app/three-member | organization_assemble_control | ~10 | ThreeMemberApp.vue |
| 9 | FaceSet | /app/face-set | personal | ~5 | FaceSetApp.vue |
| 10 | ANN | /app/ann | ai_assemble_control | ~15 | AnnApp.vue |
| 11 | attendancev2 | /app/attendance-v2 | attendance_assemble_control | ~50 | AttendanceV2App.vue |
| 12 | Homepage | /app/homepage | portal_assemble_surface | ~10 | HomepageApp.vue |
| 13 | MinderEditor | /app/minder-editor | mind_assemble_control | ~20 | MinderEditorApp.vue |
| 14 | Common | /app/common | general_assemble_control | ~10 | CommonApp.vue |
| 15 | Empty / ForumCategory / ForumDocument / ForumPerson / ForumSearch / ForumSection | — | bbs_assemble_control | ~38 | 整合到 BBSForum.vue |
| 16 | appstore / appstore_application | /app/app-store | program_center | (含在405中) | AppStoreApp.vue |
| 17 | AppMarketV2_Application | /app/app-market-detail | program_center | (含在405中) | AppMarketDetailApp.vue |
| 18 | AI | /app/ai-assistant | ai_assemble_control | ~38 | 已实现(AIChatApp) |

**实现策略**：
- PdfViewer：PDF在线查看（iframe嵌入）
- Note：笔记管理（CRUD+富文本）
- Template：模板管理（列表+预览）
- Search：全局搜索（搜索框+结果列表）
- Selector：选择器组件（通用选人/选组织弹窗）
- BAM：业务活动监控（图表+时间线）
- Collect：收集管理（表单收集+数据汇总）
- ThreeMember：三方成员管理（列表+同步）
- FaceSet：人脸设置（上传+识别）
- ANN：神经网络配置（AI配置面板）
- AttendanceV2：考勤V2（增强版考勤管理）
- Homepage：首页配置（门户首页布局）
- MinderEditor：思维导图编辑器（集成 MindApp）
- Common：公共组件库（复用组件）
- BBS子组件：整合到 BBSForum.vue
- AppStore/AppMarket：应用商店（列表+详情+安装）

---

## 三、实施节奏与里程碑

```
Week 1 (Phase 1-2): CMS + 服务管理
  Day 1-2: CmsDocumentApp, CmsModuleApp, CmsColumnApp (列表CRUD)
  Day 3:    CmsFormDesigner, CmsViewDesigner, CmsScriptDesigner (编辑器)
  Day 4:    ServiceManagerApp, AgentDesigner (服务管理)
  Day 5:    AppCenterApp, AppMarketApp, DeploymentApp (应用中心)

Week 2 (Phase 3-4): 流程深化 + 查询深化
  Day 1-2:  ProcessManagerApp, TaskCenterApp, WorkCenterApp (流程管理)
  Day 3:    QueryViewDesigner, QueryTableDesigner (查询设计)
  Day 4:    QueryStatementDesigner, QueryStatDesigner, QueryImporterDesigner
  Day 5:    QueryExplorerApp (查询浏览器)

Week 3 (Phase 5-6): 门户深化 + 系统管理
  Day 1-2:  WidgetDesigner, PortalScriptDesigner, PortalManagerApp
  Day 3:    PortalExplorerApp, PortalDictDesigner
  Day 4:    LogViewerApp, SystemConfigApp, ControlPanelApp
  Day 5:    DesignCenterApp, FindDesignerApp

Week 4 (Phase 7): 扩展功能
  Day 1-2:  PdfViewer, Note, Template, Search, Selector
  Day 3:    BAM, Collect, ThreeMember, FaceSet
  Day 4:    ANN, AttendanceV2, Homepage, MinderEditor
  Day 5:    AppStore, BBS子组件整合, 清理调试
```

---

## 四、验收标准

### 功能性验收

| 指标 | 目标值 | 当前值 |
|------|--------|--------|
| o2web组件覆盖率 | ≥ 80% (69/86) | 22% (19/86) |
| 核心功能覆盖率 | 100% | 100% |
| TypeScript错误 | 0 | 0 |
| Vite构建 | 通过 | 通过 |
| 端到端可操作 | ≥ 80% | ~45% |

### 代码质量验收

- 每个新视图 ≤ 400行（保持可读性）
- 所有API调用使用类型化参数
- 新增视图遵循现有UI规范（glass-card + 深色主题）
- 无 console.log 遗留（用 tracing 替代）
- 无 `any` 类型（用 `unknown` + 类型守卫替代）

### 交付物清单

- 新增 49 个 Vue 视图文件
- 更新 `oa4rust-web/apps/desktop/src/main.ts`（注册39条新路由）
- 更新 `oa4rust-web/packages/apis/src/index.ts`（如有新增API方法）
- 更新 `docs/plans/` 进度文档
- 每次Phase完成后提交一个commit

---

## 五、风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| CMS编辑器复杂度超预期 | Phase 1延期 | 先用JSON编辑器 MVP，后续迭代可视化拖拽 |
| 服务设计器API路径不确定 | Phase 2阻塞 | 先用 `request()` 通配兜底，逐步精确化 |
| 后端API返回格式不一致 | 调试成本增加 | 统一使用 `ApiResponse<T>` 类型，增加错误处理 |
| 视图数量膨胀导致构建变慢 | 开发体验下降 | 代码分割（lazy import），按需加载 |
| 与o2web功能对比遗漏 | 覆盖率虚高 | 每个Phase完成后手工对比o2web截图 |

---

## 六、完成后预期状态

```
o2web组件总数:  86
已实现覆盖:    69 (80%)
未实现(可选):  17 (ANN/FaceSet/ThreeMember等低频功能)

核心业务闭环:  ✅ 日常办公 + 流程设计 + 表单设计 + 查询设计 + 门户设计 + CMS + 服务管理
系统设计能力:  ✅ 全流程可视化设计 + 配置管理 + 监控告警
系统管理能力:  ✅ 日志查看 + 配置管理 + 控制面板 + 部署管理
```

---

## 七、快速启动命令

```bash
# 查看当前状态
cd D:/WORKSPACE/fakeToys
grep -c "^export const.*Api = {" oa4rust-web/packages/apis/src/index.ts  # API模块数
ls oa4rust-web/apps/desktop/src/views/*.vue | wc -l                      # 视图数
grep "path:" oa4rust-web/apps/desktop/src/main.ts | wc -l                # 路由数

# 开发调试
cd oa4rust-web
pnpm --filter @oa4rust/desktop dev

# 类型检查
pnpm --filter @oa4rust/apis exec tsc --noEmit
pnpm --filter @oa4rust/desktop exec tsc --noEmit

# 构建
pnpm --filter @oa4rust/desktop build
```
