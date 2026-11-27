# R597b 修复：申诉状态值域错配（P2）前后端对齐

- 任务：t156/R597b（implementation）；真父 HEAD：`056cade063d4d954eebb817559388e8e3148e429`
- 立项来源：t152/R596b 验证型甄别（判 P2 确凿），文档 `oa4rust-r596b-appeal-status-audit.md`
- 结论：修复完成，RED→GREEN + 三 verify 全通过；真 DB 三 case 与前端四值映射坐实。
- 写域精确：后端 list SELECT 表达式、前端两映射函数 + 审批门、后端契约测试、前端契约测试、本文档。
- 零改：提交 INSERT 的 appealed 字面量、两审核端点 SQL、后端路由、前端其他函数/V2/改状态端点均未改。

## 一、根因（立项坐实）
- `submit_appeal`（lib.rs:493）写 appeal_status='appealed'；`audit_appeal`（lib.rs:539）与 V2 `attendanceappealInfo_audit`（assemble_control/src/lib.rs:736）只写 audit_status。
- `list_appeal_records`（lib.rs:415）旧版只 SELECT appeal_status，审核终态不反映；前端 appealClass/appealStatus 只认三值域、审批门 v-if 仅 pending，徽标恒 —、按钮恒不可达。

## 二、修复（写域精确）
- 后端 lib.rs:415：SELECT 状态列改为 COALESCE(NULLIF(audit_status,''), appeal_status) AS appeal_status，使 legacy 与 V2 两审核路径写的 audit_status 反映到列表；列由 migration 088 第12行 ADD COLUMN IF NOT EXISTS audit_status TEXT 提供，仅改 SELECT 表达式。
- 前端 AttendanceApp.vue:406 appealClass 增 appealed→pending；:409 appealStatus 增 appealed→待审核；:189 审批门由 a.status==='pending' 改为 pending || appealed。
- 契约测试：后端 tests/r597b_appeal_status_fix.rs（源码守卫 + DB-gated 真 handler 三 case）；前端 tests/contracts/appeal-status-fix.test.ts（四值映射 + 审批门）。

## 三、RED→GREEN
- RED（未改源码）：后端源码守卫 panic「list SELECT must COALESCE audit_status/appeal_status」；DB case 无 DB 时 SKIP；前端两测试失败（appealStatus('appealed') 得 —、门不含 appealed）。
- GREEN（改后）：后端 2 passed（源码守卫 ok + DB case ok）；前端 2 passed。

## 四、真 DB 三 case（临时 PG，占位 <TEST_HOST>:<PORT>）
| case | seed appeal_status | seed audit_status | 列表 status | 判定 |
|---|---|---|---|---|
| A legacy 审核 | appealed | approved | approved | PASS |
| B V2 审核 | appealed | rejected | rejected | PASS |
| C 无审核回退 | approved | NULL | approved | PASS |
- DB 直查 DIAG 坐实 seed 原值正确；cargo clean -p attendance 后重编译，COALESCE 生效。

## 五、三 verify
- cargo test -p attendance（带真 DB）：10 passed / 0 failed（8 unit + 2 契约）。
- cd oa4rust && pnpm typecheck：6 projects Done，EXIT 0。
- cargo fmt --check -p attendance：无 diff，EXIT 0。

## 六、据实披露与去伪
- 本方案不改写入路径：V2 审核与改状态端点仍写各自独立列；仅在列表读侧以 COALESCE 覆盖，使审核终态可见。写侧统一另案治理。
- Fail Loud：真 DB 三 case 首跑因 cargo 编译缓存未刷新（源码守卫读文本总最新、handler 为旧二进制）一度误判 CASE_A=appealed；cargo clean -p attendance 重编译后三 case 全过，已查清非逻辑缺陷。
- 并行干扰：工作树内 calendar 相关文件由他组成员并行修改，非本任务写域，commit 精确排除。
- 数据可见性：前端 appealed 映射为「待审核」而非「待审批」，与 pending 区分；审批门两值均可操作。

## 七、sha256 证据表（runner 紧凑 JSON 与文件 sha256sum 分列）
- runnerSha（sha256(JSON.stringify(meta))）：`bc0323598beea9acd52ab529a94d089dfc8590351d967219ad42ddee0d2e038c`
- 文件 sha256sum：
  - crates/attendance/src/lib.rs：`a35aaa5adbfffa941e79a0133638f3681919dcb83237690c3bf5c4b149621fee`
  - frontend/apps/desktop/src/views/AttendanceApp.vue：`938c0839b705345ed41039f9794bfbd833b868320b735176e9bb452e5d017d21`
  - crates/attendance/tests/r597b_appeal_status_fix.rs：`6aede70e1ce7de444f4e4c5ec8529984685a6024f58d2b2ef7ad30c84397d2a1`
  - tests/contracts/appeal-status-fix.test.ts：`32197c17a01e9ccbb9568595ad379fbc6ab799ee36e622fcc90f755a02fc6fee`
  - migrations/088_attendance_endpoint_status_columns.sql：`919b0aab2072206b13600e3d1b3b3fbd2b4af236d6e5d9305abff9f6821adb8e`
- 脱敏：DB 地址/端口/账号/口令仅 <TEST_HOST>:<PORT> 占位，0 真值入库。
