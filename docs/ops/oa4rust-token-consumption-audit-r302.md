# R302 设计令牌消费度审计与缺口清单（只读）

- 审计日期：2026-11-21（Asia/Shanghai）
- 分支：`opt1000/oa4rust-pc-ai-refactor`
- 任务：t35（R302 设计令牌消费度审计）
- 性质：**只读审计**，未改任何源码、未提交实现代码。
- 令牌规范：`oa4rust/docs/plans/design-tokens.md` 第 8 章「R300 AI 风格令牌层」+ 实现 `oa4rust/frontend/packages/ui/src/theme/ai-tokens.css`（`--ai-*` 前缀令牌与 `.ai-*` 组件类）。
- 证据来源：① 源码逐文件 grep（OPS ops_grep）；② t10 [C1] 已落盘的**测试服真浏览器**证据 `oa4rust/test-results/r300-c1-full-pc/evidence.json`（PC 74 页）与 `r300-c1-full-h5/evidence.json`（H5 19 页），采集自测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust` / `/oa4rust-h5`，无头 Chromium、非物理真机。本审计复用该真浏览器 DOM 数据，**未伪造数据、未新造截图**。

## 一、统计口径（重要）

三套口径并行，避免单一指标误导：

1. **源码 `var(--ai-*)` 引用**：PC views 目录 `*.vue` 中出现 `var(--ai-*)` 的文件数/次数（CSS 变量真实消费）。
2. **真浏览器 DOM aiPage 数**：C1 脚本对每页执行 `document.querySelectorAll('[class*="ai-page"],[class*="aiPage"]').length`（`r300-c1-full-pc.cjs:121`）。注意此口径只数 **class 名含 ai-page 的元素**，与口径 1 不等价：`Dashboard.vue` 有 11 处 `var(--ai-*)` 但 `aiPage=0`（因其用 `.dash-view` 等类而非 `ai-page`），属**口径差异**而非完全未消费。
3. **H5 DOM 令牌解析**：C1 脚本读取 `uni-page-body` 计算样式 `--ai-gradient-brand/--ai-grid-unit/--color-primary` 与 `.ai-brand-fill/.ai-surface-gradient` 元素数。

## 二、PC 端令牌消费度（74 业务+管理路由）

### 2.1 源码层

| 指标 | 数值 |
| --- | --- |
| views 目录 `.vue` 总数 | 75 |
| 含 `var(--ai-*)` 的文件 | **21 / 75（28%）**，共 175 处 |
| 含 `.ai-*` 类/CSS 消费的文件 | 24 个 |
| 含 `var(--ai-grid-*)`（8px 栅格）的文件 | 18 个 |
| 含 `var(--ai-glass-*)`（克制玻璃）的文件 | 18 个 |
| 骨架屏 `--ai-skeleton` 消费的路由视图 | 11 个（另有 `admin/AdminTable.vue` 1 个） |

已消费代表页（`var(--ai-*)`）：`MeetingApp.vue`(14)、`CmsDictDesignerApp.vue`(12)、`ConfigDesignerApp.vue`(12)、`ControlPanelApp.vue`(12)、`LogViewerApp.vue`(12)、`RoleManager.vue`(12)、`ThreeMemberApp.vue`(12)、`OrgDirectoryApp.vue`(11)、`ProcessWork.vue`(11)、`Dashboard.vue`(10)、`CalendarApp.vue`(9)、`IMChat.vue`(7)、`RecycleApp.vue`(7) 等 21 个。

**未消费 `var(--ai-*)` 的路由视图（54 个）**，代表：`OrgViewer.vue`（组织架构主视图 `/app/org`，0）、`AttendanceApp.vue`（考勤 `/app/attendance`，0）、`SearchApp.vue`（全局搜索 `/app/search`，0）、`QueryManager.vue`（查询管理 `/app/admin/query`，0）、`AppInfoApp.vue`（应用管理，0）、`HotpicApp.vue`（热帖管理，0），以及绝大多数 `Query*/Cms*/Portal*/Process*DesignerApp.vue` 管理设计器。

### 2.2 真浏览器 DOM 层（C1 evidence.json）

74 页 `aiPage`（class 含 ai-page 的元素数）分布：

| aiPage | 页数 | 占比 |
| --- | --- | --- |
| 0（无 ai-page class） | **31** | 41.9% |
| 1 | 9 | 12.2% |
| 2 | 17 | 23.0% |
| 3 | 10 | 13.5% |
| 4 | 7 | 9.5% |

即 43/74（58.1%）页 DOM 至少有一个 ai-page 容器；31 页为 0。

**aiPage=0 的 31 页**（真浏览器实测）：`dashboard`、`org`、`im`、`personal`、`settings`、`file`、`bbs`、`attendance`、`recycle`、`bam`、`note`、`search`、`admin-query`、`admin-jpush`、`admin-queryview`、`admin-form`、`admin-view`、`admin-fileinfo`、`admin-query-designer`、`admin-portal-designer`、`admin-query-explorer`、`admin-query-view-designer`、`admin-portal-dict/page/script/widget-designer`、`admin-service-invoke-designer`、`admin-find-designer`、`admin-pdf-viewer`、`admin-deployment`、`admin-common`。

**低消费 aiPage=1 的 9 页**：`directory`、`process`、`calendar`、`meeting`、`admin-portal`、`admin-program`、`admin-server`、`admin-role`、`admin-design-center`。

> 口径校准：`dashboard` 源码有 11 处 `var(--ai-*)` 但 DOM aiPage=0，说明已用令牌但未打 `ai-page` 类标记；`org`(`OrgViewer.vue`) 源码 0 处且 DOM 0，属**双重未消费**，是真缺口。

## 三、H5 端令牌消费度（19 路由）

### 3.1 令牌定义与消费分层

- `mobile/src/App.vue:40-65` 集中定义完整 `--ai-*` 令牌（49 处：8px 栅格、靛蓝→青渐变、克制玻璃、阴影、spring 动效、骨架屏、命令面板），与桌面 light 同值。**令牌层已就绪**。
- **pages 目录 19 个页面中 `var(--ai-*)` 引用为 0**——页面不直接用变量，而是通过全局 class 消费。
- 页面 class 消费（`ops_grep` 精确统计）：19/19 页面各有 **1 处** `ai-surface-gradient`（页面根容器）；`ai-brand-fill`（品牌渐变实心）仅 `index/index.vue`(2)、`login/login.vue`(1) 使用；`bbs/contacts/meeting/message/process` 5 页各 1 处骨架屏 class。

### 3.2 真浏览器 DOM 证据（C1 h5 evidence.json）

| 指标 | 结果 |
| --- | --- |
| `--ai-gradient-brand` 解析到 19 页 | 19/19（值 `linear-gradient(135deg,#4f46e5,#0891b2)`） |
| `--ai-grid-unit` = 8px | 19/19 |
| `.ai-surface-gradient` 元素 ≥1 | 19/19 |
| `.ai-brand-fill` 元素 = 0 的页 | **17/19**（仅 index=2、login=2 有） |
| 设计令牌一致性 bad 数 | 0 |

**结论**：H5 令牌"全局注入、页面浅消费"。19 页都吃了渐变变量与表面渐变 class，但 **17 页没有品牌渐变实心元素**（`branded=0`），`ai-*` 类基本只落在根容器一层，未渗透到卡片/按钮/统计/空态等组件，属"有令牌、缺深度消费"。

## 四、五类视觉缺口（对照 design-tokens.md 第 8 章）

| 维度 | 令牌/类 | PC 覆盖 | H5 覆盖 | 缺口判定 |
| --- | --- | --- | --- | --- |
| 玻璃面板 | `--ai-glass-bg/border/blur` · `.ai-panel` | 18/75 文件 | 19/19 页根容器（仅 1 层） | PC 管理页普遍缺；H5 仅根层，卡片/弹层未用 |
| 渐变 | `--ai-gradient-brand/surface/text` · `.ai-gradient-text` | 21/75 文件 | 19/19 有变量；17 页无 brand-fill | PC 54 视图未用；H5 渐变实心几乎只在首页/登录 |
| 8px 栅格 | `--ai-grid-1..8` | 18/75 文件 | 19/19 变量解析 8px | PC 多页仍用旧 4px/硬编码间距 |
| 骨架屏 | `--ai-skeleton-*` · `.o2-skeleton` | 11/74 路由视图 | 5/19 页 | 双端覆盖不足，加载态仍多为裸 `Loading...` |
| 空态引导 | （无专门令牌，依赖 `.ai-*` 空态样式） | C1 实测 **14 页空态** | C1 实测 **8 页空态** | 空态页缺 AI 风格插画/引导/渐变强调 |

补充：命令面板令牌 `--ai-cmd-*` 在 PC 仅 `Dashboard.vue`/`Personal.vue`/`AppShell.vue` 消费，H5 仅 `App.vue` 定义未落地 palette；"Ctrl/Cmd+K 真命令面板"仍是缺口（底层快捷键 `globalSearchShortcut.ts` 已就绪）。

## 五、优先级缺口清单（供后续轮次排期）

### P0 — 高流量业务页双重未消费（源码 0 + DOM aiPage=0）

1. **组织架构 `/app/org`**（`OrgViewer.vue`）：源码 0 处 `var(--ai-*)`、DOM aiPage=0，且 C1 实测 loading 常驻 + 端点调试侵入。建议下一「视觉重构」轮次优先：玻璃面板 + 8px 栅格 + 骨架屏 + 空态引导全套落地，并配 `Pages.aiStyle.*.contract.test.ts` 守护。
2. **考勤 `/app/attendance`**（`AttendanceApp.vue`）：源码 0、DOM 0、C1 空态；统计卡假数据（A3 P0）与视觉缺席叠加。建议与统计 RED→GREEN 轮次一并做 AI 风格卡片。
3. **BAM `/app/bam`**、**论坛 `/app/bbs`**、**搜索 `/app/search`**、**文件 `/app/file`**、**消息 `/app/im`**：均源码 0 或 DOM 0，为核心业务入口。建议按 B1–B5 已验证的 `.ai-panel`/`.ai-gradient-text`/`.ai-skeleton-line` 模式批量补齐。

### P1 — H5 渐变实心与组件级消费深度

4. **H5 17 页 `branded=0`**：`message/chat/process/process-start/attendance/doc/contacts/meeting/calendar/bbs/collect/browse/search/recycle/mine` 等仅根层 `ai-surface-gradient`，无 `ai-brand-fill`。建议在统计卡、主按钮、tab 选中态引入品牌渐变，目标 branded ≥1/页，保持与 `ai-visual.contract.test.ts` 一致。
5. **PC 9 页 aiPage=1**（directory/process/calendar/meeting/admin-portal/program/server/role/design-center）：令牌容器仅 1 个，建议增到 ≥2 并将玻璃/渐变下沉到面板内部组件。

### P2 — 骨架屏、空态与设计器批量覆盖

6. **骨架屏**：PC 63/74、H5 14/19 路由视图未用 `--ai-skeleton`，加载态仍是裸文本；建议抽取公共 `.o2-skeleton` 并在 loadList 场景统一替换。
7. **空态**：PC 14 页 + H5 8 页空态（多为 CMS/Query designer 与 attendance/collect/doc 等）缺 AI 风格引导；建议空态组件消费 `--ai-gradient-surface` + 提示文案。
8. **命令面板**：`--ai-cmd-*` 仅外壳零星消费，建议后续用真 palette 组件承接 Ctrl/Cmd+K，不改现有快捷键行为（design-tokens.md 8.3 约束）。

## 六、结论与边界

- PC：源码 `var(--ai-*)` 覆盖 28%（21/75），真浏览器 DOM ai-page 覆盖 58%（43/74），两个口径都显示**管理设计器与核心业务页仍有系统性缺口**；B1–B5 轮次已修饰部分页面，但未全域铺开。
- H5：令牌层 100% 注入、页面浅消费 100%，但**品牌渐变实心仅 2/19**，深度消费是主要缺口。
- 本审计为只读量化，未改源码；所有 DOM 数值来自已落盘的测试服真浏览器证据，截图为非物理真机渲染。后续按队长排期进入视觉重构 + RED→GREEN 轮次，本文档不构成项目完成结论（≥1000 轮迭代中的一环）。
