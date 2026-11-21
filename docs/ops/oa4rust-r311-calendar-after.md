# R311 建日历入口改造后真浏览器独立复查

> 来源与边界：本文由 QA E2E 在真实测试服以 Playwright Chromium 无头浏览器逐页操作取证，**属本地渲染截图，非物理真机**。测试域名/主机以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，口令仅在运行时经 SSH 从服务器凭据文件读取、不入库、不回显。本轮为只读复查：未修改任何前端/移动端/后端源码、契约测试；测试记录仅经真实 UI 在测试服产生。

## 1. 背景与依赖时序

- 立项来源：R310 审计（docs/ops/oa4rust-r310-recycle-calendar-audit.md）确认「日程新建取证受限」——测试服 my_calendars=[] 且 PC/H5 均无「新建日历」UI 入口，无法构造日程前置日历，属取证限制而非缺陷。
- 改造落地：t70（PC，commit b10f73e5）在 CalendarApp.vue 增加可见入口 data-testid=calendar-create-entry 与表单 data-testid=calendar-create-form，替换 prompt 调试入口，并修 my_calendars 信封解析与跨轨 404；t71（H5，commit 9d679c19）将 calendarApi.create 指向同源端点 POST /api/calendar_assemble_control/calendar，新增 .calendar-create-btn / .calendar-create-form / .c-submit。
- 时序：等到 t70/t71 完成并原子部署测试服后才执行本复查。复查时测试服入口 PC=index-D1EjQmHJ.js、H5=index-BUQSTyeU.js，服务 active；本地 HEAD=2bd3e876892ea1e1a0404ae13fb477326119ec37，分支 opt1000/oa4rust-pc-ai-refactor。

## 2. 复查口径

- PC：1440×900 真登录 → /app/calendar → 点「+ 新建日历」→ 提交创建 → 校验 chip 数 +1 → 在该日历建事件 → 回显 → 刷新校验持久；同时抽检 /app/org 组织页 loading 兜底。
- H5：390×844、isMobile/hasTouch 真登录 → /pages/calendar/calendar → 点「+新建日历」→ 创建 → 校验列表数 +1 且自动选中 → 保存事件 → 今日列表可见 → 刷新校验持久；同时记录信封归一化与渐变品牌。
- 全程采集 pageerror（errors）与 API 失败（apiFailures，含 400/404/≥500）。

## 3. PC 闭环结果（R311_AFTER_PC_DONE，pass=true）

| 步骤 | 实测值 | 判定 |
| --- | --- | --- |
| 真实登录 | loggedIn=true | 通过 |
| 入口/表单可见 | entryVisible=true / formVisible=true | 通过 |
| 日历 chip 数 | chipsBefore=4 → chipsAfter=5，新日历命中 1 | 通过（+1） |
| 新建日历名 | R311复查日历_<TS> | 通过 |
| 日程管理面板 | managerVisible=true，标题「R311复查日历_<TS> · 日程管理」，managerOnNewCal=true | 通过 |
| 事件创建/回显 | eventVisible=true，R311复查日程_<TS> | 通过 |
| 刷新持久 | chipAfterReload=1 / eventAfterReload=true | 通过 |
| pageerror / API 失败 | errors=[] / apiFailures=0（apiCalls=24） | 通过 |

API 链（均 200）：calendar/calendar/list/my → calendar/calendar/create → list/my → calendar/calendar/<id> → calendar/event/create → calendar/event/list/<id>；组织页 organization/assemble/control/group/list/like 亦 200。截图：01-calendar-before、02-calendar-form、03-calendar-created、04-event-form、05-event-created、06-after-reload、07-org-regression（共 7 张）。

## 4. H5 闭环结果（R311_AFTER_H5_DONE，pass=true）

| 步骤 | 实测值 | 判定 |
| --- | --- | --- |
| 真实登录 | loggedIn=true | 通过 |
| 信封归一化（t25） | envelopeOk=true（calendar_assemble_control/calendar/list/my 正常解析） | 通过 |
| 渐变品牌（t51） | branded=4（>0） | 通过 |
| 入口/表单可见 | createBtnVisible=true / calFormVisible=true | 通过 |
| 日历数 | calsBefore=6 → calsAfter=7，刷新后 calsAfterReload=7 | 通过（+1 且持久） |
| 新建后自动选中 | pickerAutoSelectNew=true | 通过 |
| 事件创建/回显 | eventVisible=true，R311H5复查日程_<TS>，今日 12:36-13:36 | 通过 |
| 刷新持久 | eventAfterReload=true | 通过 |
| pageerror / API 失败 | errors=[] / apiFailures=0（apiCalls=40） | 通过 |

截图：01-calendar-before、02-calendar-form、03-calendar-created、04-event-form、05-event-created、06-after-reload（共 6 张），均为 390×844 视口的本地无头浏览器渲染、非物理真机。

## 5. 回归抽检

| 历史项 | 复查证据 | 判定 |
| --- | --- | --- |
| t24 PC 日历 TDZ | PC errors=[]，日历页正常渲染，无「Cannot access before initialization」 | 无回归 |
| t28 组织 loading 兜底 | PC orgLoadingStuck=false，orgBodySample 含「工作台 / 组织架构」，无「Loading...」常驻 | 无回归 |
| t25 H5 信封归一化 | H5 envelopeOk=true，list/my 经 calendar_assemble_control 端点正常解析 | 无回归 |
| t51 H5 渐变品牌 | H5 branded=4（>0），今日/本周渐变 tab 与「+新建日历」渐变按钮保留 | 无回归 |

## 6. 与 R310 基线量化对照及取证限制结论

| 维度 | R310 基线 | R311 复查后 | 变化 |
| --- | --- | --- | --- |
| PC 我的日历 chip 数 | 0（my_calendars=[]） | 4 → 5 | 建日历入口生效 |
| PC 建事件链路 | 取证受限（无前置日历） | 事件创建 200 + 回显 + 刷新持久 | 链路打通 |
| H5 我的日历数 | 0 个 | 6 → 7（刷新仍 7） | 建日历入口生效 |
| H5 建事件链路 | 取证受限（+新建仅事件表单、calendars=[]） | 事件创建 200 + 今日列表可见 + 刷新持久 | 链路打通 |
| pageerror / API 失败 | 0 / 0 | 0 / 0 | 持平 |

→ **R310 记录的「日程新建取证受限」已解除**：PC/H5 均可通过真实 UI 新建日历并创建事件，跨刷新持久，0 错误/API 失败。改造引入的两个真 bug（PC my_calendars 信封解析与跨轨 404、H5 core entity 端点双表漂移）在真浏览器下未见复现。

## 7. 证据索引（均 gitignored）

- PC：oa4rust/test-results/r311-after/pc/evidence.json + 7 PNG；H5：oa4rust/test-results/r311-after/h5/evidence.json + 6 PNG。
- 脚本：oa4rust/test-results/r311-after-pc.cjs、r311-after-h5.cjs（运行时经 SSH 读取凭据，不入库）。
- 关键截图目视：PC 03-calendar-created（「日历已创建」toast + 新 chip）、PC 05-event-created（「事件已创建」toast + 日程条目）；H5 03-calendar-created（我的日历 7 个 + 渐变入口）、H5 05-event-created（今日列表新增日程）。

## 8. 结论

R311 建日历入口改造在 PC 与 H5 双端均通过真浏览器闭环复查：新建日历与事件创建、回显、刷新持久全部生效，R310 取证限制解除，四项历史回归无异常，全程 0 pageerror / 0 API 失败。本轮未发现新增真实缺陷。截图为本地无头浏览器渲染，非物理真机；结论基于单一时点真实 UI + 接口取证。项目持续迭代（≥1000 轮），非完成态。