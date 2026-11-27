# R609b MeetingApp 预订日期 UTC 日绕过业务日助手甄别（验证型甄别）

- 结论：P2。MeetingApp.vue:161 预订日期默认值 `new Date().toISOString().slice(0,10)` 取 UTC 日；UTC+8 每天 00:00–08:00 显示昨天（跨月显示上月），未来 7 天日历整体偏移一天，不手改会订到昨天。纯展示默认日期偏移，不写库错误、不越权、不崩溃，用户可手改，凌晨时段可感知。
- 派发真父 HEAD：c78dee77692b4242dc31c20691bab0c2040edabf（git rev-parse 核实）。
- 零源码改；唯一产出：本文档。测试探针与日志在 gitignored 临时路径，用后即删。

## 立项与范围
- 主对象 frontend/apps/desktop/src/views/MeetingApp.vue:161 bookingDate 默认值、:162-167 onBookingDate、:20 日期输入、:77/:168-185 calendarDays、:237 createMeeting POST。
- 正确口径参照 frontend/apps/desktop/src/utils/date.ts:99-119 businessTodayYMD（固定 UTC+8，+8h 平移取 UTC 字段）。
- 路由入口 frontend/apps/desktop/src/main.ts:94-99 path 'meeting' → MeetingApp.vue（真实 UI）。
- 不扩范围：不修改任何源码/默认值/路由/测试配置；仅甄别取证。

## 机制锚点（真父源码逐行）
- MeetingApp.vue:161 `const bookingDate = ref(new Date().toISOString().slice(0, 10))`：UTC 日。
- :162-167 onBookingDate 将 bookingDate 写入 `form.value.startTime = day+'T09:00'`、`endTime = day+'T10:00'`；:237 POST /api/meeting/assemble/control/meeting/create 下发 startTime/endTime，默认日语义直接进入业务数据。
- :119 仅 `import { formatServerDateTime } from '../utils/date'`，全文件 businessTodayYMD 零命中；全 views 中 businessTodayYMD 命中 11 文件，R167 后 9 视图 29 处全改助手，唯独本视图漏改。
- date.ts:102 BUSINESS_UTC_OFFSET_HOURS=8；:108 注释明确断言不能用 toISOString.slice(0,10|7)，UTC+8 凌晨 00:00–08:00 回落昨天/上月；:113-119 businessTodayYMD 平移 +8h 后按 UTC 字段拼接。
- git blame -L 161,161 = 86867e1d4（2026-11-20），晚于 R167（2026-11-18），故 R167 复查时该行尚不存在；审计台账 bookingDate/预订日期/onBookingDate/calendarDays 全 0 命中，R326/R296 契约只查控件存在不查默认值口径。

## 决定性实测（vitest 真执行）
- 探针 tests/r609b_probe.test.ts（临时，已删）：4 用例；命令 cd oa4rust && npx vitest run tests/r609b_probe.test.ts → EXIT 0，4 passed / 0 failed（vitest v5.0.5，9ms）。
- 用例 1：vi.setSystemTime('2025-12-31T16:30:00Z')（北京 2026-01-01 00:30）下 bookingDate 表达式 = '2025-12-31'，businessTodayYMD = '2026-01-01'，differ=true。
- 用例 2：从 date.ts 源码正则抽 businessTodayYMD 函数体，new Function 注入 8h 偏移真执行得 '2026-01-01'。
- 用例 3：从 MeetingApp.vue 正则抽 bookingDate 表达式，new Function 真执行得 '2025-12-31'；import 行含 formatServerDateTime 而无 businessTodayYMD。
- 用例 4：锚点守卫——日期输入、calendarDays、onBookingDate 写时间、POST create、main.ts 路由均存在。
- 决定性日志（gitignored 临时路径，用后即删）：<PROBE_LOG_PATH>，字节 sha256=47ed37ec868b74c2f8ce693e86de0018bafb84fc6eafcb4818ea4570d428f677。

## 三分裁决（据实，不夸大）
1. 判 P2：默认首屏预订日期在 UTC+8 每天头 8 小时显示昨天、跨月显示上月，未来 7 天日历偏移，用户可感知且默认下单会订到错误日期。
2. 不判 P1：非数据写错、非越权、非崩溃、非阻断；用户可在日期输入手改纠正；仅默认值与展示口径问题。
3. 不判 P3：非边缘文案，是会议室预订核心默认日期，每天固定时段稳定复现。
4. 非安全族：与 IDOR/DEV 门控/写 envelope 假成功无关，根因是 UTC 日与业务日口径错配。

## Fail Loud
- 未跑浏览器 GUI 真页面；以 vitest 真执行 + vi.setSystemTime 冻结时刻 + new Function 真调用源码表达式替代，未静默跳过。
- typecheck 门 `pnpm --recursive run typecheck` 中 desktop 实为 tsc --noEmit，不解析 .vue script setup，故 TS 层不会捕获本类默认值口径问题，缺陷存活 HEAD。
- 探针与日志均在 gitignored 临时路径并已删除，未入库；未读取或回显任何密钥/口令/真实地址。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=d55986f08fbe34e5427d6462113ba3332880ab44992af0e12c016c7f506ddbdb
- meta={"task":"t196","round":"R609b","kind":"verification-work","target":"apps/desktop/src/views/MeetingApp.vue bookingDate UTC day vs businessTodayYMD","verdict":"P2","evidence":"vitest 4passed + vi.setSystemTime(2025-12-31T16:30Z) + new Function real date.ts/MeetingApp expression exec","parent":"c78dee77692b4242dc31c20691bab0c2040edabf","head":"c78dee77692b4242dc31c20691bab0c2040edabf","logSha":"47ed37ec868b74c2f8ce693e86de0018bafb84fc6eafcb4818ea4570d428f677"}
- 证据日志字节 sha256=47ed37ec868b74c2f8ce693e86de0018bafb84fc6eafcb4818ea4570d428f677
- frontend/apps/desktop/src/views/MeetingApp.vue=1381feea87c2c8030bd248bd097ac28d2dc5151a2259fd59cc3324e27f450205
- frontend/apps/desktop/src/utils/date.ts=3dac93cd37fb5ff8fbf4235e953383e5e1df158183e54cd504925d768147d553
- frontend/apps/desktop/src/main.ts=e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0

## 修复方向（不在本任务范围）
- MeetingApp.vue:119 引入 businessTodayYMD，:161 改为 ref(businessTodayYMD())，calendarDays 基准与默认 startTime/endTime 随之使用业务日；补一条冻结 UTC+8 凌晨时刻的默认值回归测试。
