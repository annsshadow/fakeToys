# R337 PC 收集管理 CollectApp + 笔记 NoteApp 写闭环只读深度审计

## 1. 范围与取样

- 对象：`oa4rust/frontend/apps/desktop/src/views/CollectApp.vue`（239 行，路由 `/app/admin/collect`，main.ts:439-444）、`oa4rust/frontend/apps/desktop/src/views/NoteApp.vue`（217 行，路由 `/app/note`，main.ts:114）。
- 方式：测试服 1440x900 Playwright Chromium headless 真浏览器（**非物理真机**），走既有 SSH 部署/取证流程；登录用 gitignored 凭据脚本运行期读取，不回显。
- 证据：`oa4rust/test-results/r337-collect-note/{evidence.json,evidence-v2.json,C1-list.png,C2-created.png,C3-edited.png,C4-confirm.png,C7-deleted.png,C6-fail500.png,N1-list.png,N2-edited.png}`（gitignored，不入仓；截图均为 1440x900 headless）。
- 性质：只读审计，未改源码/契约/后端/配置。

## 2. 既有覆盖甄别（先做，避免重复）

| 端点 | 源码 | 既有文档覆盖 | 本轮动作 |
|---|---|---|---|
| Collect 列表 GET collect/list | CollectApp.vue:100,120 | 无任何 R296/R300/R309/R310/R311/R312 文档命中 `program_center/collect` | 本轮真浏览器加载 200 |
| Collect 新建 POST collect/create | CollectApp.vue:101,168 | 只在 R312 write-entry-gap-audit 作为源码级写端点枚举（docs/ops 全库 grep `program_center/collect` = 0 命中，R312 独立文档当前版本不在库），从未真 UI | 本轮新建→回显 |
| Collect 编辑 POST collect/save/{id} | CollectApp.vue:167 | 同上，从未真 UI | 本轮编辑→回显更新 |
| Collect 删除 POST collect/delete/{id} | CollectApp.vue:184,192 | 同上，从未真 UI | 本轮确认守卫+删除→刷新消失 |
| Note 列表 GET note/list | NoteApp.vue:82,101 | R300 首屏五页 READ 覆盖；R321 新建流程 GET 200 | 回归观察 |
| Note 新建 POST note/create | NoteApp.vue:83,144 | R321 pc-deep-audit:18 真 UI PASS（GET 200 + POST 200 + 回显 rowCount=3）；R317 静态枚举 | 不重复 |
| Note 编辑 PUT note/save/{id} | NoteApp.vue:143 | 无任何文档真 UI 覆盖 | 本轮补测 |
| Note 删除 POST note/delete/{id} | NoteApp.vue:160,167 | R310 删除确认守卫覆盖（历史闭环，见 R321:46 注明）；R317 静态枚举 | 不重复真删 |

结论：CollectApp 三写端点此前 0 真浏览器覆盖；NoteApp 仅编辑 PUT 未覆盖，其余已被 R300/R310/R321/R317 覆盖。

## 3. 真浏览器观察

### 3.1 CollectApp 新建 POST /api/program_center/collect/create — 【通过】

- DOM：工具栏含「刷新 / + 新建」（collectButtons 中仅此二业务按钮）；点击「+ 新建」弹出 `.modal-overlay .modal` 1 个，含 personId/标题/链接三输入与「保存」按钮。
- 操作：填标题 `R337T…‑收集`、链接 `https://example.invalid/r337`（personId 由 openCreate 默认 admin unique，CollectApp.vue:146）+ 点保存。
- HTTP：`POST /oa4rust-api/api/program_center/collect/create → 200`；刷新后 `GET collect/list → 200`。
- DOM 证据：列表 0→1 行，新建标题出现（evidence.json `collectCreatedInList=true`），截图 C2-created.png 肉眼可见该行「标题=R337T95787387-收集 / 链接=https://example.invalid/r337 / 成员=admin」；保存后弹窗关闭（`collectCreateModalClosed=0`）。

### 3.2 CollectApp 编辑 POST /api/program_center/collect/save/{id} — 【通过】

- 源码：CollectApp.vue:167 按 `editingId` 派发 `api.post(/collect/save/${id}, payload)`；editItem(:150-154) 回填标题/链接/personId。
- 操作：定位新建行点「编辑」，表单标题回填 `R337T95787387-收集`（`collectEditFormTitle`），改标题为 `…‑收集-改` 后保存。
- HTTP：`POST /oa4rust-api/api/program_center/collect/save/22b926d5-… → 200`；刷新 GET 200。
- DOM 证据：`collectEditedInList=true`，截图 C3-edited.png 可见新标题；后端注册同时支持 PUT/POST（program_center/src/routes.rs:335-336），前端用 POST 合法。

### 3.3 CollectApp 删除确认 + POST /api/program_center/collect/delete/{id} — 【通过】

- 源码：CollectApp.vue:192 `confirmMsg("确定删除该收集记录？")`，:184 `api.post(/collect/delete/${id})`。
- DOM：点「删除」弹出 `#oa4-confirm-overlay`，文案「⚠ 确认操作 确定删除该收集记录？ 取消 确认」（v2Confirm）；点「取消」记录仍在（`collectCancelKept=true`）→ 守卫生效。
- 再次点删除→「确认」：`POST /oa4rust-api/api/program_center/collect/delete/48b16f95-… → 200`；刷新 GET 200。
- DOM 证据：`v2DeletedGone=true`、列表行数回到 0（v2RowsAfterDelete=0），截图 C7-deleted.png 该行消失。

### 3.4 CollectApp 加载失败 500 兜底 — 【通过，与空态可区分】

- 方法：`page.route` 将 `**/api/program_center/collect/list` fulfill 500 后刷新。
- HTTP：GET collect/list 500（3 次观察）。
- DOM：`.list-error-empty` 1 个，文案「收集管理加载失败：HTTP 500: Internal Server Error」（`v2Fail500Text`），与普通空态「📐暂无数据」可区分；截图 C6-fail500.png。无 pageerror。

### 3.5 NoteApp 编辑 PUT /api/cms/core/entity/note/save/{id} — 【通过（补覆盖）】

- 源码：NoteApp.vue:143 `api.put(/note/save/${editingId}, payload)`；后端 cms_core_entity/src/lib.rs:963/967 该路径同时注册 PUT/POST。
- 操作：列表首行（旧标题「R321审计笔记08566」）点编辑，表单标题回填一致（`v2NoteFormTitle`），改标题为 `R337V…‑笔记改` 后保存。
- HTTP：`PUT /oa4rust-api/api/cms/core/entity/note/save/8af4a476-… → 200`；刷新 `GET note/list → 200`。
- DOM 证据：`v2NoteEditedInList=true`，截图 N2-edited.png 肉眼可见首行标题更新且其余两行不变（noteRows=3）。0 pageerror。

## 4. 三分判定汇总

| # | 观察 | 判定 | 依据 |
|---|---|---|---|
| 1 | Collect 新建 POST create → 列表回显 | 通过（非缺陷） | HTTP 200 + DOM 0→1 行 + 截图 C2 |
| 2 | Collect 编辑 POST save/{id} → 回显更新 | 通过（非缺陷） | HTTP 200 + 表单回填 + 标题更新截图 C3 |
| 3 | Collect 删除确认守卫（取消保留） | 通过（非缺陷） | confirm overlay 文案 + 取消后仍在 |
| 4 | Collect 删除 POST delete/{id} → 刷新消失 | 通过（非缺陷） | HTTP 200 + 行数归零截图 C7 |
| 5 | Collect 500 兜底 | 通过（非缺陷） | 500 + ListLoadError 文案可区分空态 |
| 6 | Note 编辑 PUT save/{id} → 回显更新 | 通过（非缺陷） | HTTP 200 + 标题更新截图 N2 |
| 7 | Note 新建/删除（R321/R310 已覆盖） | 产品语义差异：不重复真写 | 历史文档已 PASS；本轮仅首行编辑用其遗留数据 |
| 8 | 破坏性写（清空/批量，本轮未授权） | 取证限制：不伪造 | 本任务仅单条已授权写；未触碰批量/清空 |

本轮 **0 新增真缺陷**；CollectApp 三写端点与 NoteApp 编辑端点均真实可用。

## 5. 结论

- CollectApp 写闭环（新建/编辑/删除+确认守卫/500 兜底）首次真浏览器坐实，全部 PASS。
- NoteApp 未覆盖的编辑 PUT 端点补测 PASS；已覆盖项不重复。
- 首轮脚本在删除后 `.admin-table` 因空态（0 行）不存在而 innerText 超时，属测试脚本鲁棒性问题，非业务缺陷；v2 用安全取值重测坐实删除生效。
- 页面 `pageerror=0`；截图/evidence 均标注「local headless Chromium against deployed test server; not a physical device」。
