# R304 基线契约失配甄别审计（20 用例）

## 0. 来源与口径声明
- 审计人：qa-e2e（AgentTeams 成员）；轮次 R304；attempt_id=204d750c-d570-4fee-a8ef-1b601ac0dfab。
- 性质：**只读源码契约甄别**。全程未修改任何源码/测试，未执行 RED→GREEN 修复；仅对失败断言逐条判定【契约过时】或【真实回归】并给出修复方向。
- 证据来源：`pnpm exec vitest run` 的 JSON 报告 + 实现源码逐行读取。本轮为静态契约审计，未进行物理真机截图，不存在冒充真机行为；测试服地址统一以 `<YOUR_TEST_DOMAIN>` 占位，未写入任何真实域名/IP/口令。
- 基准：`git rev-parse HEAD` = `595a79096c35cc96cb623cce18c452f1396d1cd4`，分支 `opt1000/oa4rust-pc-ai-refactor`（禁 main）。
- 失败规模：desktop `cd oa4rust && pnpm exec vitest run` → numFailedTests=19 / numPassed=2046（13 失败文件）；mobile `cd oa4rust/mobile && pnpm exec vitest run` → numFailed=1 / numPassed=429。合计 **20 失败用例**。注：`tabStateA11y.*.mjs` 不在 vitest `include: ['**/*.test.ts']` 内，不纳入本次判定。
- 判定口径：【契约过时】= 实现按 AI 风格/令牌体系正确演进、旧断言写死字面量未同步（同步断言即可）；【真实回归】= 实现行为与已入库契约冲突且无明确演进依据，属退化或迁移遗漏（需 RED→GREEN 或清单治理）。
- 汇总：契约过时 **15** 条；真实回归 **5** 条（用例 3、4、9、13、15）。

## 1. 逐条判定

### 用例 1：R300 B4 业务工作域 AI 风格契约 > 八个业务页面都消费 --ai-* 令牌并保留单一 h1

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/BusinessWork.aiStyle.B4.contract.test.ts:51-52
- ②断言期望：`template(src).match(/<h1\b/g)` 长度为 1（页内保留单一 h1）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/ProcessTaskCenterApp.vue:1-14 根模板已改为 `<div class="mod-view modern-page ai-panel">` + `<AdminPageHeader title="流程任务中心" />` + `<AdminFormSection>`，页内标题为 :11 `<h2 class="ai-gradient-text" data-testid="process-task-center-page-title">流程任务中心</h2>`；页内 h1 数量=0，主 h1 由 AdminLayout shell 承担。
- ④判定+理由：【契约过时】R302 标题去重后管理页页内 h1→h2，同批 EndpointFold.devGate.B302.contract.test.ts:58/60/61 亦断言「页内无 h1 + 有 h2」并通过，证明 h2 化是当前正确形态；旧断言未同步。
- ⑤建议修复：同步断言：将期望改为「页内 h1=0 且存在带 data-testid 的 h2 标题」，或仅断言存在单一页面主标题（h1 或 h2）。无需改实现。

### 用例 2：回收站：彻底删除与清空共用进行中守卫

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/destructiveGuards.test.ts:16-18
- ②断言期望：`source.split('if (busy.value) return').length - 1 === 2`（期望 2 处进行中守卫）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue:159/174/192/210 四处 `if (busy.value...)`，对应 :160/176/194/211 四处 `busy.value = true`、:169/187/205/220 四处 `busy.value = false`；二次确认文案仍在（:213「所有项目将被永久删除」）。
- ④判定+理由：【契约过时】实现从「彻底删除/清空共用 2 处守卫」扩展为 4 个破坏性动作各自独立防重入，语义更强；测试写死计数 2 未同步，无功能退化。
- ⑤建议修复：同步断言：改为 `>= 2` 或精确断言 4 个破坏性动作各自的 busy 守卫存在，并保留二次确认文案断言。无需改实现。

### 用例 3：部署管理页有 aiPageB5 标记与 h2 分区标题，页面级不重复 shell h1

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/EndpointFold.devGate.B302.contract.test.ts:51（同文件 :58/:60/:61 相关断言通过）
- ②断言期望：模板包含 `<h2`，且含 ai-page-b5、页内 h1 数量=0、不含「端点调试」/mono。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/DeploymentApp.vue:5 `<div class="crud-view modern-page ai-panel ai-page-b5">`（ai-page-b5 ✓），但 :8 仍为 `<h1 class="ai-gradient-text" data-testid="deployment-page-title">部署管理</h1>`，全文件无 `<h2`。首条失败即 `expected '<template>...' to contain '<h2'`。
- ④判定+理由：【真实回归】标题去重迁移遗漏：同批 CmsIndexApp.vue:10 / ProcessManagerApp.vue:11 / ProcessTaskCenterApp.vue:11 均已改为 `<h2 class="ai-gradient-text" data-testid=...>`，唯独 DeploymentApp 残留本地 h1，与用例 4 指向同一根因，且无「有意保留 h1」的演进说明。
- ⑤建议修复：RED→GREEN：将 DeploymentApp.vue:8 的 `<h1 ...>` 改为 `<h2 class="ai-gradient-text" data-testid="deployment-page-title">`（结构与同批页面一致），页内不再重复 shell h1；需补 h2 分区标题断言。与用例 4 一次修复即可同时 GREEN。

### 用例 4：it.each(pages)：DeploymentApp.vue 不得保留本地 h1 大标题

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/flowPortalAdminTitleDedup.contract.test.ts:141
- ②断言期望：`expect(src).not.toMatch(/<h1[\s>]/)`（DeploymentApp.vue 无本地 h1）。
- ③当前实现实际（路径+行号）：同用例 3：oa4rust/frontend/apps/desktop/src/views/DeploymentApp.vue:8 存在 `<h1 class="ai-gradient-text" data-testid="deployment-page-title">部署管理</h1>`。
- ④判定+理由：【真实回归】与用例 3 同一根因（DeploymentApp 标题去重迁移遗漏），两条契约互相印证：当前实现确实违反了「页内无 h1」约定。
- ⑤建议修复：RED→GREEN：同用例 3，将本地 h1 改为 h2 + data-testid；一次改动同时消除用例 3、4 的失败。

### 用例 5：Dashboard 统计计数失败不再伪装成真实的 0

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/liveQueryItems.test.ts:126
- ②断言期望：模板包含字符串 `s.value ?? '—'`。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/Dashboard.vue:24 `<div class="stat-value" :class="{ 'stat-placeholder': s.value === null }">{{ s.value ?? null }}</div>`；:17-20 `<div v-if="countError" class="error-banner stats-error">` + 重新加载按钮；:254-256 `statCount` 失败时返回 `null` 而非 0。
- ④判定+理由：【契约过时】实现用 `null` + `stat-placeholder` CSS + 错误横幅表达取数失败，比字符串占位 `'—'` 更稳健（避免数字语义污染）；旧断言写死字面量未同步。
- ⑤建议修复：同步断言：改为断言存在 `stat-placeholder` 绑定与 `countError` 错误态，且失败分支返回 `null`（而非 `'—'` 字面量）。无需改实现。

### 用例 6：Dashboard 的待办取数失败不再折叠成「暂无待办」

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/liveQueryItems.test.ts:137
- ②断言期望：`source.indexOf('暂无待办事项') > source.indexOf('pendingItems.length === 0 && pendingError')`（错误态分支在空态之后，判为缺陷）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/Dashboard.vue:60-63 `v-else-if="pendingItems.length === 0 && pendingError"` 错误态在前；:64-67 `v-else-if="pendingItems.length === 0"` 空态在后，文案为「暂无待办，去流程中心发起」。错误态顺序正确（:60 < :64）。
- ④判定+理由：【契约过时】实现已把错误态置于空态之前（正确行为），仅文案由「暂无待办事项」改为「暂无待办，去流程中心发起」；旧断言以硬编码文案计算位置，条件恒不成立。
- ⑤建议修复：同步断言：直接断言两个 `v-else-if` 分支的源码顺序（错误态 index < 空态 index），并用正则匹配空态文案而非固定字符串。无需改实现。

### 用例 7：会议页套用现代页头/卡片栅格

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/ModernBusinessB.contract.test.ts:47
- ②断言期望：包含精确字符串 `class="meeting-view modern-page"`。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue:5 `<div class="meeting-view modern-page ai-page">`；其余 modern-hero/section/grid/card/empty/chip/stat 断言均通过。
- ④判定+理由：【契约过时】modern-page 仍在，仅新增 ai-page 令牌后缀；精确字串断言未同步。
- ⑤建议修复：同步断言：用 `toContain('meeting-view')` + `toContain('modern-page')` 分项匹配，不再整串精确相等。无需改实现。

### 用例 8：工作流列表改为卡片行

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/ModernBusinessB.contract.test.ts:76
- ②断言期望：包含精确字符串 `class="work-view modern-page"`。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/ProcessWork.vue:5 `<div class="work-view modern-page ai-page ai-panel">`；其余 modern-* 断言均通过。
- ④判定+理由：【契约过时】同用例 7，modern-page 保留、新增 ai-page/ai-panel 后缀，语义一致。
- ⑤建议修复：同步断言：分项匹配 work-view / modern-page。无需改实现。

### 用例 9：ModernUI.vue 提供根容器与默认插槽，样式非 scoped

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/ModernUI.contract.test.ts:59
- ②断言期望：`expect(src).not.toMatch(/<style[^>]*scoped/)`（样式块不得 scoped）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/components/admin/ModernUI.vue:1-13：:4 `<div class="modern-page ai-page-b5 ai-b5-surface">`、:5 `<slot />`、:8 `<style scoped>`、:9-12 `.ai-b5-surface{background:var(--ai-surface-gradient);border-radius:var(--ai-radius-lg,12px)}`。
- ④判定+理由：【真实回归（待队长裁决）】已入库契约明确要求非 scoped（避免局部化全局类），实现改为 scoped 且无「有意 scoped」的演进说明，字面上与契约冲突。但 scoped 下 `.ai-b5-surface` 仅作用于本组件根 div、实际也仅本组件使用，可能是 t1 令牌层的有意接入方式——**需队长裁决意图**：若确认 scoped 为设计选择则判契约过时并同步断言；否则按真实回归处理。本轮依契约文本暂判真实回归。
- ⑤建议修复：待裁决后二选一：①同步断言（允许 scoped，仅断言根容器/默认插槽/令牌消费）；②RED→GREEN 去掉 `<style scoped>` 的 scoped 属性。不要未经确认直接改实现。

### 用例 10：AdminLayout 通过 injection key 提供并消费标题隐藏开关，默认不隐藏

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/ModernUI.contract.test.ts:74
- ②断言期望：`toMatch(/<h1[^>]*class="admin-title"/)`（h1 class 精确等于 admin-title）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue:38 `<h1 v-if="!hideTitle" class="admin-title ai-gradient-text">{{ pageTitle }}</h1>`；:58/:76-81 injection key `ADMIN_LAYOUT_HIDE_TITLE` 与 `ADMIN_HIDE_PAGE_TITLE` 提供/消费完整。
- ④判定+理由：【契约过时】h1 仍存在、class 以 admin-title 开头，仅追加 ai-gradient-text 渐变令牌；正则要求 class 属性立即闭合才失败，语义等价。
- ⑤建议修复：同步断言：正则改为 `/<h1[^>]*class="[^"]*admin-title/`。无需改实现。

### 用例 11：AppShell 用户下拉具备玻璃表面

- ①测试文件+用例名：oa4rust/frontend/packages/ui/src/components/popupSurfaces.contract.test.ts:60-62
- ②断言期望：包含旧令牌 `var(--bg-glass)` / `var(--glass-blur)` / `var(--glass-border)`。
- ③当前实现实际（路径+行号）：oa4rust/frontend/packages/ui/src/components/AppShell.vue:862-883 `.user-dropdown`：:876 注释「R294：玻璃表面…」、:877 `background: var(--ai-glass-bg)`、:878/:879 `backdrop-filter/-webkit: var(--ai-glass-blur)`、:880 `border: 1px solid var(--ai-glass-border)`、:882 `box-shadow: var(--ai-shadow-2)`。
- ④判定+理由：【契约过时】玻璃表面（背景+模糊+描边+阴影）语义完整保留，仅令牌迁移到 `--ai-glass-*` 新体系（t1 设计令牌层），旧断言未同步。
- ⑤建议修复：同步断言：改断言 `--ai-glass-bg` / `--ai-glass-blur` / `--ai-glass-border`，或统一断言「存在玻璃令牌三件套（背景/模糊/描边）」以兼容重命名。无需改实现。

### 用例 12：ProcessTaskCenterApp 列表标题走 displayTitle 处理原始工作 id

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/ProcessFold.contract.test.ts:44
- ②断言期望：`toMatch(/\^work-/)`（源码字面出现 /^work-/ 正则）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/views/ProcessTaskCenterApp.vue:712-724 `function displayTitle(item: Item): string`：:715 `/^start:/i`、:716-717 正则替换、:724 `raw.startsWith('work-') ? '工作实例' : /^start:/i.test(raw) ? '运行闭环' : '流程任务'`；:65 `<td data-testid="ptc-title-cell"><span class="task-title">{{ displayTitle(item) }}</span>`。
- ④判定+理由：【契约过时】原始 `work-` id 已被 displayTitle 处理（用户可见标题为「工作实例」），仅实现用 `.startsWith('work-')` 而非字面正则 `/^work-/`，断言写法过期。
- ⑤建议修复：同步断言：断言包含 `startsWith('work-')` 或正则 `/work-[^\s]*/` 任一形态，或直接断言 displayTitle 内含「工作实例」映射。无需改实现。

### 用例 13：covers every exact crud-view and rejects unadjudicated additions

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/views/w13-w14-closure.test.ts:41-44,72-77
- ②断言期望：manifest adjudicated 集合（23 条 W13/crud_view）与实际含 `class="crud-view"` 的 .vue 列表 toEqual；实际 29 条。
- ③当前实现实际（路径+行号）：测试 :41-44 exactCrudViews() 扫描 viewsDir 所有含 `class="crud-view"` 的 .vue；实际比 manifest 多 6 个（如 CmsIndexApp.vue:8 / ProcessManagerApp.vue:9 / ProcessTaskCenterApp.vue:10 等 R300/R302 AI 重构产物），错误信息 `expected ['CmsColumnApp.vue', …(28)] to deeply equal ['CmsColumnApp.vue', …(23)]`。
- ④判定+理由：【真实回归（清单治理漂移）】非 UI 功能 bug，而是 closure manifest 未随新增 crud-view 页面同步裁决；契约「拒绝未裁决新增」的守卫本身工作正常，是清单数据过期。
- ⑤建议修复：清单治理（建议由 manifest owner 执行）：核对 6 个新增页面是否应纳入 W13/crud_view，向 `docs/audits/w13-w14-repository-closure.manifest.json` 补条目（含 id/workItem=W13/kind=crud_view/view/status/replacementClaim/evidence），或显式标记豁免；补齐后 GREEN。本轮只审计不改清单。

### 用例 14：顶部面包屑与页标题来自当前路由（meta.title 同源）

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.contract.test.ts:34
- ②断言期望：`toMatch(/<h1[^>]*class="admin-title"/)`。
- ③当前实现实际（路径+行号）：同用例 10：oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue:38 `<h1 v-if="!hideTitle" class="admin-title ai-gradient-text">`；:33 系统管理；route.meta.title 相关断言（:33）未失败，说明同源逻辑成立。
- ④判定+理由：【契约过时】仅 class 追加 ai-gradient-text 导致精确正则失配，标题来源逻辑正常。
- ⑤建议修复：同步断言：正则放宽为 class 包含 admin-title。无需改实现。

### 用例 15：四个原语都消费设计令牌且不重复定义 glass-card / 裸 hex 颜色

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/adminPrimitives.contract.test.ts:62
- ②断言期望：`not.toMatch(/#[0-9a-fA-F]{3,8}\b/)`（无裸 hex 颜色）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/components/admin/AdminToolbar.vue:108-113 `.admin-toolbar-btn.primary { background: var(--ai-gradient-brand); color: #fff; border-color: var(--color-primary); color: var(--bg-base); }`：:110 `color: #fff` 命中；但 :112 又写 `color: var(--bg-base)` 覆盖 :110，#fff 为死代码残留。
- ④判定+理由：【真实回归（低危）】既有契约明确禁裸 hex，且 :110 被 :112 覆盖属冗余死代码；无视觉影响但确属契约违反（非断言过期）。
- ⑤建议修复：RED→GREEN：删除 AdminToolbar.vue:110 的 `color: #fff;` 一行，保留 :112 令牌写法即可 GREEN，视觉无变化。

### 用例 16：adminTitleDedup：AdminLayout 默认提供隐藏页标题开关

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/adminTitleDedup.contract.test.ts:39
- ②断言期望：`toMatch(/<h1[^>]*class="admin-title"/)`。
- ③当前实现实际（路径+行号）：同用例 10/14：oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue:38 h1 class 为 `admin-title ai-gradient-text`；:58/:78 provide(ADMIN_LAYOUT_HIDE_TITLE, hideTitle)、:81 provide(ADMIN_HIDE_PAGE_TITLE, ref(true)) 完整。
- ④判定+理由：【契约过时】class 追加渐变令牌导致正则失配，隐藏开关机制正常。
- ⑤建议修复：同步断言：正则放宽为 class 包含 admin-title。无需改实现。

### 用例 17：adminTitleDedup：AdminPageHeader 只隐藏标题/副标题，保留 actions

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/adminTitleDedup.contract.test.ts:47
- ②断言期望：包含 `class="admin-page-title"`。
- ③当前实现实际（路径+行号）：oa4rust/frontend/apps/desktop/src/components/admin/AdminPageHeader.vue:7 `<div v-if="!hidePageTitle" class="admin-page-heading">`；:8 `<h2 class="admin-page-title ai-gradient-text">`；:9 副标题、:11 `<div class="admin-page-actions">`、:19/:27 inject ADMIN_HIDE_PAGE_TITLE。
- ④判定+理由：【契约过时】class 以 admin-page-title 开头、追加 ai-gradient-text，精确字串失配；隐藏标题/副标题而保留 actions 的语义完整。
- ⑤建议修复：同步断言：改用 `toContain('admin-page-title')`。无需改实现。

### 用例 18：adminTitleDedup：管理区路径下 topbar 隐藏当前标题面包屑

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/adminTitleDedup.contract.test.ts:73
- ②断言期望：`toMatch(/<div[^>]*v-if="!isAdminArea"[^>]*class="breadcrumb"/)`（div + v-if + class 顺序）。
- ③当前实现实际（路径+行号）：oa4rust/frontend/packages/ui/src/components/AppShell.vue:72 `<nav v-if="!isAdminArea" class="breadcrumb">`；:78 `<span class="breadcrumb-current" :aria-current="isAdminArea ? undefined : 'page'">`；:294 `isAdminArea = route.path.startsWith('/app/admin')`。
- ④判定+理由：【契约过时】面包屑元素由 `<div>` 升级为 `<nav>` landmark（无障碍语义更强），v-if="!isAdminArea" 与 breadcrumb class 均保留；断言写死标签名与属性顺序。
- ⑤建议修复：同步断言：改匹配 `<(div|nav)[^>]*v-if="!isAdminArea"[^>]*class="breadcrumb"` 或按语义断言 isAdminArea 条件存在。无需改实现。

### 用例 19：adminTitleDedup：AdminLayout 面包屑只到分组，不重复末级页名

- ①测试文件+用例名：oa4rust/frontend/apps/desktop/src/components/admin/adminTitleDedup.contract.test.ts:81
- ②断言期望：`toMatch(/<h1[^>]*class="admin-title"/)`（同 :39 的 h1 class 精确正则）。
- ③当前实现实际（路径+行号）：同用例 10/14/16：oa4rust/frontend/apps/desktop/src/components/admin/AdminLayout.vue:38 h1 class 为 `admin-title ai-gradient-text`；面包屑 :32-37 仅含 admin-breadcrumb / currentGroupLabel，已无 `admin-breadcrumb-current` 模板类（:79 断言应通过）。
- ④判定+理由：【契约过时】class 追加渐变令牌导致正则失配；面包屑只到分组的实现已符合约定。
- ⑤建议修复：同步断言：正则放宽为 class 包含 admin-title。无需改实现。

### 用例 20：R294 空状态与交互反馈 > 列表空状态统一为 o2-empty（图标 + 主文案）

- ①测试文件+用例名：oa4rust/mobile/src/design-system.contract.test.ts:87（mobile）
- ②断言期望：`expect(page).toContain('class="o2-empty"')`（5 页均精确包含该字串）。
- ③当前实现实际（路径+行号）：oa4rust/mobile/src/pages/message/message.vue:91-93、bbs/bbs.vue:94-96、meeting/meeting.vue:171-173、contacts/contacts.vue:222-224、process/process.vue:271-273 均有 `<view ... class="o2-empty ...">` + `<view class="o2-empty-icon" aria-hidden="true">` + `.o2-empty-text`；部分页 class 带修饰类（如 message.vue:91 `class="o2-empty empty-guide"`），精确字串不匹配。
- ④判定+理由：【契约过时】o2-empty 图标+主文案五页齐全，仅部分 class 追加修饰类；旧断言精确字串匹配过期，空态统一语义已落地。
- ⑤建议修复：同步断言：改用 `toMatch(/class="[^"]*o2-empty(?:\s|")/)` 或 class 列表包含判断。无需改实现。

## 2. 收敛建议（按优先级）

| 优先级 | 用例 | 动作 | 说明 |
|---|---|---|---|
| P1 | 3、4 | RED→GREEN 修实现 | DeploymentApp.vue:8 h1→h2 + data-testid，一次改动同时消除 2 条失败，与同批页面标题去重对齐 |
| P2 | 15 | RED→GREEN 删死代码 | AdminToolbar.vue:110 删除被覆盖的 `color: #fff`，无视觉影响 |
| P2 | 9 | 队长裁决后二选一 | ModernUI.vue:8 `<style scoped>`：确认有意则同步断言，否则去 scoped |
| P2 | 13 | 清单 owner 治理 | manifest 补 6 个 crud-view 条目或显式豁免，非 UI 功能 bug |
| P3 | 1、2、5、6、7、8、10、11、12、14、16、17、18、19、20 | 同步断言（15 条） | 均为实现按 AI 风格/令牌体系正确演进、旧断言写死字面量未同步，无需改实现 |

## 3. 取证限制与诚实声明
- 本轮为**静态契约甄别**，未改任何源码/测试、未重新部署测试服、未做 RED→GREEN；所有「修复方向」仅为建议，实际修复需在后续轮次按 test-first 流程执行并真浏览器复验。
- vitest 失败规模以本轮 JSON 报告为准：desktop 19 / mobile 1；JSON 报告与失败明细位于 gitignored 的 `oa4rust/test-results/r304-*.json`、`oa4rust/test-results/r304-failures.txt`、`oa4rust/mobile/test-results/r304-mobile.json`，不入库。
- 用例 9（scoped）含设计意图不确定性，已明确标注为「待队长裁决」，未按猜测定论——若裁决与本文不一致，以裁决为准并更新本文。
- 不涉及真实域名、IP、口令、私钥或环境变量值；测试服地址以 `<YOUR_TEST_DOMAIN>` 占位。

