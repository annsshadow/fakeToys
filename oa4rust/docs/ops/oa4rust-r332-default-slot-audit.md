# R332 PC 默认插槽静默丢弃候选页面群 — 只读甄别报告

## 1. 范围与环境

- 轮次：R332，任务 t132；性质：只读甄别，不改源码/契约/配置/后端。
- 对象：PC 端使用 AdminToolbar / AdminTable 的系统管理页面群（R329 已单独判定 DocumentApp.vue，本轮不重做）。
- 环境：测试服 <YOUR_TEST_HOST>，浏览器为本地 Playwright Chromium headless，视口 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 部署脚本与 ssh 通道读取，未入库、未回显；本文域名/主机统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位。
- 方法：ops/grep 精确重算差集 → 逐页源码精读 → 真浏览器抓 DOM 按钮存在性 + 截图 + HTTP 状态 + pageerror。

## 2. 根因（组件插槽契约）

- AdminToolbar.vue:17 仅 <slot name="filters"/>，:20 仅 <slot name="actions"/>，:21 内置刷新按钮，:22 <button v-if="createLabel">，组件内**无默认 <slot/>**。
- AdminTable.vue:13 渲染 table.admin-table-grid，:36 <slot :name="'cell-'+col.key" :row :value> 且带默认回退到 row[col.key]，组件内**无默认 <slot/>**。
- 因此写在 AdminToolbar / AdminTable 标签对之间的默认插槽内容（按钮）被 Vue 静默丢弃，编译期与运行期均无告警，pageerror 全程为 0。这与 R329 判定的 DocumentApp.vue 属同一根因（commit 6545b6e39，P1）。

## 3. 差集精确重算（要求①）

ops/grep 得到使用 AdminToolbar 或 AdminTable 的 .vue 共 24 个（23 个 views + 组件自身）。逐页精读排除：具名插槽被真实消费者、组件自闭合且按钮在组件外者、自造表格者、R329 已判者。结果：

- 真候选（默认插槽写按钮且被丢弃）：**AppInfoApp.vue、HotpicApp.vue**（2 个）。
- 非候选（11 个）：ProcessApplicationApp / ProcessDictDesignerApp / ProcessFormDesignerApp / ProcessXformApp / QueryImporterDesignerApp / QueryQueryApp / QueryStatDesignerApp 均 AdminToolbar 自闭合、按钮在组件外；QueryExplorerApp / QueryViewApp 自造结构且无 AdminTable；ProcessManagerApp / ProcessTaskCenterApp 的 #group-read/#group-write 由 EndpointFold 组件消费（见 §4.4）。
- 已判：DocumentApp.vue（R329，不在本轮重做）。

## 4. 逐页三分判定

### 4.1 AppInfoApp.vue（路由 /app/admin/appinfo，main.ts:152-154）— 【真缺陷 P1】

- 源码：:14-22 AdminToolbar 默认插槽写 6 个入口（:17 新建应用、:18 管理/视图/类型、:19 发布/全部视图、:20 受控栏目/带流程、:21 类型管理/含文档类型）；:24-46 AdminTable 默认插槽写 5 个行操作（:38 权限、:39 设权限、:40 建文件、:41 图标、:42 删除）；columns 含 操作 列。
- 真浏览器 DOM：adminToolbar=1、toolbarSearch=1、toolbarBtn=1（仅内置刷新），btnCreate=0；adminTable=1、adminTableRow=1，btnAct=0/btnAct2=0/btnDel=0；6 个标签命中数全 0；表头第 4 列为 操作 但 td 全为 —。allButtons 仅 [☾深色模式, ☰, ⌘K, 🔔, 刷新, 标准, 紧凑]。
- HTTP：GET /oa4rust-api/api/appinfo/list/all = 200（列表接口正常，页面加载无报错）。
- 判定：与 DocumentApp.vue 同构的双重默认插槽丢弃，管理入口全部不可见，P1 真缺陷。pageerror=0。

### 4.2 HotpicApp.vue（路由 /app/admin/hotpic，main.ts:140-142）— 【真缺陷 P1】

- 源码：:14-36 AdminToolbar 默认插槽写 21 个入口（:16 搜索、:17 热图/面板、:19 深度读、:20 孪生端点、:21 建热图、:22 改标题、:23 存配置、:24 建用户热图、:25 删用户热图、:26-27 清 BBS/CMS 密文、:28-29 建/删实体热图、:30-35 存在校验/按应用/按ID/密文列表/用户列表等）；:38-56 AdminTable 默认插槽写行删除（:52 btn-del）；columns 含 操作 列。
- 真浏览器 DOM：adminToolbar=1、toolbarBtn=1（仅刷新）；6 个抽样标签（热图面板/建热图/改标题/存配置/用户列表/密文列表）命中数全 0；btnDel=0、btnAct2=0；表头 操作 列 td 为 —。allButtons 与 4.1 相同，无任何热帖业务按钮。
- HTTP：GET /oa4rust-api/api/hotpic/core/entity/list = 200（列表正常）；pageerror=0。
- 判定：21 个热图管理入口 + 行删除全被丢弃，P1 真缺陷（严重度较 4.1 更高，涉及更多写操作入口）。

### 4.3 QueryViewApp.vue（路由 /app/admin/queryview，main.ts:176-178）— 【产品语义差异，非缺陷】

- 源码：:7 AdminToolbar 自闭合（仅搜索框），大量按钮写在组件外的 div.toolbar（:13-55），不存在默认插槽传入。
- 真浏览器 DOM：查询列表=1、执行视图=1、导Excel=1、插入行=2，allButtons 含 40+ 业务按钮；adminTable=0（自造结构）。HTTP GET /oa4rust-api/api/queryview/view/list/query/all = 200。
- 判定：入口真实渲染，不属丢弃问题。

### 4.4 ProcessManagerApp.vue（路由 /app/admin/process-manager，main.ts:308/314）— 【产品语义差异，非缺陷】

- 源码：:7 AdminToolbar 自闭合；:17 EndpointFold 组件，具名 #group-read（:18-38）与 #group-write（:39-）被 EndpointFold 消费，不是 AdminToolbar / AdminTable 的默认插槽。
- 真浏览器 DOM：工具栏仅内置刷新；高级操作按钮位于 EndpointFold 折叠面板内，默认折叠态未在 allButtons 展开出现，属折叠 UI 语义而非丢弃。HTTP GET /oa4rust-api/api/processplatform/assemble/surface/process_manager/list = 200。
- 判定：入口在折叠面板中，源码确认插槽被正确消费，非缺陷。（本轮未逐一展开折叠组点击写端点：属破坏性写取证限制，不影响插槽判定。）

## 5. 真缺陷最小修复方向（仅建议，不改源码）

方案 A（具名插槽改造，推荐）：
- AdminToolbar：将两页工具按钮从默认插槽移入 <template #actions>（新建类可改用 createLabel 由组件内置渲染），搜索输入项移入 <template #filters>。
- AdminTable：将行操作按钮由默认插槽改为 <template #cell-actions="{ row }">，与 columns 的 操作 列对齐。
- 验证：改后真浏览器 DOM 中 AppInfo 6 个入口 + 5 个行操作、Hotpic 21 个入口 + 行删除全部可见；点选只读入口返回 200；pageerror=0；不破坏 R329 已修 DocumentApp.vue。

方案 B（组件兼容默认插槽，不推荐）：在 AdminToolbar / AdminTable 内补 <slot/> 回退。改动面覆盖所有使用方，回归成本高。

## 6. 取证限制声明

- 两候选页的写按钮（新建应用/建热图/删除等）本轮仅做 DOM 存在性甄别，未点击执行破坏性写，避免污染测试服数据；故端点写入行为不在本轮结论内。
- 截图为本地 headless Chromium 1440x900 渲染结果，不代表物理真机外观。

## 7. 结论

- 精确差集 2 个真候选（AppInfoApp、HotpicApp）经真浏览器坐实均为 P1 真缺陷：默认插槽按钮被静默丢弃，管理操作入口整体缺失，0 pageerror 掩盖问题。
- 11 个粗集合页面经源码与 DOM 双重甄别为非缺陷（组件外按钮、自造结构或 EndpointFold 具名插槽）。
- 与 R329 DocumentApp.vue 合并看，该系统管理页群存在系统性插槽契约错配，建议统一按方案 A 治理。
- 本轮 0 新增 P0；2 个 P1 真缺陷待队长另派修复。≥1000 轮持续迭代，非终态。

## 8. 证据

- 截图：test-results/r332-default-slot-audit/{appinfo,hotpic,queryview,process-manager}-list.png（gitignored，非物理真机）。
- 结构化证据：test-results/r332-default-slot-audit/evidence.json（gitignored，含四页 DOM counts/tagHits/buttons/HTML、API 状态、pageerror=0）。
- 脚本：test-results/r332-default-slot.cjs 与 r332-default-slot-run.sh（gitignored，BASE 经 normalizeBase 补 /oa4rust，凭据运行期读取不回显）。
