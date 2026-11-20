# R300 C1 首轮全页真浏览器实测与逐页批判（PC 全路由 + H5 全路由）

- 实测日期：2026-11-21（Asia/Shanghai）
- 分支：`opt1000/oa4rust-pc-ai-refactor`
- 任务：t10 [C1] 首轮全页浏览器实测与截图
- 来源声明：全部截图来自 **Playwright Chromium 无头真浏览器** 访问测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust`（PC）与 `https://<YOUR_TEST_DOMAIN>/oa4rust-h5`（H5），已确认测试服部署为当前分支最新产物（PC 构建 `index-BUAk0CnL.js`, 时间戳 2026-11-21 03:23；H5 `index.html` 存在）。**非本地 dev server、非静态假数据、非物理真机**；视口 PC 1440×900、H5 390×844（isMobile/hasTouch/deviceScaleFactor=2）。PC 与 H5 均从测试服运行时读取真实账号经登录页 UI 登录（口令在运行时从测试服凭据文件读取，不入库、不在本文档回显）。
- 证据目录（gitignored，不入库）：`oa4rust/test-results/r300-c1-full-pc/`（74 路由 PNG + 登录流程 PNG + `evidence.json` + `run.log`）、`oa4rust/test-results/r300-c1-full-h5/`（19 路由 PNG + 登录流程 PNG + `evidence.json`）。补充取证：`r300-c1-api-probe2.json`（H5 日历 API 响应实证）、`r300-c1-pc-calendar-repro.json`（PC 日历错误 3 次复现）、`r300-c1-fetch-calendar-js.cjs`（PC 日历压缩产物定位）。
- 判定口径：`errors` = 页面级 JS 未捕获异常（pageerror）；`apiFailures` = /api/ 响应 404 或 ≥500；`ERRTEXT` = 可见文本命中「连接服务器超时/加载失败/请求失败/网络错误/data is undefined」；`EMPTY` = 可见文本命中「暂无…」。

## 一、实测覆盖与总体结论

| 端 | 路由总数 | 成功截图 | pageerror | apiFailures(404/5xx) | 可见错误文案页 |
| --- | --- | --- | --- | --- | --- |
| PC | 74（16 业务 + 58 管理） | 74/74 | 1（日历页，稳定复现） | 0 | 1（日历页） |
| H5 | 19 | 19/19 | 0 | 0 | 1（日历页） |

- 两端所有路由均可达、无整页空白/路由错配/重定向异常；登录流程通过真实 UI 完成（H5 `loggedIn=true`）。
- **本轮新发现两个此前 A2/A3 审计未记录的真 bug，且均已取得源码级根因与真实接口/堆栈实证**（见第二节 P0-1 / P0-2）。
- 未对业务代码做任何修改；本轮为实测与批判，修复由后续 RED→GREEN 轮次承接。

## 二、P0 真 bug（含实证，需 test-first RED→GREEN）

### P0-1 PC 日历页进入即抛 TDZ 异常并弹「页面发生意外错误，请重试」

- 现象：真实浏览器进入 `/app/calendar`  consistently 弹出右上角红色错误提示「页面发生意外错误，请重试！」，同时 `pageerror` = `ReferenceError: Cannot access 'q' before initialization`。日历网格仍渲染（说明错误发生在 setup 初始化阶段的非致命路径），但用户每次进页都看到错误提示。
- 复现：登录 PC → 导航 `/app/dashboard` → 进入 `/app/calendar`。独立脚本连续 3 次进入均复现，堆栈恒定：`at V (…/assets/CalendarApp-bKyuvB3o.js:1:2256) → at setup (…:1:2465)`（证据 `r300-c1-pc-calendar-repro.json`）。
- 根因（源码级）：`oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue:326` 在函数定义后立即同步调用 `loadCalendars('my')`；该函数首行写 `calListError.value = ''`，而 `const calListError = ref('')` 声明在 **第 344 行**（调用之后）。const 的暂时性死区（TDZ）导致首次同步执行访问未初始化绑定而抛 ReferenceError。压缩产物中该变量被混淆为 `q`——`async function V(e){R.value=e,q.value=''…} V('my'); … q=n('')`，与堆栈 `Cannot access 'q'` 完全吻合（`r300-c1-fetch-calendar-js.cjs` 抓取片段）。
- 修复方向（供后续轮次，不在本轮实施）：将 `loadCalendars('my')` 的调用下移到所有 ref 声明之后（含 `calListError`/`calEventsError`/`evtForm` 等），或把这些 ref 声明提前到函数定义之前；建议配一条组件挂载无 pageerror 的 RED→GREEN 测试。
- 验证方式：修复后重跑本轮 PC 全量脚本，`errors` 应为 0，且日历页无「页面发生意外错误」可见提示。
- 截图：`r300-c1-full-pc/08-calendar.png`。

### P0-2 H5 日历页「日程加载失败：S.value.map is not a function」，日程完全不可用

- 现象：H5 `#/pages/calendar/calendar` 可见红字「日程加载失败：S.value.map is not a function」+「重试」按钮，日程区空白。
- 复现：H5 UI 登录 → 工作台 → 日历快捷入口（或直接 hash 导航）。全量 E2E 与独立取证脚本均稳定复现。
- 根因（真实接口实证）：登录态下浏览器 fetch 到 `GET /oa4rust-api/api/calendar_assemble_control/calendar/list/my` 返回 HTTP 200，body 为 `{"data":{"myCalendars":[],"unitCalendars":[]},"type":"success",…}`——`data` 是**对象**（含 `myCalendars`/`unitCalendars` 两个数组字段），**不是数组**（证据 `r300-c1-api-probe2.json`）。而 `oa4rust/mobile/src/pages/calendar/calendar.vue:164` 写 `calendars.value = (cals.data ?? []) as CalendarRow[]`，仅用类型断言当作数组；随后 `:169` 执行 `calendars.value.map((c) => …)` 抛 `is not a function`，被 `:192` 的 catch 归一为上述错误文案。
- 不可通过 UI 补数绕过：该错误发生在读取日历列表阶段、与是否已有日程数据无关；即便通过 `+ 新建` 也因拿不到 `calendarId`（`calendars.value` 被整体赋值为对象）而无法提交，属结构性缺陷，必须改代码。
- 修复方向（供后续轮次）：在 `:164` 对 `cals.data` 做运行时归一化——数组直接用，对象则合并 `myCalendars` 与 `unitCalendars`；或调整后端 `/calendar/list/my` 返回数组信封。建议先写一条「给定对象形 data 时日历不报错且能渲染」的 RED 测试。
- 验证方式：修复后重跑 H5 全量脚本，日历页 `errorText=false` 且恢复「暂无日程」正常空态。
- 截图：`r300-c1-full-h5/11-calendar.png`。

### P0-3 PC 工作台统计卡与真实待办列表口径不一致（沿用 A3，本轮再确认）

- 现象：PC `/app/dashboard` 四个统计卡全 0（待审批 0 / 未读 0 / 已办结 0 / 今日会议 0），但下方「我的待办」列表有 5 条真实数据（`start:S3 运行闭环 work-…` 等）；H5 工作台同口径显示「9 待办审批」。
- 源码定位：`oa4rust/frontend/apps/desktop/src/views/Dashboard.vue:183-207` 统计卡 `statCount(0, …)` 走 `procCounts`；待办列表 `loadPending()` 走 `/api/processplatform/assemble/surface/task/list/my/paging/1/size/20`。两处数据源不同且统计侧未与列表对齐。
- 修复方向：统计卡与「我的待办」统一到同一分页接口计数，避免全零误导；配一致性断言测试。
- 截图：`r300-c1-full-pc/01-dashboard.png`。

## 三、P1/P2 逐页批判（分组，非阻断但需排期）

### PC 端（74 路由）

- **组织架构 `/app/org`**：`loading=true` 常驻、bodyLen 仅 255，端点调试 UI 仍侵入业务页（沿用 A2）。截图 `02-org.png`。
- **考勤 `/app/attendance`**：命中 EMPTY（列表「暂无考勤记录」），与统计卡 45/42/3/2 不一致（A3 已判定统计为 `AttendanceApp.vue:329-334` 硬编码假数据）。截图 `12-attendance.png`。
- **首屏数据缺失页**：通讯录 `/app/directory` 共 0 人；文件 `/app/file`、论坛 `/app/bbs`、会议 `/app/meeting`、笔记 `/app/note`、回收站 `/app/recycle` 均空态或需手动加载。H5 同口径（通讯录 E2E001…、论坛 e2e topic、待办 9 条）有真实数据，证明 PC 首屏数据加载策略落后。需按 H5 真实数据源补齐。
- **空态 14 页**：`12-attendance`、`40-admin-query-table-designer`、`42-admin-query-stat-designer`、`43-admin-query-importer-designer`、`44-admin-cms-column`、`45-admin-cms-column-manager`、`49-admin-cms-dict-designer`、`51-admin-cms-view-designer`、`54-admin-cms-module`、`59-admin-process-form-designer`、`60-admin-process-dict-designer`、`62-admin-process-xform`、`63-admin-process-application`、`69-admin-collect`。多数为设计器/管理页无种子数据（截图显示框架与操作入口完整，如 `40-admin-query-table-designer.png` 有「新建表/保存定义/添加列」），属可接受空态；考勤与论坛/会议等业务页空态需结合数据补数或引导优化。
- **轻微 loading 未收敛**：`19-admin-hotpic`、`21-admin-appinfo` 采集时 `loading=true`（页面主体已渲染，疑为骨架/局部加载标记未清），非阻断。
- **可读性与调试侵入**：待办/流程展示原始 UUID 与 `work-…` 串；多处「端点调试」入口暴露于业务页；可见「已切换到：XXX」无障碍 live 提示进入视觉文本（沿用 A2/A3）。

### H5 端（19 路由）

- **消息 `#/pages/message/message`**：8 条会话标题均为原始 `im-conv-…` ID，缺可读姓名/摘要。截图 `03-message.png`。
- **审批 `#/pages/process/process` / 发起流程 `#/pages/process/start`**：待办/流程列表可用，但标题为 `start:S3 运行闭环 work-…`、流程名为 `s3-process-…` 原始 ID，语义弱。
- **考勤 `#/pages/attendance/attendance`**：「未加入考勤组/今日暂无打卡记录/月度正常 0 异常 0」；与 PC 考勤统计 45/42/3/2 口径差异大。
- **文档 `#/pages/doc/doc`**：「暂无附件/暂无文件」；**会议**「暂无待接受的会议邀请」；**收藏**「暂无收藏」；**浏览**「暂无数据」；**回收站**为空——多处空态缺引导文案。
- **论坛 `#/pages/bbs/bbs`**：有 e2e topic 主题数据（较 A2 的「连接服务器超时」已恢复），但 **主题详情 `#/pages/bbs/topic`** 显示「主题不存在或已被删除」（deeeplink 未带 id，属参数缺失，非数据缺陷）。
- **我的 `#/pages/mine/mine`**：admin 账号邮箱/手机号/部门均未设置，含修改密码表单；信息完整度低。
- **登录页 `#/pages/login/login`**：全量脚本在已登录态二次访问仍显示工作台内容（路由未强制跳登录），行为可接受，记录备查。

## 四、数据补数与取证说明

- 本轮所有页面均通过**真实 UI 登录**后采集，未使用静态假数据。对 H5 日历缺陷做了专门的补数可行性论证：其错误源于接口信封类型不匹配（P0-2），任何 UI 补数都无法绕过，必须以代码修复 + RED→GREEN 处理，故未做无效的数据构造。
- 管理区设计器空态（第三节 14 页）框架与操作入口完整、属「无种子数据」而非功能缺失，本轮不伪造数据填充；若后续评审需种子数据，应走真实 UI 新建流程逐条添加。
- 对两个新 P0 均做了超出单次截图的额外取证：PC 日历 3 次独立复现 + 压缩产物堆栈定位；H5 日历抓取真实 API 响应体确认 `data` 为对象。二者均为确定性缺陷，非偶发。
- 补充：早期 A2 记录的 H5 bbs/search/mine「连接服务器超时」在本轮全量采集中**未复现**（本轮 bad=0 / errors=0 / apiFailures=0），判断为当时 tabbit 工具链或测试服瞬时状态，已不作为当前业务缺陷；保留历史记录备查。

## 五、本轮结论

R300 C1 首轮全页真浏览器实测完成：PC 74 路由 + H5 19 路由逐页截图留证，覆盖全部可达页面。确认两个确定性 P0 真 bug（PC 日历 TDZ、H5 日历 data 信封类型错误，均含源码级根因与真实接口/堆栈实证）并给出修复方向；其余按 P1/P2 排期。所有截图为测试服无头真浏览器渲染，已如实标注为「非物理真机」。本轮未修改业务代码，不是项目完成，仅为 ≥1000 轮迭代中的首轮全页实测记录；下一步由队长据本清单安排 RED→GREEN 与后续设计评审轮次。
