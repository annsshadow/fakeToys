# oa4rust H5 考勤月度统计字段错配修复（R356）

- 轮次：R356 / 任务 t166；角色 page-core（工作台与个人域工程师）
- 对象：oa4rust/mobile/src/pages/attendance/attendance.vue（考勤打卡页 statText）
- 性质：真 bug 修复（P2），test-first RED→GREEN，源码级契约 + 390x844 真浏览器注入对照复验
- 依赖：t165 / R355 只读审计（commit e30bc5ed0），坐实月度统计恒显「正常 0 · 异常 0」P2
- 测试服：域名以占位符 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 代替，真值运行期从 .local.keys / ssh oa4rust-test 读取，未入库、未回显
- 取证环境：Playwright Chromium headless，390x844，isMobile + hasTouch，DPR 2；截图均为 headless 模拟移动视口，**非物理真机**

## 1. 任务与验收标准

修复 R355 审计坐实的 P2：statText 读平铺 normalCount / abnormalCount，后端 v2_my_statistic 实际返回 detailByStatus 数组，任何真实数据都显示 0。要求 statText 从 detailByStatus 按 status 聚合并兼容平铺 fallback，不改后端 / SDK / services / desktop / 其它页 / 端点，RED→GREEN，mobile 与根两套 vitest 全绿，真浏览器注入对照复验。

## 2. 根因、裁决与边界（fail-loud）

根因：attendance.vue statText 旧实现读 s.normalCount / s.normal、s.abnormalCount / s.abnormal；后端 attendance_assemble_control/src/lib.rs v2_my_statistic 真实返回 detailByStatus:[{status,count}] + recordCount，无平铺字段，故恒取 0。R355 两轮真浏览器 v2 注入对照铁证：真实形状 normal=5/abnormal=3 显示 0，平铺形状 7/2 显示正确。

队长裁决方案A：statText 从 s.detailByStatus（Array.isArray 判空）按 status 聚合 normal / abnormal 计数，聚合值优先；平铺 normalCount / abnormalCount 仅作 fallback（保留旧形状兼容）；status 字面量兼容大小写与中文「正常 / 异常」，同状态多行求和，坏元素与坏 count 跳过；month 取值与其余文案不变。实测状态字面量若不符裁决须先回报队长——本轮实测后端 status 为 normal / abnormal，与裁决一致，未触发回报。

硬边界（本轮严格未越界）：不改后端 / SDK / mobile services/index.ts 与 http.ts / desktop / 其它页 / migrations；不改任何端点路径与 payload；不改 doCheck / doCheckFromOut / applyLeave 打卡与请假端点逻辑；不改 month 文案与模板调用。破坏性真实打卡 / 请假写端点未授权，未执行。

## 3. 修复内容

- attendance.vue statText：新增 detailByStatus 聚合（Array.isArray 判空、逐行校验对象与 status、normal / abnormal 两桶大小写与中文归一、Number.isFinite 校验 count 后求和、recognized 标记决定聚合是否优先）；无有效聚合行时回退原平铺读取；month / 暂无统计文案与模板调用原样保留。
- 契约 attendance.stat.contract.test.ts（新建）：源码级 bodyOf 花括号配对提取 statText，剥离 TS 标注后 new Function 真执行注入 myStat 桩，7 项断言覆盖真实形状聚合、多状态求和、平铺 fallback、聚合优先、坏数据容错、month 文案与写读链保护。函数参数均带冒号 string 标注防 TS7006，不引入新依赖。

## 4. RED→GREEN 台账

- 定点契约 RED：4 failed / 3 passed（真实形状聚合、多状态求和、聚合优先、坏数据容错均失败，坐实旧逻辑读错字段）；修复后 GREEN：7 / 7 passed。
- mobile 全量（cd oa4rust/mobile && pnpm exec vitest run）：74 files / 476 tests，0 failed。
- 根全量（cd oa4rust && pnpm exec vitest run）：276 files / 2210 tests，0 failed，既有测试无回归。
- mobile typecheck（pnpm exec tsc --noEmit）：exit 0。

## 5. 构建、部署与真浏览器复验（非物理真机）

H5 build（pnpm exec uni build）exit 0，入口 index-CfObUd-W.js；整目录 tar 原子替换测试服 h5 目录（DEPLOY_R356_H5_OK）。390x844 headless Chromium 注入对照脚本 test-results/r356-h5-attendance-check.cjs：登录 → 考勤页 → A_detailOnly 注入 detailByStatus normal=5/abnormal=3 → B_flatOnly 注入平铺 7/2 → 真实后端只读探针；五项断言 loginOk / A_detailOnly / B_flatOnly / realNoFalseZero / noPageErrors 全 true。

实测值：A 「2026-11 · 正常 5 · 异常 3」（修复前为 0）；B 「2026-11 · 正常 7 · 异常 2」；真实后端「2026-11 · 正常 1 · 异常 0」（HTTP 200，真实样本 1 条 normal，不再恒 0 误报）。0 pageerror。

截图 01-detailonly-fixed.png / 02-flatonly-compatible.png / 03-real-server.png 与 evidence.json 落 gitignored test-results/r356-h5-attendance-stat/，evidence source 明确标注 Playwright Chromium headless 390x844、非物理真机。

## 6. 取证限制与结论

打卡 check / checkFromOut 与请假 selfHolidayApply 真写端点未获授权，本轮未执行，仅保留端点调用未改的源码级断言；真实后端样本当月仅 1 条 normal 记录（无 abnormal），真实计数以样本为准，注入对照已独立坐实聚合逻辑正确。P2 已修复：月度统计从恒 0 改为按后端真实 detailByStatus 聚合显示，兼容旧平铺形状，0 pageerror。项目持续迭代中，非终态。

## 7. 敏感自检

文档仅含 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位符，无真域名 / IP / 口令 / 私钥 / 个人路径；真值运行期读取未入库。截图来源均如实标注为 headless 非物理真机。