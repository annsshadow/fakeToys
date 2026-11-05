# O24-3「功能仍不可用」清单

**这份清单不进基线，也不计入棘轮数字。**
基线只统计「真正被执行到且返回 500」的路由；本清单记录的是被**降级为 `400 missing required parameter`** 的端点——它们不再 500、报错也更明确，但**功能对用户仍然不可用**。

来源：批次 6（`E23502` 实体引用族）把无法推导的父实体参数改为 400 而不是编造 id，删掉了 5 条 hotpic + 2 条 portal page 共 7 条基线行。
清空基线（批次 16）不等于这个族完成，7 条当时仍在这里。

**2026-11-05 后续裁定后：hotpic 5 条已按「全局热点图」模型转可用（见下节 A′），本清单当前只剩 portal page 2 条 400。**
历史记录保留在下方，不用 7 条旧数字覆盖现状。

## 判定口径

这 7 条缺的都是**实体引用**（不是状态值），且列均为 `NOT NULL`、无 `DEFAULT`、无 `CHECK`：

| 表 | 列 | 证据 |
|---|---|---|
| `x_hotpic` | `application`、`info_id` | `information_schema`：NOT NULL、无 DEFAULT/CHECK；唯一写入来源是 body |
| `x_portal_page` | `portal_id` | 同上；与同 crate `create_widget` 的取参口径一致（body `portalId`） |

处理原则：能从请求上下文推导就推导并断言落库值指向正确父实体；推导不出来 → 400，**不许编造合法 id**（会产生悬挂外键或写到别人的资源下）。

## 2026-11-05 裁定后的查证（A / B）

### A. portal page：前端写死的 `default` 不是真实门户 → 停止修复

现场 SQL（本地库 `oa4rust`，容器 `lyp-pg`）：

```sql
SELECT count(*) FROM x_portal;                              -- 0
SELECT count(*) FROM x_portal WHERE id = 'default';         -- 0
SELECT count(*) FROM x_portal WHERE alias = 'default';      -- 0
SELECT count(*) FROM x_portal WHERE name = 'default';       -- 0
SELECT count(*) FROM x_portal_page;                         -- 0
SELECT count(*) FROM x_portal_widget;                       -- 0
```

- `x_portal` **一行都没有**，`id/alias/name` 都没有 `default`；迁移目录里也没有 `INSERT INTO x_portal`（0 命中）。
- 因此前端 [`PortalApp.vue:437`](../../frontend/apps/desktop/src/views/PortalApp.vue#L437) 用 `portal/default` 当作门户标识，**本身就是错的**：`x_portal_page.portal_id` 存的是门户 id，而 `default` 不是任何门户的 id。
- 按裁定 A.3：**停止**，不改前端、不用 `default` 当默认值、不编造门户 id。页面创建的前置条件是先有一个真实门户行；当前部署没有，所以这两条无法通过“前端带真实 portalId”转绿。

### B. hotpic：`application='default'` / `infoId='0'` 不存在 → 登记为未完成功能（空壳）（**已被下方 A′ 取代**）

```sql
SELECT count(*) FROM x_hotpic WHERE application = 'default';   -- 0
SELECT count(*) FROM x_hotpic WHERE info_id = '0';             -- 0
SELECT application, info_id, count(*) FROM x_hotpic GROUP BY 1,2 ORDER BY 3 DESC;
--  ''      | ''       | 51
--  'test'  | 'test-id'|  1
```

- 没有任何行以 `default`/`0` 为键；`x_hotpic` 上也没有指向“应用表”的外键（`pg_constraint` 查 `x_portal_page`/`x_hotpic` 的外键，0 行）。
- 全仓按 `application` 过滤的读路径只有 hotpic 消费方，且都是**精确匹配 (application, info_id)**：
  - [`hotpic/src/lib.rs:29`](../../backend/crates/hotpic/src/lib.rs#L29)（`exists_check`）
  - [`hotpic/src/lib.rs:105`](../../backend/crates/hotpic/src/lib.rs#L105)（`list_by_app_and_info`）
  - [`hotpic_core_entity/src/lib.rs:140-142`](../../backend/crates/hotpic_core_entity/src/lib.rs#L140-L142)（`count`）
  - [`hotpic_core_entity/src/lib.rs:92-94`](../../backend/crates/hotpic_core_entity/src/lib.rs#L92-L94)（`list_by_app_and_info`）
- 结论：`default`/`0` 不是真实键，读路径也不认可占位值。按裁定 B.3：**登记为「未完成功能（无后端支撑的空壳）」**，保留 400，不删前端入口，等产品决定。
- 旁证：`hotpic_core_entity::create`（[`lib.rs:159-176`](../../backend/crates/hotpic_core_entity/src/lib.rs#L159-L176)）对 `application`/`infoId` **不校验**，缺省写空串；前端「建实体热图」只发 `{ title }`，这解释了库里 51 行空键数据。它是“能写但写的是空壳”，不是可用功能。

### A′（2026-11-05 二次裁定）hotpic：改判「全局热点图」→ 5 条已转可用

**依据**：`x_hotpic` 已有 51 行 `application=''`/`info_id=''`，证明「无所属应用 = 全局」这条生产路径事实上存在，
只是读路径不认——数据写成功了、读的时候被精确匹配丢掉（**写成功、读不到**，规模 51 行）。

**隐式约定显式化（此前只存在于数据里，没写在任何文档）**：

> `x_hotpic` 的归属键 `(application, info_id)` **两列皆为空串 = 部署级全局热点图**，在任何 application 视图下可见。

- 代码以 `GLOBAL_APPLICATION` / `GLOBAL_INFO_ID` 常量 + `HotpicScope::global()` 具名表达（`hotpic_assemble_control`），
  写路径注释写明「空键 = 部署级全局热点图」；缺省即落全局行，半截参数（只给一列）→ 400。
- 读路径统一为**精确匹配 `(application, info_id)` 或全局（两列皆 `''`）**，只用 AND/OR 精确比较：
  `hotpic::{exists_check,list_by_application_and_info_id}`、`hotpic_assemble_control::user_hotpic_application_infoId`、
  `hotpic_core_entity::{list_by_app_and_info,exists_check}`。不使用 `LIKE` / `IS NULL` 混用 / `COALESCE` 兜底。

**行为断言**（`hotpic_global_scope_contract`）：

1. 5 条写端点缺省创建 → 200，回库 `application=''`/`info_id=''` 且 title/image_url 同值（写后必读）；
2. 半截参数（只给 `application` 或只给 `infoId`）→ 400；
3. 全局行在任意 application 视图可见；特定 `(application, infoId)` 行只在该视图可见；
4. 写错 `(application, info_id)` 的行（如 `('default','0')`）仍然读不到——写侧错误不被读侧全局兜底掩盖；
5. 51 行空键在任意视图全部可见（隔离库造 51 行，`hotpic` 列表返回 51 条）。

**51 行存活证据（真实请求）**：对开发库 `oa4rust` 发 `GET /api/hotpic/user/hotpic/it-probe-view/it-probe-info`，
实测 `type=success`、`count=51`、`data` 长度 **51**。（探针测试跑完即删，不入库。）

**当前状态**：hotpic 5 条 = **可用**（真实 200 + 行为断言），不再是「无后端支撑的空壳」。

### 总表

| # | 端点 | 前端入口（文件:行 · 页面 · 按钮） | 缺的字段 | 字段业务来源 | 前端现有能力 |
|---|---|---|---|---|---|
| 1 | `POST /api/hotpic/create/hotpic` | [`HotpicApp.vue:24`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L24) · 热帖管理 ·「建热图」→ `:125-126` | `application`、`infoId` | 外部系统（该热图所属业务应用 + 应用内信息条目 id） | **无**（整页没有 application/infoId 输入；仅 `:96-97` 深度读里硬编码 `'default'`/`'0'`） |
| 2 | `GET /api/hotpic_assemble_control/create/hotpic` | [`HotpicApp.vue:23`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L23) ·「孪生端点」→ `:273` | `application`、`infoId` | 同上 | **无**（裸 GET，无 query/body） |
| 3 | `POST /api/hotpic/assemble/control/user/hotpic` | [`HotpicApp.vue:27`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L27) ·「建用户热图」→ `:133-134` | `application`、`infoId` | 同上 | **无**（只 `prompt` 了 title） |
| 4 | `POST /api/hotpic/assemble/control/update/control/config` | [`HotpicApp.vue:26`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L26) ·「存配置」→ `:131` | `application`、`infoId` | 同上 | **无**（发空 body `{}`） |
| 5 | `GET /api/hotpic_assemble_control/update/control/config` | [`HotpicApp.vue:23`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L23) ·「孪生端点」→ `:274` | `application`、`infoId` | 同上 | **无**（裸 GET） |
| 6 | `POST /api/portal/assemble/designer/page` | [`PortalApp.vue:30`](../../frontend/apps/desktop/src/views/PortalApp.vue#L30) · 门户管理 ·「建页面」→ `:296-302` | `portalId` | 父资源上下文（页面所属门户） | **无**（body 只有 `name/category/content`；页面列表 `:437` 还写死门户 flag `default`） |
| 7 | `POST /api/portal/assemble/designer/page/create` | [`PortalPageDesignerApp.vue:11`](../../frontend/apps/desktop/src/views/PortalPageDesignerApp.vue#L11) · 门户页面设计器 ·「+ 新建」→ 弹窗 `:54`「保存」→ `:247-251` | `portalId` | 父资源上下文（页面所属门户） | **无**（表单 `:48-50` / `PageForm` `:73-77` 只有 name/category/content） |

> 上表 #1–#5 为**历史登记**：2026-11-05 二次裁定后已按「全局热点图」模型转可用（见 A′），
> 缺省不再 400，而是落 `('' ,'')` 全局行；只有半截参数仍 400。#6–#7（portal page）保持 400。

> 对照：同 crate 的组件设计器 [`PortalWidgetDesignerApp.vue:40`](../../frontend/apps/desktop/src/views/PortalWidgetDesignerApp.vue#L40) 有「门户 ID」输入框（`form.portalId`，`:107` 初始化），说明前端**有能力**做这件事，只是页面这一支没做。

## 逐条说明

### 1. `POST /api/hotpic/create/hotpic`（「建热图」）

- 后端：`hotpic_assemble_control::create_hotpic`，`lib.rs` 入口处取 `application`/`infoId`，任一为空即 400。
- 前端：`prompt('热图标题:')` 后直接 `api.post(..., { title })`。
- 字段来源：外部系统。热图在 o2 语义上挂在某个业务应用的某条信息上（`application` 是应用标识，`infoId` 是该应用内的信息 id），这两个值不在“热帖管理”页的上下文中。
- 前端能力：没有。页面里唯一的相似能力是「按应用热图」按钮 `:166` 用 `prompt` 让用户填 `infoId`（`application` 仍写死 `default`）——可以复用这种交互，但当前创建路径没有。

### 2. `GET /api/hotpic_assemble_control/create/hotpic`（「孪生端点」第一条）

- 后端：与 #1 同一 handler，注册为 GET（`routes.rs:29`），因此永远读不到 body。
- 前端：`loadHotpicTwin` 里 `api.get(...)`，无参数。
- 字段来源/能力：同 #1。另外这条 GET 变体本身没有可承载参数的路径段，即使前端有值也无处可传——需要改成带 `{application}/{infoId}` 的路径或 query 才能用。

### 3. `POST /api/hotpic/assemble/control/user/hotpic`（「建用户热图」）

- 后端：`routes.rs:137-139` 把该别名路由也注册到同一个 `create_hotpic`。
- 前端：只发 `{ title }`。
- 字段来源/能力：同 #1。

### 4. `POST /api/hotpic/assemble/control/update/control/config`（「存配置」）

- 后端：`update_control_config`（`lib.rs:114`），写 `INSERT INTO x_hotpic (..., application, info_id, ...)`，缺参 400。
- 前端：`api.post(..., {})`，空 body。
- 字段来源/能力：同 #1；前端连 `name`/`enabled` 都没发，更谈不上父实体。

### 5. `GET /api/hotpic_assemble_control/update/control/config`（「孪生端点」第二条）

- 后端：同 #4 handler 的 GET 注册（`routes.rs:41-43`）。
- 前端：`api.get(...)`，无参数。
- 字段来源/能力：同 #2 与 #4。

### 6. `POST /api/portal/assemble/designer/page`（「建页面」）

- 后端：`portal_assemble_designer::create_page`（`lib.rs:323`），`portalId` 缺失即 400。
- 前端：`designerCreate('page')` 发 `{ name, category: 'default', content: {} }`。
- 字段来源：父资源上下文——页面属于哪个门户，应由“当前所在门户”提供，不应让用户手填。
- 前端能力：没有。该页是门户列表/操作面板，没有“当前门户”状态；列表接口 `:437` 用的是 `portal/default` 这种 flag，不是 `x_portal.id`。

### 7. `POST /api/portal/assemble/designer/page/create`（门户页面设计器「保存」）

- 后端：同 #6 handler，注册在 `lib.rs:509`。
- 前端：`createEp`（`:144`），提交 `{ name, category, content }`（`:247-251`）。
- 字段来源：父资源上下文（同 #6）。
- 前端能力：没有。这个设计器是“全量页面列表 + 新建”的独立页（列表 `:79` `/page/list` 不按门户过滤），没有门户选择器；可参照组件设计器的 `portalId` 输入框补，但那是用户手填，与“父资源上下文自动带出”不是同一口径。

## 结论

- 2026-11-05 状态：**当前 2 条仍为 400，5 条已转可用**（不是最初「7 条全部 400」的口径）。
- hotpic 5 条：**已修好**——采用「全局热点图」模型（`(application, info_id)` 两列空串 = 部署级全局），
  缺省写全局行、任意视图可读；真实请求读回开发库 **51 行**（`count=51`）。半截参数仍 400。
- portal page 2 条：**保持 400，不动**——`x_portal` 没有任何行（也就没有 `default`），迁移无
  `INSERT INTO x_portal` = 门户功能没有数据底座。依赖：**先有门户创建流程与至少一条真实门户行，才谈页面创建**。
  前端 [`PortalApp.vue:437`](../../frontend/apps/desktop/src/views/PortalApp.vue#L437) 写死的 `portal/default`
  单独登记为**错误常量**（不是待办默认值）。
- 这些条目不占基线（基线 0 行），不影响棘轮；它们是产品层面的待补项，不是回归。
- 在来源确定前，保持 400，不写占位 id。

## 2026-11-05 追加：流程引擎「定时器」按钮（O25）

| # | 端点 | 前端入口（文件:行） | 缺的字段 | 字段业务来源 | 前端现有能力 |
|---|---|---|---|---|---|
| 8 | `POST /api/processplatform/service/processing/timer/start` | [`ProcessManagerApp.vue:682`](../../frontend/apps/desktop/src/views/ProcessManagerApp.vue#L682) · 流程实例建/执行/取消/网关分叉/定时器 ·「定时器」按钮 → `:682` | `workId`、`fireAt` | 当前流程实例 id + 用户选择的触发时刻 | **无**（`api.post(..., {})` 发空 body，两个字段都没有） |

- 后端现状：once 任务已真实可用（真实 POST → 200、回库正确、到期恰好执行一次、重启恢复）。
  缺参时返回 **400**（明确报错），不是 500。
- 前端现状：按钮只 `prompt('目标 ID:')` 拿一个 id，但**没有**把这个 id 传给 `timer/start`；
  请求体是空对象 `{}`，`workId` / `fireAt` 均为空 → 必然 400。
- 判定：**功能对用户仍不可用**，登记在此，不计入基线；修复需前端补两个输入（流程实例 id + 触发时间）并按
  `YYYY-MM-DD HH:MM:SS` 格式化 `fireAt`。

#### 2026-11-05 后续（O27）：已修复

- 前端 `pdEngine('timerStart')` 改为二次输入：流程实例 ID + 北京时间（UTC+8），
  由 `timerStart.ts` 校验并换算成后端要求的 **UTC naive** 串后 POST；
  空 ID / 非法时间（含 `2026-02-30`）在前端直接报错，不发半成品请求。
- 守卫：`timerStart.test.ts` 12 用例（换算 4 / 非法 4 / 载荷形状 / 错误分支 / `beijingNow`），
  含规模下限断言；破坏实验去掉 -8h 换算 → 5 条红。
- 后端契约未变（O25 集成场景实证：同形状 body → 200 + 回库行）。
- 该条从「仍不可用」移出，当前清单只剩 portal page 2 条（见上）。
