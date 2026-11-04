# O24-3「功能仍不可用」清单

**这份清单不进基线，也不计入棘轮数字。**
基线只统计「真正被执行到且返回 500」的路由；本清单记录的是被**降级为 `400 missing required parameter`** 的端点——它们不再 500、报错也更明确，但**功能对用户仍然不可用**。

来源：批次 6（`E23502` 实体引用族）把无法推导的父实体参数改为 400 而不是编造 id，删掉了 5 条 hotpic + 2 条 portal page 共 7 条基线行。
清空基线（批次 16）不等于这个族完成，7 条仍在这里。

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

### B. hotpic：`application='default'` / `infoId='0'` 不存在 → 登记为未完成功能（空壳）

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

## 总表

| # | 端点 | 前端入口（文件:行 · 页面 · 按钮） | 缺的字段 | 字段业务来源 | 前端现有能力 |
|---|---|---|---|---|---|
| 1 | `POST /api/hotpic/create/hotpic` | [`HotpicApp.vue:24`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L24) · 热帖管理 ·「建热图」→ `:125-126` | `application`、`infoId` | 外部系统（该热图所属业务应用 + 应用内信息条目 id） | **无**（整页没有 application/infoId 输入；仅 `:96-97` 深度读里硬编码 `'default'`/`'0'`） |
| 2 | `GET /api/hotpic_assemble_control/create/hotpic` | [`HotpicApp.vue:23`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L23) ·「孪生端点」→ `:273` | `application`、`infoId` | 同上 | **无**（裸 GET，无 query/body） |
| 3 | `POST /api/hotpic/assemble/control/user/hotpic` | [`HotpicApp.vue:27`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L27) ·「建用户热图」→ `:133-134` | `application`、`infoId` | 同上 | **无**（只 `prompt` 了 title） |
| 4 | `POST /api/hotpic/assemble/control/update/control/config` | [`HotpicApp.vue:26`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L26) ·「存配置」→ `:131` | `application`、`infoId` | 同上 | **无**（发空 body `{}`） |
| 5 | `GET /api/hotpic_assemble_control/update/control/config` | [`HotpicApp.vue:23`](../../frontend/apps/desktop/src/views/HotpicApp.vue#L23) ·「孪生端点」→ `:274` | `application`、`infoId` | 同上 | **无**（裸 GET） |
| 6 | `POST /api/portal/assemble/designer/page` | [`PortalApp.vue:30`](../../frontend/apps/desktop/src/views/PortalApp.vue#L30) · 门户管理 ·「建页面」→ `:296-302` | `portalId` | 父资源上下文（页面所属门户） | **无**（body 只有 `name/category/content`；页面列表 `:437` 还写死门户 flag `default`） |
| 7 | `POST /api/portal/assemble/designer/page/create` | [`PortalPageDesignerApp.vue:11`](../../frontend/apps/desktop/src/views/PortalPageDesignerApp.vue#L11) · 门户页面设计器 ·「+ 新建」→ 弹窗 `:54`「保存」→ `:247-251` | `portalId` | 父资源上下文（页面所属门户） | **无**（表单 `:48-50` / `PageForm` `:73-77` 只有 name/category/content） |

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

- 2026-11-05 状态：**7 条全部为 400（缺参数），0 条可用**；查证后 **7 条全部保留 400**。
- hotpic 5 条：登记为「未完成功能（无后端支撑的空壳）」——`application`/`infoId` 没有真实键、没有外键、没有可推导的父上下文，等产品决定是否做应用选择器或由业务页带入。
- portal page 2 条：**停止修复**——`x_portal` 没有任何行（也就没有 `default`），前端写死 `portal/default` 本身是错的；先要有真实门户，才谈得上带 `portalId`。
- 这 7 条不占基线（基线 0 行），不影响棘轮；它们是产品层面的待补项，不是回归。
- 在来源确定前，保持 400，不写占位 id。
