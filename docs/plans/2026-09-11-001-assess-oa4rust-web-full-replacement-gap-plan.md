---
title: "assess: oa4rust + oa4rust-web 能否完全替代 oa/o2server + oa/o2web —— 全面对比与缺口闭环计划"
type: assessment-and-plan
status: active
date: 2026-09-11
rev: 6   # v6：form 域并入 designer_route_match 对账（合并 cms_assemble_control；27 调用点/15 不闭合=4×404+5×405+6 静默）
module: oa4rust + oa4rust-web vs oa/o2server + oa/o2web
tags: [replacement-parity, gap-analysis, backend, frontend, designer-engine-contract, form-runtime, greenfield-cutover, seeded-behavior-compare, canary-dogfood]
problem_type: capability-parity-audit
---

# 全面对比：oa4rust + oa4rust-web 能否完全替代 oa/o2server + oa/o2web

**判定日期**：2026-09-11（v3 实测复核）　|　**签核对象**：技术负责人（A3）
**证据基线**：HEAD `ab25b16b`。口径分两类，逐处标注：
- **【实测 09-11】**：本次直接对当前代码跑命令复核（模块/视图/路由注册计数、桩残留、ProcessDesigner schema、crud 空壳数、`:param` 缺陷、`activities` 解析点、`056` 种子、`fail_baseline` 等）。
- **【引用基准+日期】**：沿用基准文档结论，本次未重测，须按其日期理解（端点覆盖 3092/4688 与 100% 出自 `final-coverage-sweep.md` 2026-08-25；行为比对 1212/836/1996 出自 08-25/26 本地双侧全种子实跑；V3 深逻辑 215 条分布出自 `behavior-divergence-backlog.md` 2026-08-29；`ImAction` 计数出自 residual-gaps brainstorm R6）。

审计修订记录见 §九。

---

## 〇、前提变更与判据重定义（v2 核心）

> **新增约束**：现场**没有 o2server 的历史数据**，也**不具备影子流量/灰度比对条件**。这直接推翻了 v1 里"以 R1 生产影子流量为后端放行闸门、以导入历史 `.o2` 流程为前端验收"两条设定。据此重定义：

1. **替代性质 = 能力替代（greenfield cutover），不是数据迁移。** 既然无历史数据需要搬迁，"完全替代"不再要求"在既有数据上与 Java 逐条行为一致"，只要求：**在 newly-created 数据上，oa4rust+web 提供与 O2OA 等价的业务行为，且 web 产物能被 Rust 引擎执行**。→ 不再需要迁移校验、不需要回滚旧数据、R1 不再是闸门。
2. **验证锚点从"生产 Java"换成"参考 Java + 自建种子 + 金丝雀/试用"。** 参照物是干净的 `o2server:latest` 参考实例（体现 O2OA 通用行为，而非你的历史定制——无历史数据即默认标准版），配合**双侧匹配的种子数据集**跑行为比对，再叠加新栈上的**小范围真实试用（dogfood）+ 错误预算金丝雀**。
3. **一条被现有代码"锁死"的事实**：Rust 引擎把流程定义存为 `x_process_definition.process_definition` **JSONB** 并在 `processplatform_service_processing` 按 **`activities`** 键解析；`query/portal` 同理按 O2OA 结构存储。→ **前端设计器必须产出 O2OA activities/form JSON**，这一契约目标由 Rust 引擎自身决定，与有无历史数据无关。v1 的"对齐 O2OA schema"结论**成立，但理由变了**（为内部 web↔Rust 契约，而非历史导入）。

| v1 设定 | v2 修订 |
|--------|--------|
| R1 生产影子流量 = 后端唯一放行闸门 | **废除为闸门**；改用"双侧匹配种子 behavior_compare（FAIL≤阈值）+ 金丝雀试用" |
| 前端验收：导入历史 `.o2` 流程/表单可编辑 | **改为**：web 从零创作 → 存 Rust → 引擎执行 → web 渲染 → 回传（全新建，无需导入） |
| 业务状态不对称 FAIL 靠真实数据定论 | **靠扩充种子**（两侧命中同一资源）暴露并收敛，无需生产 |
| 缺真实流程种子即可 | **必须新建真实 activities/form 种子**（现 `056` 种子是 `'{}'` 占位，workflow 行为对比形同未测） |

---

## 一、结论速览（TL;DR）

> **不能"完全"替代。** 后端接近、前端有明显硬缺口。且因"无历史数据"，这属**能力替代**判定，验证靠种子行为比对 + 端到端闭环 + 试用，而非影子流量。
>
> - **后端 oa4rust vs o2server**：**端点级 / 模块级可替代成立**（100% 路由注册、0 桩残留）。行为级接管的可信度取决于**双侧匹配种子下的 behavior_compare** 与 workflow 深语义收敛，**不再是 R1**。
> - **前端 oa4rust-web vs o2web**：**结构覆盖 ~100%，功能不可完全替代**。读侧/列表类可信，但设计器类普遍是 172 行 CRUD 空壳，且 **设计器 → 引擎 → 表单运行时的数据契约不闭合**——这才是替代的真正拦路虎。
> - **一句话**：`完全替代 = 后端(行为待种子比对收敛) ∧ 前端(未达成)`。前端缺口 + 前后端互操作契约缺口，是"完全替代"不成立的两条主因。

| 维度 | o2 侧规模 | oa4rust 侧规模 | 结构替代 | 功能替代 | 主阻断项 |
|------|----------|---------------|---------|---------|--------|
| 后端 o2server | 54 `x_` 模块【实测】 / 3092 唯一端点【引用08-25】 | 全模块有对应 crate【实测】 / 4688 路由注册【引用08-25】 | ✅ 100% | ⚠️ 待双侧种子比对 | workflow 深语义、字段/信封第三层、种子太薄 |
| 前端 o2web | 86 `x_component_*`【实测】 | 84 views（83 路由）【实测】 | ✅ ~98% | ❌ 设计器↔引擎契约断裂 | 见 §三 P0 |
| IM/协议 | XMPP/WebRTC 全栈 | WebSocket 广播 + ImAction（条数引自 R6 待核，本次 `grep message* = 0`） | — | ❌ 完整协议 | R6 设计性排除 |

---

## 二、后端对比：oa4rust vs o2server（接近可替代）

### 2.1 已达成（证据充分）

- **模块覆盖**：o2server 全部 54 个 `x_*` 模块在 `oa4rust/crates/` 均有对应 crate，并扩展 `auth / base / captcha_store / control / empower / express / ldap / mcp_server / openapi / orm / personal / preview / realtime / search / shared / signature / sms`。
- **端点覆盖**【引用 08-25，本次未重测路由数】：终扫 3092 唯一 Java 端点 100% 覆盖（2026-08-25 复核，7 项残留闭环；4 条 `{}.{}` 用整段 `Path<String>` 捕获），注册面 4688 条。重跑口径见 `final-coverage-sweep.md §六` 与 `regen_endpoints.py`。
- **桩残留（今日实测复核）**：`stub_ = 0`、`TODO = 0`、`unimplemented!/todo! = 0`。
- **验证设施（今日确认，无需生产即可用）**：`tests/behavior_comparison/` 具备 `comparator.rs`（双侧 `with_tokens` 分别登录）、`seeds/`、`seed_fixtures.sql` + `seed_fixtures_java.http.md`、`fail_baseline`、`BEHAVIOR_COMPARE_MAX_FAILS` 容忍闸门（`behavior_compare.rs:324`）。**这就是"无影子流量"下的主验证通道**，不依赖生产 Java。

### 2.2 仍不能宣布"完全替代"的原因（按 v2 判据）

1. **种子太薄，行为比对当前不可信**：CI 仅播种 Rust 侧（无 Java），FAIL 天然虚高；本地双侧基线 `1212/836/1996`，但 `fail_baseline` 仍是 `bootstrap`（未启用硬门槛）；`056` 流程种子为 `'{}'` 占位 → **workflow 执行行为实际未被比对覆盖**。要宣布替代，先把种子做到"两侧命中同一资源、含真实 activities 流程/表单"。
2. **行为语义第三层未收敛**（`behavior-divergence-backlog`）：信封 `data` vs `prompt`、列表/计数包装命名、附件响应结构、字段 camel/snake——空库掩盖，需靠扩充种子暴露并逐类收敛或 allowlist 留档。
3. **workflow 深层业务逻辑缺口**：`processplatform_service_processing` 引擎执行语义、复杂过滤查询分支未实现（V3 基线 215 条，P1 集中 cms 50 / processplatform surface 44 / org control 23）。
4. **零星排除项**：BAM 分阶段验收；Java 跨 war 错误 prompt 不一致（Rust 恒填属正确偏差，留档）。
5. **IM/XMPP/WebRTC 完整协议（R6）**：设计性排除，保留 WebSocket 广播 + ImAction 若干（条数引自 residual-gaps R6，本次 `grep message*/ImAction = 0`，**具体数字未证实**），**不等于完整协议替代**。

**后端判定**：端点/模块级可承接流量层面建议签核；"关闭参考 Java、宣布完全替代"以 **双侧匹配种子 behavior_compare 达标（FAIL 收敛并启用 fail_baseline 门槛）+ workflow 深语义闭环** 为前置。生产影子流量在此前提下**不再必需**。

---

## 三、前端对比：oa4rust-web vs o2web（不可完全替代，本计划重点）

> 旧口径（`docs/plans/2026-09-04-003` "功能深度 ~8%"）已过时：ProcessDesigner 807→**11,807 行**、FormDesigner 418→**2,839**、QueryStatementDesigner 329→**2,698**。但**行数增长 ≠ 可替代**——真正问题是**兼容性与闭环缺失**。

### 3.1 结构覆盖（已达成）

86 `x_component_*` → 84 views，83 个在 `main.ts`（537 行）逐路由注册；`AIAssistant.vue` 为唯一未路由孤儿。读侧/列表类（`OrgViewer`、`AttendanceApp`、`CalendarApp`、`QueryStatementDesigner`）具真实端点，可信。

### 3.2 P0 —— 阻断"完全替代"的硬缺口

1. **ProcessDesigner 与流程引擎数据契约不兼容（最致命）**：输出自定义 `{config:{nodes,edges}}`（BPMN 风格），而 **Rust 引擎按 O2OA `activities` 键解析 JSONB**（`service_processing/lib.rs:2476`）→ **引擎无法执行**。缺 15 类 activity（begin/manual/choice/condition/split/merge/embed/publish/delay/invoke/service/agent/cancel）、字段权限、edition 列表/diff、waypoint；事件脚本为裸 `<textarea>`；"流程地图"含 `window.__fakeProcesses` 演示兜底。
2. **无 Xform 表单运行时**：全局缺把 `surface/form/view` JSON 渲染为控件的运行时；`ProcessWork.vue` 审批提交空 `{}`。→ 即便有流程也无法填报审批。
3. **FormDesigner 契约偏差**：保存自定义 `{schema}` 到 `/jaxrs/form`，非 O2OA `moduleList`；缺移动端、DOM 树、样式刷、Actions/Events/Validation。
4. **3×ScriptDesigner（process/cms/portal）全为 172 行 CRUD 空壳**：零代码编辑、无 XScript 补全、无版本历史。【实测 09-11：`ProcessScriptDesignerApp`/`CmsScriptDesignerApp`/`PortalScriptDesignerApp` 各 172 行；service 域无脚本设计器视图（仅 `ServiceInvokeDesignerApp`，属 Invoke 设计器），实为特性整类缺失。】
5. **设计器前端调用与后端路由逐端点对账【rev6】**：`ProcessDesigner/QueryStatementDesigner/QueryManagerDeep/PortalDesigner/FormDesigner.vue` 真实调用点（27 条），对账后端路由实测，**15 条不闭合分三档**（明细+复现见 §9.4；固化于测试 `oa4rust/tests/designer_route_match.rs`）：
   - **404 无路由（4）**：`POST /designer/process`（应 `/create`）、query `GET /designer/list`、`POST /designer/execute`、portal `POST /designer/script`。
   - **405 方法不符（5）**：query `PUT /update/{id}`、`POST /stat/do`、portal `PUT /script/{id}`、`POST /script/run`、form `POST /form/submit`。
   - **静默影子误路由（6，GET 进错 handler，生产带 DB 即 200 错体，最高危）**：process `process/list`、`process/export`；query `table/list`；portal `page/list`、`script/list`；form `form/list`。
   - form CRUD（`GET/PUT form/{id}`、`POST form`）路径存在，其真缺口是 §3.2 P0-3 的 schema 层（写 `{schema}` 非 `moduleList`）。→ W6 按 §9.4 执行，优先收窄宽路由消静默档。

### 3.3 P1 —— 显著差距

- **~38 个 crud-view 空壳（172–175 行）** 冒充设计器/管理器：全部 `Cms*`/`Portal*`/`Process*Designer|Manager|Xform`、`Query{Table,View,Stat,Importer,Explorer,Query}App`、`ServiceInvokeDesignerApp`、`AnnApp`、`NoteApp`、`CollectApp`、`TemplateApp` 等。
- **PortalDesigner**：有页面/脚本 CRUD，但无拖拽布局、无 60 模块类型、无 widget 主体设计器。
- **QueryTable/ViewDesigner**：列编辑/建表 DDL、可视化视图（过滤/排序/分页/simulate/bundle/lookup）为空壳。
- **Selector（组织/人员选择器）**：核心复用组件缺位（仅 crud 壳，`packages/ui` 无组件）——**阻塞任何真实流程/表单编辑器**。
- **IMChat**：纯文本，缺 12 类富媒体/交互能力。
- **API 层质量缺陷**【实测 09-11】：`packages/apis/src/index.ts` 含 **11 处未插值路径参数字面量**（如 `:1214 getid:(_id)=>api.get('/jaxrs/person/empower/:id')`，另有 `/generate/:model`、`/paging/:page`、`/size/:size`、`/room/:room`、`/classname/:className`），参数被接收却不进 URL → 调用必错；部分分组仅只读壳函数。

### 3.4 P2

Minder 无画布编辑器（仅列表）；`service_AgentDesigner`/`ServiceManager`/`DictionaryDesigner`、`AppMarketV2`/`appstore`、`Forum*` 家族无完整对应。

---

## 四、横切缺口：前后端"互操作闭环"是替代门槛

即使两侧各自"存在"，不共享契约仍不构成替代。三处断裂：① 流程定义（web 产物 ≠ 引擎可执行，且现无真实种子验证）；② 表单（设计/存储/渲染三者不闭环）；③ 查询/门户/表单（设计器↔后端路由**已逐端点对账，见 §9.4：15 条不闭合 = 4×404 + 5×405 + 6 静默影子误路由**，含 form 域）。**判定单位 = 端到端业务闭环（创作→保存→执行→渲染→回传），非单侧结构覆盖率。** 因无历史数据，闭环只需"新建即可跑通"，不必"导入旧数据再跑通"。

---

## 五、明确不替代 / 已接受边界（书面声明）

- IM/XMPP/WebRTC **完整协议**（R6）：v1 排除，保留 WebSocket + ImAction 作部分能力，须对外列明。
- **无历史数据迁移**：不提供向 Java 旧库导入能力，也不承诺旧数据行为回放——这是范围定义而非缺陷。
- **参考锚点局限**：验证对齐的是 `o2server:latest` 通用行为，不含任何历史定制（无数据即默认标准版）。若日后发现有 fork 定制，须另立对齐轨道。
- Java 错误信封 prompt 跨 war 不一致：Rust 恒填为正确偏差，留档。

---

## 六、落地规划（缺口闭环 · v2）

> 角色：A1 运维/部署、A2 后端、A3 技术负责人、A5 工具链、A6 前端；A4 文档。所有工作流以"端到端闭环"为验收单位。**先决：先建验证地基（种子+门槛+闭环脚手架），再谈功能补齐，否则无法证明。**

### 阶段 P-Setup —— 验证地基（无影子流量下的替代性验证设施）【最高优先，1–2 周】

- **S1（A2｜双侧匹配种子）**：扩 `tests/behavior_comparison/seeds/` + `seed_fixtures.sql`/`.java.http.md`，使组织/人员/内容/流程/查询域**两侧命中同一批资源**；重点补 **`056` 真实 O2OA activities 流程种子 + 一份 form 定义种子**（当前 `'{}'` 使 workflow 行为比对失效）。目标：把"业务状态不对称"型 FAIL 从"数据缺失假象"转成"可判定的真语义差异"。
- **S2（A2/A5｜启用 CI 行为门槛）**：跑一次稳定双侧全种子绿色基线，把 `fail_baseline` 从 `bootstrap` 固化为整数、`BEHAVIOR_COMPARE_MAX_FAILS` 设为该值，CI 增"仅回归（observed>baseline）失败"。→ 替代 R1 成为可自动执行的放行闸门。
- **S3（A6｜闭环冒烟脚手架）**：建 E2E（如 Playwright + 测试账户）与"设计器→保存→执行"契约测试（pin `activities`/`moduleList`/query/portal JSON schema），作为前端闭环的自动化验收。
- **S4（A1｜金丝雀/试用替代影子流量）**：无生产镜像流量，改为在小范围 pilot 上**只用新栈产生真实使用数据**，以错误率/接口 5xx 预算 + 抽样人工走查替代"影子比对"，观察期后放大。

### 阶段 P0 —— 前端设计器↔引擎↔表单闭环（本计划核心）

- **W3（A6/A2｜ProcessDesigner 契约对齐）**：让 `ProcessDesigner.vue` **读入并输出 O2OA activities JSON**（15 类 activity、字段权限、edition、waypoint、事件脚本挂 XScript），移除 `__fakeProcesses` 兜底，保存走后端新建路由（配 W6）。**验收（新建式）**：web 从零画一条流程 → 保存 → Rust 引擎成功发起实例（由 S1 真实种子 + S3 契约测试守护），**不再要求导入历史 `.o2`**。
- **W4（A6｜Xform 表单运行时）**：新建运行时渲染 `surface/form/view` JSON → 控件；`ProcessWork.vue` 审批携带真实数据。**验收**：一个带表单的新流程可填报→提交→待办可见→审批通过。
- **W5（A6/A2｜FormDesigner 契约）**：保存改 O2OA `moduleList`，补移动端/DOM 树/Actions-Events-Validation，与 W4 共用渲染契约。

### 阶段 P1 —— 设计器与共享组件补齐

- **W6（A2｜设计器路由闭合，验收即 `tests/designer_route_match.rs` 转绿到"目标态"）**：实测对账已固化为该测试。三类动作按危险度排序——① **先消 5 条静默档**：收窄 `process/{id}`、`table/{flag}`、`page/{id}`、`script/{id}` 宽路由（给字面量段 `list/export` 设专用路由或提优先级），否则"列表页"拿到"单资源"错数据仍 200；② 前端改名命中既有：process 新建 `POST /designer/process`→`/create`（实测 `/create` 匹配）、query 保存 `PUT /update/{id}`→`POST /save/{id}`；③ 后端补缺：query 裸 `list`/`execute`、portal 裸 `POST /script`（实测 404）。每闭合一条，把测试对应 case 从当前实测档改为 `Matched` 并附真实语义 → CI 死链回归（对齐 `92d09e1f` behavior gate）。
- **W7（A6｜3×ScriptDesigner + service 域）**：`Process/Cms/PortalScriptDesignerApp`（现各 172 行空壳）复用已引入 CodeMirror 落真实编辑器 + XScript 补全 + 版本历史；service 域脚本设计器视图当前缺失，按需补或书面排除。
- **W8（A6｜Selector 共享组件）**：`packages/ui` 建组织/人员/身份选择器，接入 W3/W4/W5。**（先决：无它则负责人/权限无法闭环。）**
- **W9（A6｜Portal & Query 设计器）**：Portal 拖拽布局 + 模块类型 + widget 设计器；QueryTable 列编辑/建表、QueryView 可视化过滤排序分页/simulate/bundle。
- **W10（A6｜IMChat 富媒体）**：图片/文件/语音/视频/位置/链接卡片、引用/撤回/合并转发/收藏/拖拽上传；传输走既有 WebSocket，不承诺 XMPP 全协议（对齐 R6）。
- **W11（A6｜API 层质量）**：修 `packages/apis` 中 11 处未插值路径参数字面量（`:id`/`:model`/`:page` 等，见 §3.3）为正确模板插值、补齐只读壳为真实调用。

### 阶段 P2 —— 长尾与后端语义收敛

- **W12（A2｜后端语义第三层）**：在 S1 富种子下收敛 `behavior-divergence-backlog` 的 workflow 深语义 + 字段/信封层（camel/snake、`data`/`prompt`、附件响应、BAM 月键对象）；以 S2 的 FAIL 门槛下降为度量。
- **W13（A6｜crud-view 空壳裁决）**：~38 空壳逐个"补齐 or 声明范围外"，禁止以壳冒充实装；纳入回归守卫（复用 `autoquery-guards.test.ts`）。
- **W14（A6｜长尾视图）**：Minder 画布、`service_*Designer`、`AppMarketV2`、`Forum*` 按需补齐或书面排除。

### 依赖顺序（关键路径）

```
S1 种子 + S2 门槛 + S3 闭环脚手架 + S4 金丝雀（P-Setup，先做）
        │
W8 Selector ─┐
W6 后端路由 ─┼─► W3 ProcessDesigner 契约 ─► W4 Form 运行时 ─► W5 FormDesigner ─► E2E 闭环
             │                                          │
             └─► W7/W9/W10/W11（并行）                  └─► W12/W13/W14（持续）
```

---

## 七、验收总纲（v2："可完全替代"的定义，无影子流量版）

1. **后端**：在**双侧匹配富种子**（S1）下的 `behavior_compare` 达 FAIL≤固化阈值（S2 门槛生效），其中 workflow 执行域有真实种子覆盖；参考 `o2server:latest` docker 可关停。
2. **前端 E2E 闭环**（W3–W5 + S3 契约测试，缺一不可宣布替代）：
   - 在 web **从零**创作流程/表单/查询/门户 → 保存 → Rust 引擎执行 → web 渲染 → 审批回传，全链路成功；
   - 四设计器保存 round-trip 无 404；Selector 在四设计器可复用；
   - **不含**任何"导入历史数据"验收项（无此数据）。
3. **无伪装实装**：38 crud-view 全部转"真实装"或"书面范围外"；`views/` 不得以壳冒充分替代。
4. **金丝雀试用通过**（S4）：pilot 真实使用一个观察窗口内 5xx/错误预算达标，替代"影子流量无差异"这一原 v1 判据。
5. **书面边界声明**：IM 完整协议、无历史迁移、参考锚点局限、Java prompt 不一致——对外披露。

> **未满足 1、2 之前不得宣布完全替代**；v1 的"R1 影子流量报告归档"要求已删除，由 S1+S2+S4 三件套取代。

---

## 八、相关文档

- `docs/REPLACEABLE-oa4rust-2026-09-01.md` — 后端端点级可替代判定
- `docs/audits/final-coverage-sweep.md` — 端点对齐终扫 + 行为实跑（1212/836/1996）
- `docs/audits/behavior-divergence-backlog.md` — 后端语义差异分类
- `docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` — 后端残差需求 R1–R10（R1 在 v2 中降级为非必需）
- `oa4rust/tests/behavior_comparison/`（`seeds/`、`fail_baseline`、`behavior_compare.rs:324`）— 无影子流量下的主验证通道
- `oa4rust/migrations/056_seed_process_definition_test_data.sql` — 流程种子当前为 `'{}'` 占位，S1 待实化
- `docs/plans/2026-09-04-003-frontend-completion-audit.md` — 前端旧口径（~8%，已被本计划 §三取代）
- `docs/plans/2026-09-09-001-...httponly-cookie-cargo-audit-rustsec-db-biome-plan.md` — 安全/工具链加固（正交轨道）

---

## 九、v3 审计修订记录（逐条实测复核）

**方法**：HEAD `ab25b16b`，2026-09-11 对 v2 每条结论跑命令独立复核（不只依赖子代理报告）。

### 9.1 复核确认（证据充分，已加"实测"标注）

| 结论 | 复核命令/位置 | 结果 |
|------|--------------|------|
| ProcessDesigner 输出 `{config:{nodes,edges}}`、`gate_and/xor`，非 `activities` | `grep -oE "config:\{\|nodes:\|edges:\|gate_and\|activities" ProcessDesigner.vue` | ✅ 命中 config/nodes/edges/gate_*，**无 activities** |
| 引擎按 `activities` 解析 | `service_processing/lib.rs:2476 "\"activities\" =>"` | ✅ |
| `window.__fakeProcesses` 演示兜底 | `ProcessDesigner.vue:11717` | ✅ |
| 无 Xform 运行时 | 全仓 `grep -rln "resolveComponent\|component :is\|formField.*widget"` | ✅ **0 命中** |
| 审批提交空 `{}` | `ProcessWork.vue:147 approve ... post(.../approve, {})` | ✅ |
| crud-view 空壳 ~38 | `grep -rlE 'class="crud-view"' views/ \| wc -l` | ✅ 38 |
| Xform/ScriptDesigner 视图为薄壳 | ProcessXform/CmsXform/各ScriptDesigner `wc -l`=172 | ✅ |
| `:param` 未插值缺陷 | `grep -coE "/[a-z]+/:[a-zA-Z]+" apis/index.ts` | ✅ 11 处（empower/:id 等） |
| AIAssistant 孤儿、main.ts 537 行 | `grep -c AIAssistant main.ts`=0；`wc -l main.ts`=537 | ✅ |
| Selector 缺 ui 组件 | `packages/ui/src` 仅 AppShell/Login/WindowPanel/OAuthCallback | ✅ |
| 桩残留 0 / 056 种子 `{}` / fail_baseline `bootstrap` / CI 仅播 Rust 侧 | 见 §2.1/§2.2 | ✅ |

### 9.2 修订项（v2 有误或过度陈述，已改）

1. **§3.2 P0-5「query/portal designer 无后端路由 → 404」**：overstated。实测 `query_assemble_designer` 有 `table/list`、`portal_assemble_designer` 有 `/page` 族、`processplatform_assemble_designer` 有 `/designer/create`。→ 改为"前端调用路径与后端路由形状/方法**需逐端点对账**，不预设缺路由"；W6、§四③ 同步收紧。
2. **§3.2 P0-4「4×ScriptDesigner」**：service 域无脚本设计器视图（仅 `ServiceInvokeDesignerApp`）。→ 改为 **3×（process/cms/portal）** + service 域整类缺失。
3. **§2.2.5 / 结论表「ImAction×33」**：`grep message*/ImAction = 0`，×33 未证实（出自 R6 文）。→ 去数字，标注"具体数字未证实"。
4. **数字溯源**：端点 3092/4688 与 100%、行为 1212/836/1996、V3 215 条分布均标注为**引用基准+日期**（本次未重测），区别于【实测 09-11】项；frontmatter 升 rev 3 并加证据口径说明。

### 9.3 未纳入本轮实测、仍属引用/待验

- 端点总数与 100% 覆盖：沿用 `final-coverage-sweep.md`（08-25）；如需硬证可跑 `regen_endpoints.py` 复扫。
- 各 `Cms*/Portal*/Query*` 设计器"具体缺哪类交互"：来自子代理功能级核对，属定性结论，W9/W13 执行时按端点级再确认。
- BAM 分阶段缺口、字段/信封第三层聚类计数：引用 `behavior-divergence-backlog.md`（08-29）。

### 9.4 设计器↔后端路由逐端点对账（rev6：27 调用点；process/query/portal 已 oneshot 实跑，form 依 route-table 精确推导）

方法（下表逐格为对账结论；**以末尾"合计（rev6）"为准**——query `update/{id}`、`stat/do`、portal `script/run`/`script/{id}`、form `submit` 为 405 非 404；`delete/{id}`/form CRUD 匹配）。从 `ProcessDesigner/QueryStatementDesigner/QueryManagerDeep/PortalDesigner/FormDesigner.vue` 抽真实 `api.<verb>(url)` 调用点，按 `crates/*/src/{lib,routes}.rs` 全路径 + 方法链核对。图例：✅ 匹配 / ❌ 404 无路由 或 405 方法缺 / ⚠️ 静默影子(命中宽 `{id}`，200 错体)。

**Process designer**（base `/jaxrs/processplatform/assemble/designer`）
| 前端调用 | 后端路由 | 判定 |
|---|---|---|
| GET `process/{id}` | `process/{id}` (get) | ✅ |
| PUT `process/{id}` | `process/{id}` (put) | ✅ |
| POST `process`（裸） | 无裸；新建为 POST `create` | ❌ |
| GET `process/list` | 命中 `process/{id}`（id=`list`） | ⚠️ |
| GET `process/export` | 命中 `process/{id}`（id=`export`） | ⚠️ |

**Query designer**（base `/jaxrs/query/assemble/designer`）
| 前端调用 | 后端路由 | 判定 |
|---|---|---|
| GET `list` | 仅 `list/{category}`；裸 `list` 无 | ❌ |
| PUT `update/{id}` | 无 `update/`；后端为 POST `save/{id}` | ❌ |
| POST `create` | `create` (post) | ✅ |
| DELETE `delete/{id}` | `delete/{id}` | ✅ |
| POST `execute` | 无裸 `execute`；仅 `table/execute/{flag}` | ❌ |
| GET `table/list` | 命中 `table/{flag}`（flag=`list`） | ⚠️ |
| GET `entity/entity/properties/{tbl}/default/default` | `entity/entity/properties/{query}/{category}/{entityCategory}` | ✅ |
| POST `stat/do` | 命中 `stat/{id}`（id=`do`），但 `stat/{id}` 无 POST | ❌ 405 |

**Portal designer**（base `/jaxrs/portal/assemble/designer`）
| 前端调用 | 后端路由 | 判定 |
|---|---|---|
| GET `page/list` | 仅 `page/list/{category}`+`/portal/{id}`；裸命中 `page/{id}` | ⚠️ |
| GET `script/list` | 仅 `script/list/{paging,portal,manager}`；裸命中 `script/{id}` | ⚠️ |
| PUT `page/{id}` | `page/{id}` (get/put/delete) | ✅ |
| POST `page`（裸） | `page` (post create_page) | ✅ |
| DELETE `page/{id}` | `page/{id}` delete | ✅ |
| PUT `script/{id}` | `script/{id}` 仅 GET | ❌ |
| POST `script`（裸） | 无裸 `script` | ❌ |
| POST `script/run` | 无 | ❌ |

**Form designer**（base `/jaxrs/form`，`cms_assemble_control::router` 承载；路由见 `cms_assemble_control/src/routes.rs:400-630`；schema 层另列 P0-3）
| 前端调用 | 后端路由 | 判定（route-table 精确） |
|---|---|---|
| GET `form/list` | 命中 `form/{id}`(get, L404) id=`list`；真实列表为 `form/list/all` | ⚠️ 静默影子 |
| GET `form/{id}` | `form/{id}` (get, L404) | ✅ |
| PUT `form/{id}` | `form/{id}` (put, L493) | ✅ |
| POST `form`（裸） | `/jaxrs/form` (post, L492) | ✅ |
| POST `form/submit` | 无 `form/submit`；命中 `form/{id}` 但无 POST 变体 | ❌ 405 |

**合计（rev6）**：现覆盖 **27 个前端真实调用点**（process/query/portal 22 + form 5），固化于提交测试 **`oa4rust/tests/designer_route_match.rs`**（合并 `cms_assemble_control::router`）。**15 条不闭合按 HTTP 码分三档**：
- **404 Absent（4）**：process `POST /process`、query `GET /list`、`POST /execute`、portal `POST /script`。
- **405 Method（5，含 form）**：query `PUT /update/{id}`、`POST /stat/do`、portal `PUT /script/{id}`、`POST /script/run`、form `POST /submit`。
- **静默影子误路由（6，含 form）**：process `process/list`、`process/export`；query `table/list`；portal `page/list`、`script/list`；**form `GET /form/list`（命中 `form/{id}`，真列表是 `form/list/all`）**。
- form CRUD 三件套（`GET/PUT form/{id}`、`POST form`）**实测路径存在**，但 §3.2 P0-3 的 **schema 写 `{schema}` 而非 `moduleList` 仍是不闭合根因**（route 匹配 ≠ 功能可替代）。

**证据强度分层（fail loud）**：process/query/portal 的 22 条**已于 rev5 用 oneshot 实跑并全绿**（HEAD `ab25b16b`）；form 5 条**本轮加入同一测试但尚未执行**——当前 Linux 沙箱无 Rust 工具链、仅有上轮的旧 Windows `.exe`，无法重跑。其分类由 `cms_assemble_control/src/routes.rs:400-630` **逐字路由表推导**（与 rev5 完全吻合的静态法），`cargo test --test designer_route_match`（Windows/host）跑一次即固化。
