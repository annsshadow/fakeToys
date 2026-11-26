# R310 回收站与日程闭环只读审计

> 来源与边界：本文由 QA E2E 在真实测试服以 Playwright Chromium 无头浏览器逐页操作取证，**属本地渲染截图，非物理真机**。测试域名/主机以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，口令仅在运行时经 SSH 从服务器凭据文件读取、不入库、不回显。审计范围为只读：未修改任何源码、契约测试、后端或生产数据（新建/删除仅通过测试服真实 UI 产生的测试记录）。

## 1. 目标与结论摘要

针对 R309 深度功能审计遗留的两项待判问题做只读定性，并补 H5 跨端抽检：

- **D2 笔记删除不进回收站**：真实行为成立，但**产品口径非缺陷**。回收站后端实现是文件专用（仅查 `FILE_FILE` 表），笔记软删除写入 `x_cms_note`，二者数据源隔离；文件删除可进回收站，与笔记形成决定性对照。
- **日程新建取证受限**：测试服 `my_calendars=[]` 且 PC/H5 均无“新建日历”的 UI 入口，无法构造日程创建所需前置日历；属取证限制，**不判定为缺陷**。
- **H5 跨端抽检通过**：PC 与 H5 回收站均只显示文件、均无日程数据，口径一致。
全程 0 pageerror、0 API 失败。

## 2. D2 笔记回收：源码定性（文件专用回收站）

| 环节 | 源码位置 | 事实 |
|---|---|---|
| 回收站列表 | `oa4rust/backend/crates/file_assemble_control/src/lib.rs:3996-4062` `recycle_list()` | SQL 仅查 `FILE_FILE`：`SELECT id,name,person,reference_type,extension,length,mime_type,deleted_at::text AS deleted_at FROM FILE_FILE WHERE deleted_at IS NOT NULL AND person=$1 ORDER BY deleted_at DESC` |
| 单条读 | 同文件 `:4066-4099` | 同样只查 `FILE_FILE` |
| 恢复 | 同文件 `:4143-4174` | `SELECT person FROM FILE_FILE WHERE id=$1 AND deleted_at IS NOT NULL` + `UPDATE FILE_FILE SET deleted_at=NULL,update_time=NOW() WHERE id=$1` |
| 回收站路由 | `file_assemble_control/src/routes.rs:248-253` | `/api/recycle/list` / `/api/recycle/resume/{id}` 等均在此 crate 注册 |
| 笔记删除 | `oa4rust/backend/crates/cms_core_entity/src/lib.rs:836-849` `cms_entity_note_delete` | 调 `cms_entity_soft_delete(&pool,"x_cms_note",&id)` |
| 软删除实现 | 同文件 `:472-489` | `UPDATE {table} SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL` |
| 前端回收站 | `oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue:138` / `:150` | 列表 `GET /api/recycle/list`、恢复 `POST /api/recycle/resume/${id}` |

数据源互不相交：回收站仅聚合被软删的**文件**（`FILE_FILE`），笔记软删写在 `x_cms_note`，回收站本身不覆盖笔记。这是产品边界，不是 bug。

## 3. D2 真 UI 实测证据链

脚本 `oa4rust/test-results/r310-pc.cjs`（证据 `r310-recycle-calendar/pc-evidence.json`）：

1. 笔记页新建 `R310回收笔记_...` → 列表回显 1 条；
2. 走真实自定义确认弹窗（`#oa4-confirm-overlay` 内“确认”）删除：`POST /api/cms/core/entity/note/delete/{id}` **HTTP 200** `{deleted:true}`，笔记列表残留 0；
3. 立即访问回收站：`GET /api/recycle/list` **HTTP 200**，仅显示文件 `R309末次文件_...`，无该笔记（`noteAbsent`）。截图 `D2-note-recycle.png`（1440×900）目视确认：回收概览 1 项、列表中仅 `R309末次文件_...`。

**决定性对照**（脚本 `r310-final-contrast.cjs`，`pc-final-contrast.json`，截图 `FINAL-recycle-contrast.png`）：同一时刻 `GET /api/recycle/list` 返回 2 项，均为被删文件（`R310恢复文件_...`、`R309末次文件_...`），不含被删笔记；返回字段 `deletedAt/extension/id/length/mimeType/name/person/referenceType` 全是文件属性。`filePresent=true` / `noteAbsent=true`，坐实回收站文件专用。

文件删除本身是真闭环：`r310-file-fix.cjs` 抓取 `POST /api/file/core/entity/file` 200 建文件、`DELETE /api/file/assemble/control/file/{id}` **200** `{deleted:true}`，随后回收站出现该文件。此前脚本 `inRecycle=0` 是 DOM 选择器/刷新时序问题，非业务缺陷。

## 4. 日程新建补验：取证限制（非缺陷）

- PC：`GET /api/calendar/calendar/list/my` 200 `{my_calendars:[],unit_calendars:[],follow_calendars:[]}`；页面“我的日历（）/公共日历（0）”、chip=0。源码 `CalendarApp.vue:251` `calWriteCE()` 含 `calCreate`（`:253-256`，`prompt` + `POST calendar/core/entity/calendar/create`），但全源码无调用点，UI 没有新建日历入口；`:398-419` `createEvent` 需 `activeCal`。
- H5：`oa4rust/mobile/src/pages/calendar/calendar.vue:263` “+新建”仅 `toggleCompose()` 展开事件表单，仍需 `calendarId`，而 `calendars=[]`。
- 命令面板检索“日历”：面板可打开但无建日历命令（脚本 `r310-supp.cjs` `S2` pass）。
→ 前置日历缺失，日程新建无法在真实 UI 中构造，**如实记为取证限制，不判缺陷**。后端 `calendar_create`（`calendar_core_entity/src/lib.rs:353-411`）需 name+type，若产品需开放入口可作为后续需求评估。

## 5. H5 跨端抽检

脚本 `r310-h5.cjs`（`h5-evidence.json`，视口 390×844、`isMobile`/`hasTouch`，截图 `h5-recycle.png` / `h5-calendar.png`）：

- `/pages/recycle/recycle`：bodyLen≈64，仅显示 R309 末次文件与“彻底删除/清空回收站”，`GET /api/recycle/list` 200 仅文件；
- `/pages/calendar/calendar`：bodyLen≈29，“我的日历 0 个 / +新建 / 今日暂无日程”，`GET /api/calendar/calendar/list/my` 200 `{my_calendars:[],unit_calendars:[]}`；
- 与 PC 表现一致，0 pageerror / 0 API 失败。

## 6. 局限性

- 测试服组织/日历前置数据为空，日程创建链路未能走通；
- 会议创建 HTTP 500 属 R309 已立项缺陷，本文不重复；
- 所有结论基于单一时点、真实 UI + 接口信封取证；截图为本地无头浏览器渲染，非物理真机。

## 7. 结论与建议

R309 两项遗留均已定性：D2 笔记回收为**产品口径非缺陷**（回收站文件专用，证据链完整）；日程新建为**取证限制**（缺前置日历与 UI 入口，非缺陷）。H5 跨端一致。无需为该两项开 bug；若期望笔记也能恢复，应作为“统一回收站”产品需求另行评估。

## 8. 证据索引（本地 gitignored）

- `oa4rust/test-results/r310-recycle-calendar/pc-evidence.json`、`h5-evidence.json`、`pc-supplement.json`、`pc-file-resume.json`、`pc-final-contrast.json`
- 截图：`D2-note-recycle.png`、`h5-recycle.png`、`h5-calendar.png`、`FINAL-recycle-contrast.png`、`FIX-file-recycle.png`、`FIX-file-resumed.png`（均为本地无头浏览器渲染，非物理真机）
- 脚本：`oa4rust/test-results/r310-{pc,h5,supp,file-fix,final-contrast}.cjs`
