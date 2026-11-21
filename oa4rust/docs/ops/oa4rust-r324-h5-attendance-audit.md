# OA4Rust R324 H5 考勤深度功能审计

- 轮次：R324（H5 考勤首次深度功能审计，只读，不改源码/契约/配置、禁部署禁修复）
- 分支：opt1000/oa4rust-pc-ai-refactor（非 main）
- 相关任务：t116 本文审计；对照 R317 H5 审计与 R321（t104）避免重复
- 测试服构建：H5 入口 index-BUQSTyeU.js（构建哈希，非地址真值）
- 真浏览器：Playwright Chromium headless 390x844 isMobile/hasTouch DPR2；本地无头浏览器渲染，非物理真机。

## 1. 来源与边界

- 站点地址、账号口令、会话数据一律不入库；本文统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，口令仅运行期经 gitignored 脚本读取不回显。
- 证据截图、E2E/补测/DOM 探查 JSON 与脚本落 gitignored 目录 oa4rust/test-results/r324-h5-attendance/ 与 oa4rust/test-results/r324-*.cjs，均标注非物理真机。
- 判定纪律：①真缺陷需真 UI 可复现 + 真请求/响应/页面回显/源码定位；②取证限制（无 UI 入口、账号非目标态、无头 picker 不兼容）不臆造；③产品语义差异（不同 API base/表族不同源）不判 bug。

## 2. 源码侦察（端点与 DOM 入口）

- 页面 oa4rust/mobile/src/pages/attendance/attendance.vue：onShow → ensureAuthenticated → load()（preCheck）+ void loadMine()（:46-50）。load() :32-44 捕获异常写 preError=describeLoadError('考勤信息加载失败', e)，finally 复位 loading；注释明确「刷新失败保留旧状态；首次失败必须给出错误，不能伪装成未加入考勤组」。
- 服务 oa4rust/mobile/src/services/index.ts:257-293 attendanceApi：preCheck GET v2/mobile/check/pre；check POST v2/mobile/check；myStatistic POST v2/my/statistic；myDetailList POST v2/my/detail/list；myVersion GET v2/my/version；checkFromOut POST v2/mobile/check/ from/out（同 handler 兼容 %20，sourceType=外部设备）；selfHolidayApply POST selfholidaysimple。H5 fetch 前缀 /oa4rust-api。
- 后端 oa4rust/backend/crates/attendance_assemble_control/src/routes.rs:305-307 注册 v2/mobile/check/pre（GET）、v2/mobile/check（POST）、v2/mobile/check/ from/out 与 /%20from/out（POST，同 handler v2_mobile_check_from_out）；:178 selfholidaysimple POST；:258 v2/my/statistic POST；:312 v2/my/detail/list POST。空 account 未加入考勤组时 pre.canCheckIn=false，四个打卡 button :disabled 置真。
- DOM 注意：uni-app 编译后 input.leave-input 实际渲染为 input.uni-input-input（placeholder 不落 DOM 属性），按钮为 uni-button.leave-btn；打卡按钮真实 disabled 取决于 pre.canCheckIn。

## 3. 真浏览器实测结果（六段，pageerror 全程 0）

| 段 | 场景 | 真请求/响应 | DOM 回显 | 判定 |
|---|---|---|---|---|
| A0 | 考勤 preCheck 预检态 | GET v2/mobile/check/pre → 200 | group='未加入考勤组'、canCheckIn=false、hero 提示未加入；补测 v2 坐实 preCheckStatus=200 | PASS（真请求成功；未入组是账号态非缺陷） |
| A1 | 普通上班打卡 | 按钮真实 disabled（canCheckIn=false），未发写请求 | 4 打卡按钮 disabled=true | LIMITATION（测试账号未加入考勤组，无合法写入口，禁伪造写） |
| A2 | 外勤打卡 | 按钮真实 disabled；GET 探测 ' from/out' 与 '%20from/out' 两形态均 405（路由存在但方法不符） | 外勤上班/下班 disabled=true | LIMITATION（账号态无法真写）+ 端点形态只读确证（路由双形态注册一致） |
| A3 | 自助请假申请 | 真实 UI 提交 POST selfholidaysimple → 200（补测 v2 leaveCalls s=200，leaveBtnEnabled=true） | 2 个 input.uni-input-input 填日期+事由，uni-button.leave-btn .tap() 真实派发 | PASS（真实写路径通） |
| A4 | 月度统计与明细 v2/my/* | POST v2/my/detail/list、v2/my/version、v2/my/statistic 均 200 | statText='2026-11 · 正常 0 · 异常 0'，版本 v2，月度明细 1 条 | PASS（三端点口径一致 rev102） |
| A5 | 加载竞态：刷新失败保留旧态 | page.route 强制 v2/mobile/check/pre → HTTP 500 后 reload | .tip-error 文本='考勤信息加载失败：HTTP 500 重试'，.retry-btn 可见，旧组态/旧统计保留 | PASS（首次失败给错误、刷新失败保旧态，非伪装未入组） |

首轮 A0/A3 曾报 LIMITATION，经 DOM 探查（r324-dom-probe.cjs）与补测（r324-supplement-v2.cjs）更正：A0 是取错响应集合（页面内部 fetch 进 page.on('response')→ev.api，非 api() 辅助函数的 ev.responses）；A3 是 uni-app 把 input.leave-input 编译成 input.uni-input-input。二者均为脚本归因/选择器误报，非业务缺陷，已用真浏览器补测更正。

## 4. 判定

- 真缺陷（P0/P1/P2）：0。六段场景中 A0/A3/A4/A5 真请求真回显 PASS；A1/A2 打卡写路径因测试账号未加入考勤组而无法真写，属取证限制，非缺陷。
- 取证限制：①打卡（普通/外勤）写操作——账号 canCheckIn=false，按钮真实 disabled，禁伪造写；②外勤端点仅以 GET 405 只读确证路由双形态注册（空格与 %20），未做实打卡写。
- 产品语义差异：无（本次考勤各端点同属 assemble/control 族，未发现跨表族不同源问题）。
- 加载竞态与月度 v2 端点口径：均符合源码意图与契约（attendanceLoadRace/detailRows 类），无 RED 迹象。

## 5. 修复方向与验证方式

- 本次无真缺陷，故无修复项。若要覆盖打卡写闭环，建议队长另派时使用已加入考勤组的测试账号在真 UI 走 checkIn/checkOut 与 check/ from/out，监听 POST v2/mobile/check 与 /%20from/out 的 200 + duplicated 字段并按页面回显复验。
- 验证方式：保留 gitignored 脚本 r324-h5-attendance.cjs / r324-supplement-v2.cjs / r324-dom-probe.cjs，后续轮次可直接重跑比对；证据见 oa4rust/test-results/r324-h5-attendance/。

## 6. 复现路径与证据

- 环境：运行期从 gitignored 部署脚本取 H5 测试地址与口令（不回显），Playwright Chromium 390x844 isMobile/hasTouch 登录 H5 → 工作台 → 考勤页。
- 主 E2E：oa4rust/test-results/r324-h5-attendance.cjs（证据 evidence.json + A0..A5.png）。
- DOM 探查：oa4rust/test-results/r324-dom-probe.cjs（证据 dom-probe.json + A0-dom-probe.png）。
- 补测：oa4rust/test-results/r324-supplement-v2.cjs（证据 supplement.json + A3-leave-supplement.png）。
- 以上截图均为本地无头浏览器渲染，非物理真机；站点地址与口令均不入库。

## 7. 结论

R324 H5 考勤深度审计完成：preCheck 预检、自助请假申请、月度统计/明细 v2、加载失败兜底四段真浏览器 PASS；普通/外勤打卡写操作因测试账号未加入考勤组属取证限制（按钮真实 disabled，未伪造写）；0 新增 P0/P1/P2 真缺陷；首轮两处脚本误报已用 DOM 探查与补测更正。项目持续迭代中，非终态。