# R610b MeetingApp 预订日期 UTC 日偏移真修复

- 结论：真 bug 已修复，RED→GREEN 坐实。MeetingApp.vue:161 bookingDate 用 new Date().toISOString().slice(0,10) 取 UTC 日期，UTC+8 每天 00:00–08:00 显示昨天，预订默认日期错误；改用业务日助手 businessTodayYMD() 后按固定 UTC+8 口径取业务今日。
- 真父 HEAD：a8caa2857ef91a139a6d8be039482bc1710107b1（git 核实 HEAD 等于真父；RED replay 全程 HEAD 不变）。
- 写域精确 3 文件：M MeetingApp.vue（仅 :119 / :161 两行）、A 契约测试、A 本文档。其余零改；date.ts / meetingStatus.ts / 台账零改。

## 立项与机制锚点
- R609b 甄别判 P2：MeetingApp.vue:161 const bookingDate = ref(new Date().toISOString().slice(0, 10))，绕过项目 R167 业务日助手。
- 业务助手 date.ts:113-119 businessTodayYMD(now = new Date()):string 把时刻平移 +8h 后按 UTC 字段读年月日，结果与浏览器时区无关；:105-111 注释明确禁用 toISOString().slice（UTC+8 凌晨回落到昨天/上月，影响考勤默认日期、今日筛选、导出文件名）。
- 缺陷仅影响新建会议表单的默认预订日期初值；onBookingDate/calendarDays/createMeeting 等交互逻辑零改。

## 修复（diff 1 file changed, 2 insertions(+), 2 deletions(-)）
- :119 import { formatServerDateTime } from '../utils/date' → import { businessTodayYMD, formatServerDateTime } from '../utils/date'。
- :161 ref(new Date().toISOString().slice(0, 10)) → ref(businessTodayYMD())。其余行零改。

## RED → GREEN（队长可独立 replay）
| 版本 | 定点 vitest 契约 | 关键失败点 |
|---|---|---|
| 真父 MeetingApp.vue（git show 真父覆盖） | 3 failed / 2 passed（1 file failed），EXIT=1 | 凌晨跨日 2025-12-31 vs 期望 2026-01-01；跨月 2026-01-31 vs 2026-02-01；源码守卫 initializer=new Date().toISOString().slice(0, 10) |
| 修复态（OPS CAS 恢复两行） | 5 passed / 0 failed（1 file passed），EXIT=0 | 4 路径全过：捕获 initializer、凌晨跨日、跨月、白天正常、源码守卫 |
- replay 实测：git show a8caa2857:…/MeetingApp.vue 覆盖后跑 pnpm vitest run src/views/meetingBookingDate.test.ts → RED_EXIT=1；OPS edit_file 精确恢复两行 → GREEN_EXIT=0；POST_HEAD=a8caa2857… 未变；恢复后源文件 sha256 与修复态一致（5d0b2766…）。
- 契约 src/views/meetingBookingDate.test.ts：从 Vue 源码正则抽 bookingDate initializer，vi.setSystemTime + new Function 注入真 businessTodayYMD 求值，4 条行为/守卫断言 + 1 条捕获断言。

## 三 verify
- V1：cd oa4rust/frontend/apps/desktop && pnpm vitest run src/views/meetingBookingDate.test.ts → 1 file / 5 tests passed，EXIT0。
- V2：pnpm typecheck → tsc --noEmit EXIT0；pnpm vitest run 全量 → 2246 tests 全 passed，280 files passed / 2 files failed（282），EXIT1 仅因 2 个既有基线 suite 加载失败：tabStateA11y.contract.test.mjs 无 test suite；TaskCompleted.R314.contract.test.ts 需 backend cwd 否则源码路径 ENOENT。两文件 git status 干净未改、非本次引入；据实披露不阻断。
- V3：契约文件 biome check EXIT0；MeetingApp.vue 全文件 biome check EXIT1（1 format error + 2 useTemplate info，诊断行 214/699–720），真父版经 stdin 同检同样报 format 未修复错误（EXIT1），属既有基线、远离本次改动行 119/161（diff hunk 精确在 119/161），不做整文件 --write 以免违反 surgical 原则扩散 diff。
- 已知盲区：desktop 包 typecheck script=tsc --noEmit 非 vue-tsc，不解析 .vue，故 V2 不覆盖 MeetingApp.vue；真正的 .vue 守卫由 V1 契约 + V3 承担，据实披露。

## 六维守卫（零外溢）
- MeetingApp.vue diff 仅两行（@@ -119 +119 @@ / @@ -161 +161 @@）；onBookingDate、calendarDays、createMeeting、模板、style 全部零改。
- date.ts（业务助手本体）、meetingStatus.ts、docs/ops/current-state.md、components/、其他 views/、backend crates、迁移全部零改。
- 精确 git add 仅 3 写域文件（单引号路径，禁 -A/.）；他组并行改动与既有 untracked 完全未触碰。

## 三分判定与 Fail Loud
- 可达真缺陷：1 项（P2，UTC+8 凌晨默认预订日期显示昨天），已修复；不可达：0；正常/非缺陷：0 新增。
- 据实披露：全量 vitest 2 suite 基线失败非本次引入；biome 整文件 format error 真父已存在；typecheck 不解析 .vue；均不影响 RED/GREEN 决定性证据与修复正确性。
- 严守 Secrets：文档与契约无真域名/IP/口令/私钥/真实用户目录，0 真值。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=f4748567687313b566cb5adc023fe93601b3286bdb3e964cc8ec128842bfda5d
  meta={task:t200,round:R610b,verdict:P2-fixed,trueParent:a8caa2857ef91a139a6d8be039482bc1710107b1,fix:MeetingApp.vue bookingDate new Date().toISOString().slice->businessTodayYMD(),contract:meetingBookingDate.test.ts}
- frontend/apps/desktop/src/views/MeetingApp.vue（修复态）sha256=5d0b27660b583b2672397e514a217caa0aa9041306880d91dc2433b033d51d9d
- frontend/apps/desktop/src/views/meetingBookingDate.test.ts sha256=acdda8d38508f451796317e8c7cc37d4f9e40e31343012365264d458ee5700ee

## 修复方向
- 已完工：预订日期默认改用业务日助手，UTC+8 全天显示业务今日；后续可将同类裸 toISOString().slice 取日期处统一收敛到 date.ts 助手（本轮不在写域、不扩散）。
