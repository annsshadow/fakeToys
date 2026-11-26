# R311 PC 日历新建入口与创建闭环

> 来源：AgentTeams 任务 t70（R311 PC 建日历 UI 入口）。本轮为本地无头浏览器实测，非物理真机。

## 背景与真实缺口

R310 审计指出：干净数据环境 my_calendars 为空会导致整个日程功能不可用。侦察确认后端日历创建接口已注册，PC 端 CalendarApp.vue 也已写 calWriteCE('calCreate') 分支，但模板全文没有调用点，用户无法新建日历，进而事件创建（依赖 activeCal）也不可用。属于“接口有、前端缺入口”类型缺口。

说明：任务描述引用的审计文档路径在工作树中查无文件；缺口判断以源码侦察与真浏览器探测为准，未臆造该文档内容。

## 诊断出的三个真实缺陷（均先 RED 后 GREEN）

1. 无可见入口：calWriteCE('calCreate') 仅定义未绑定，用户不可达。
2. 列表信封解析错误：loadCalendars 原逻辑取 r.data.list / r.data，无法识别后端 {data:{my_calendars:[...],unit_calendars:[...],follow_calendars:[...]}} 信封，导致已建日历 chip 数不增长。修复为取 data.my_calendars 优先，回退 list / unit_calendars / 空数组。
3. 双轨隔离：后端同时挂载 calendar::router（legacy Pool/原始 SQL）与 calendar_core_entity::router（sea_orm Entity）。原 calCreate 调 core/entity/calendar/create 写入 core 表，而列表、详情、事件读走 legacy 表，跨轨记录互不可见，详情接口 404。统一改走 legacy 同轨：创建 /api/calendar/calendar/create、列表 /api/calendar/calendar/list/my、详情 /api/calendar/calendar/{id}、事件列表 /api/calendar/event/list/{id}、事件创建 /api/calendar/event/create。

## 实现要点

- 在“我的日历”作用域区新增可见入口按钮 data-testid="calendar-create-entry"，文案“+ 新建日历”；点击展开最小表单 data-testid="calendar-create-form"（名称输入 + 颜色输入 + 提交/取消），替换原 prompt() 交互。
- 提交调用 calWriteCE('calCreate')，创建成功后 toast 提示、重载 loadCalendars('my') 并自动 selectCalendar 新日历 id，直接打开该日历的日程管理面板。
- 保留既有日历/事件读路径与 ai-page 令牌样式（--ai-glass-*、--ai-gradient-brand、--ai-motion-fast、--ai-ease-spring、--ai-shadow-*），未回退。

## 契约与验证

- 新契约 CalendarCreateEntry.R311.contract.test.ts 共 6 个断言：可见入口+data-testid、create 与 list 同轨、calCreate 分支无 prompt 且用表单、成功后刷新列表并选中新日历、my_calendars 信封解析、ai-page 与既有读路径不回退。
- RED→GREEN：首版 4 failed/1 passed；信封断言 RED 1 failed/5 passed；同轨断言 RED 1 failed/5 passed；GREEN 6 passed/6。
- pnpm run typecheck exit 0（Scope 6/7）；pnpm --filter @oa4rust/desktop build exit 0。
- 测试服原子部署（升级脚本本地 gitignored）：备份 web→web.bak-STAMP、scp 到 /tmp、解到 web.incoming-STAMP、原子替换并 grep 新构建哈希，部署 exit 0。

## 真浏览器实测（Playwright Chromium headless）

- PC 1440x900、H5 390x844（isMobile/hasTouch）均经登录进入 /app/calendar 真实操作：点入口→填名称→提交→chip 数 +1→新日历可见可选→日程管理面板指向新日历→新建日程并回显标题。
- 8 项断言全绿：entryVisible / chipIncrement / newCalendarSelectable / managerOnNewCal / eventCreated / create200 / noPageErrors / noApiFailures；pageErrors=[]，API 失败=[]。
- 截图与 evidence.json 落 gitignored 证据目录，标注“本地无头浏览器渲染，非物理真机”。
- H5 截图为桌面页在窄视口下的自适应态（侧栏抽屉展开），原生移动页不在本任务 inScope。

## 脱敏说明

测试域名、主机/SSH 地址均用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位；账号口令仅运行期由远端凭据脚本读取，不入库、不回显。部署脚本与浏览器脚本位于本地 gitignored 目录，不在提交范围。