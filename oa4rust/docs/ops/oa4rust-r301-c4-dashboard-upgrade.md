# oa4rust R301 工作台与个人域实质升级实测记录（C4 / t13）

> 截图来源：Playwright Chromium headless 对本地部署测试服的真实登录与页面渲染，非物理真机。
> 测试域名、账号与口令一律不在本文件落值，统一以 <YOUR_TEST_DOMAIN> 等占位符表示。

## 一、改动依据

- t11 设计评审 findings：DR-02 工作台待审批统计卡 0 与待办列表 5 条口径矛盾；DR-04 工作台缺语义 h1、未接入 ai-page 容器（前序已修）。
- R300 基线审计 docs/ops/oa4rust-page-audit-r300.md：工作台统计多项恒 0、待办暴露原始流程键（t37 已修可读性）。
- 本次实质问题：今日日程硬编码部门例会 / 项目评审假安排；通知公告伪造报销已通过 / 张三发消息动态；quickApps 八项入口已定义却从不渲染；统计第四项今日会议写死 0。

## 二、实施内容（Dashboard.vue）

1. 快捷入口：新增 quick-access-section，用 v-for 遍历 quickApps 渲染 8 个真实路由入口，data-testid 为 dashboard-quick-grid；标签 white-space nowrap 加最小栅格 116px，避免中文标签换行。
2. 今日日程：loadTodaySchedule 先取 /api/calendar/calendar/list/my，合并 my/unit/follow 日历；非空再按日历 id 取 /api/calendar/event/list/{id}，过滤当日事件并排序；无日历或无事件显示暂无日程安排与去日历添加引导，不再有硬编码假安排。
3. 通知公告：loadRecent 改取 /api/message/assemble/communicate/instant/list/currentperson/count/20/desc 的真实 content 与 createTime；空数据显示暂无通知公告。
4. 统计卡：第四项由硬编码今日会议 0 改为今日日程，值与真实日程加载状态同源；新增 stat-distribution 分布条，statDistribution 按相对最大值归一表达同屏量级对比，全 0 不画条，避免单项非零被误读为独占全部。
5. 保持既有契约与业务不变：DR-02 待审批仍与分页待办同源；t37 待办业务名优先加原始键折叠副文本；B2/visualRhythm 的 AI 令牌与 testid 全部保留。

## 三、验证结论

- RED→GREEN：先建 Dashboard.r301.contract.test.ts（5 断言），RED 基线 4 failed / 1 passed；GREEN 后含既有 5 份契约共 6 files / 26 tests 全通过。
- pnpm --filter @oa4rust/desktop typecheck：通过（exit 0）。
- pnpm --filter @oa4rust/desktop build：通过（exit 0），入口 index-CL6l5TAK.js 与 index-BMFkaD73.css。
- 测试服原子部署：打包、scp、web.incoming、原子切换 web/web.old，远端 grep 入口哈希与本地构建一致。
- 真浏览器实测（1440×900，登录 <YOUR_TEST_DOMAIN> 测试服工作台）：h1 早安 admin 唯一；快捷入口 8 项单行平铺无换行；待审批 5、未读 0、已办结 0、今日日程 0；fakeSchedule false（无假安排）；日程空态暂无日程安排去日历添加；通知来自真实消息；pageErrors 为空。

## 四、证据

- 截图：test-results/r301-c4/01-dashboard.png（1440×900，sha256:cfaba54b199182303ec9593b053940e7a85f440e6444e9e5b81890722af301b4）。
- 结构化证据：test-results/r301-c4/evidence.json，source 字段标注 Playwright Chromium headless against deployed test server; not a physical device。
- 上述 test-results 目录为 gitignored 本地产物，仅保留在本地工作区，不入库真值。

## 五、范围与后续

- 个人中心、设置、IM 本轮未改业务逻辑（沿用 t37 与 B2 成果）。
- 项目持续迭代中，本轮非终态；后续按独立设计评审结论续作。
