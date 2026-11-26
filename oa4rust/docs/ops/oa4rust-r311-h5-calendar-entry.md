# R311 H5 建日历入口与同源端点修复

> 任务：t71 [R311 H5 建日历入口]（分支 opt1000/oa4rust-pc-ai-refactor，≥1000 轮持续迭代中的一环，非终态）。

## 目标与根因
- t69/t70 R310 审计：H5 日历页在干净数据下 calendars=[]，原仅有“+ 新建”事件表单，仍需先选 calendarId 才能保存事件，用户在 H5 无法自建日历。
- 补 calendarApi.create + 可见“+新建日历”入口 + 最小建日历表单；创建成功后列表 +1 并自动选中新日历，保存事件可见于今日/本周。
- 真浏览器首轮暴露**双表漂移真 bug**：core entity 的 POST /api/calendar/core/entity/calendar/create 经 sea_orm ORM 写 x_cal_calendar（entities/cal_calendar.rs:7），而 list/my 用原生 SQL 读 cal_calendar（calendar_assemble_control/src/u2.rs:170），两表不同 → 建完列表读不到。改用与 list/my **同源**的 POST /api/calendar_assemble_control/calendar（routes.rs:29-32 → u2.rs:335，原生 SQL INSERT INTO cal_calendar）。未改任何后端源码。

## RED → GREEN
- RED：calendarCreate.R311.contract.test.ts 首次契约 4 failed/1 passed（入口/表单/创建后逻辑缺失）；同源端点修订后再次 RED 2 failed/12 passed（端点与 surface 测试仍指 core entity）。
- GREEN：services/index.ts calendarApi.create 指向同源端点；calendar.vue 新增建日历入口与表单；surface 测试更新。定点 14/14 通过。

## 验证
- mobile 全量 vitest：71 files / 458 tests passed。
- workspace typecheck：6/7 projects Done（含 mobile），exit 0。
- H5 build：Build complete；产物 index-BUQSTyeU.js / index-CdSwBuzB.css。
- 测试服原子部署：备份→上传→校验新哈希与同源端点字符串→切换；5 路由冒烟可访问。

## 真浏览器实测（非物理真机）
- Playwright Chromium headless 390x844 isMobile/hasTouch DPR2，真实 UI 登录→日历页→+新建日历→填名提交→+新建事件→填标题/起止→保存。
- RESULT：loggedIn=true，日历数 3→4，create POST 200 且返回新 id，事件日历下拉自动选中新日历，保存后今日列表出现 R311-E2E-事件，pageErrors=0，apiFailures=0，passed=true。
- 证据：gitignored 目录 test-results/r311-h5-calendar-entry/（evidence.json + 6 PNG + manifest.md）。截图为本地无头浏览器渲染真实测试服页面，**非物理真机、非静态/假数据拼图**。

## 脱敏与边界
- 测试域名/主机用 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST> 占位；凭据仅运行时从本机凭据读取，不回显、不入库。
- 不改后端；双表漂移作为发现记录，后续如有独立任务统一 core entity 与 assemble_control 数据源。
- 本项目 ≥1000 轮持续迭代，本轮只是一环，不声称完成。
