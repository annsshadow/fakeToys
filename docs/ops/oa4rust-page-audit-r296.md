# OA4Rust 页面梳理与信息架构重构清单（R296）

> 状态：**待用户拍板**。本文基于 2026-11-20 对 85 个桌面视图 + 20 个移动页面的逐文件扫描，不改动任何代码。
> 背景：用户确认系统是「给普通员工日常办公使用」，但现状是绝大多数页面把后端 `/api/*` 端点平铺成触发按钮的调试面板，与产品定位严重错位。

## 0. 结论先行

- 85 个桌面视图中，真正面向员工的业务界面**不足 8 个**；约 **50 个是 API 端点面板/设计器**，约 **12 个是空壳/占位/重复页**。
- 侧栏 20 个入口里，只有「工作台、通讯录、工作流、日历、会议、文件、论坛、消息、个人」具备业务语义，其余（管理、程序中心、查询视图、服务器、单元管理、表单管理、视图管理、文件信息、思维导图）都是**开发工具/admin 后台**，不该出现在员工侧栏。
- 根因：项目长期按「后端端点覆盖度」驱动界面（副标题直接写 `/api/xxx/assemble/control/*`），而非按「员工要完成什么任务」设计。

## 1. 处置分类定义

| 标签 | 含义 | 处置 |
|---|---|---|
| **A 业务保留** | 员工日常工作真正需要的界面 | 保留，但重构为任务导向（去端点按钮） |
| **B 管理后台** | 管理员/开发者的配置、设计器、监控 | 从员工侧栏移出，迁到独立「系统管理」区并做权限隔离 |
| **C 合并去重** | 功能重复/多版本并存 | 合并为一个，其余下线 |
| **D 隐藏/下线** | 空壳、纯占位、裸调试面板 | 从导航移除，视情况删除 |

## 2. 桌面视图逐项清单（85）

判定依据：标题/副标题中的 `/api/*` 暴露、前 55 行 `@click` 端点按钮密度、是否有真实业务控件。

### A 业务保留（需重构交互，8）
| 视图 | 现标题 | 现状 | 建议 |
|---|---|---|---|
| Dashboard.vue | 工作台 | 首页聚合，3 按钮 | 保留，重做为「我的待办/日程/通知」聚合 |
| OrgDirectoryApp.vue | 组织通讯录 | 11 按钮，subtitle=organization/person/unit | 保留，重做为人员搜索+组织架构树 |
| ProcessWork.vue | 工作流待办 | 41 端点按钮 | **重点重构**：只留「待我处理/我发起的/已办」，其余端点隐藏 |
| CalendarApp.vue | 日历 | 月历网格，7 按钮 | 保留，唯一形态正确的业务界面 |
| MeetingApp.vue | 会议管理 | 41 按钮，混合 | 重构为「会议室预订/我的会议」，端点按钮移到详情 |
| FileManager.vue | 文件管理 | 47 按钮 | 重构为「我的文件/共享/回收」三栏，端点按钮隐藏 |
| BBSForum.vue | 论坛 | 25 按钮 | 保留，收敛版块导航，去调试按钮 |
| IMChat.vue | 即时通讯 | 11 按钮 | 保留 |

### A- 个人/设置（保留，4）
Personal.vue（个人中心）、Settings.vue（系统设置，9 控件）、SearchApp.vue（全局搜索）、RecycleApp.vue（回收站，5 按钮）。

### B 管理后台（移出员工侧栏，约 40）
设计器类：ProcessDesigner / FormDesigner / QueryDesigner / PortalDesigner / QueryViewDesigner / QueryTableDesigner / QueryStatDesigner / QueryImporterDesigner / QueryStatementDesigner / ProcessFormDesigner / ProcessDictDesigner / ProcessScriptDesigner / ProcessXformApp / CmsDictDesignerApp / CmsFormDesignerApp / CmsViewDesignerApp / CmsScriptDesignerApp / CmsXformApp / PortalDictDesignerApp / PortalPageDesignerApp / PortalScriptDesignerApp / PortalWidgetDesignerApp / ConfigDesignerApp / ServiceInvokeDesignerApp / FindDesignerApp
管理类：QueryManager / QueryManagerDeep / ProcessManagerApp / ProcessTaskCenterApp / ProcessApplicationApp / FormApp / RoleManager / CmsColumnApp / CmsColumnManagerApp / CmsModuleApp / CmsIndexApp / PortalApp / AppInfoApp / ProgramCenterApp / DeploymentApp / JPushApp / HotpicApp / CategoryApp / NoteApp / CollectApp / TemplateApp / CommonApp / BamApp / LogViewerApp / ServerApp（命令执行，高危）
→ 统一迁到 `/app/admin/*` 下的独立管理区，按角色授权；普通员工侧栏不再出现。

### C 合并去重（约 8）
- QueryManager 与 QueryManagerDeep：功能重叠，合并为一个。
- 多个 Query*Designer：收敛到设计中心 DesignCenterApp 一个入口。
- PortalDesigner 与 PortalPageDesignerApp / PortalWidgetDesignerApp：合并。
- 三个 ScriptDesigner（Cms/Process/Portal）：合并为「脚本管理」。

### D 隐藏/下线（约 12）
EmptyApp.vue（占位页面）、HomepageApp.vue（空 h1）、SSO.vue（空）、SelectorApp.vue（组件非页面）、ListLoadError.vue（组件非页面）、PdfViewerApp.vue（查看器，按需保留为弹窗）、AnnApp.vue（神经网络AI配置，端点面板）、AIChatApp.vue（35 端点按钮，非业务）、MindApp.vue（30 端点按钮）、FtSearchApp.vue（6 端点按钮）、FaceSetApp.vue（6 端点按钮）、UnitApp（未在导航但为端点面板）。

## 3. 移动端页面（20）
| 页面 | 现状 | 建议 |
|---|---|---|
| index / login / mine | 首页/登录/我的 | 保留 |
| contacts（通讯录）/ message（消息）/ chat（聊天） | 业务 | 保留 |
| calendar / meeting / process / process-start / attendance | 业务 | 保留，按桌面同款重构 |
| bbs / bbs-new / bbs-topic / doc / collect / recycle / search / browse | 业务 | 保留 |
| — | 移动端目前无「端点面板」问题，形态干净 | 主要随桌面业务化同步即可 |

## 4. 建议的新信息架构（员工侧栏）

以「员工一天要做什么」重组，替代当前 20 项平铺：

1. **工作台** — 待办、日程、通知聚合
2. **沟通** — 消息、通讯录、论坛
3. **协作** — 工作流、会议、日历、文件、文档
4. **个人** — 我的资料、设置、回收站
5. **系统管理**（仅管理员可见，折叠） — 原全部 B 类设计器/管理页

侧栏从 20 项收敛到 4 项 + 1 个管理折叠区。

## 5. 分阶段重构路线（建议）

| 阶段 | 内容 | 风险 |
|---|---|---|
| P0（已完成） | 修「管理」死链（R296） | 低 |
| P1 | 导航信息架构调整：员工侧栏收敛 + 管理区隔离（只改路由/导航，不动页面内部） | 低 |
| P2 | 样板重构：选「工作流待办」或「会议管理」1–2 个，改为任务导向交互 | 中 |
| P3 | 样板经你确认后，按同模式推广到其余 A 类页面 | 中高 |
| P4 | B 类页面迁入管理区 + C/D 类去重下线 | 中 |

## 6. 需要你拍板的决策点

1. **范围**：先做 P1 导航收敛（低风险、立刻见效），还是直接进 P2 样板重构？
2. **样板选哪个**：工作流待办 / 会议管理 / 文件管理，三选一。
3. **管理后台去留**：B 类页面是「迁到管理区保留」（开发者仍要用）还是「直接下线」？
4. **D 类空壳**：确认可删除的清单（尤其 AIChatApp / MindApp 这类已投入开发但非业务的页面）。

> 说明：本清单仅为分析与建议，未修改任何视图代码；所有判断基于文件实证，可逐项复核。
