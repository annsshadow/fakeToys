# R604a 申诉状态机三端点无状态谓词审计（验证型甄别）

- 结论：P1。v2 申诉的 start/process、reset/status、end/process 三个写端点只有归属门、无当前状态前置谓词，owner 可把进行中(2)申诉重启为(1)/回退为(0)，更可把未发起(0)申诉直接写成通过(2)，绕过审批、篡改申诉结论，破坏考勤数据一致性。
- 真父 HEAD：a088b980da8e64eaf95bc36ef3e2a0d2a343643a（当前 HEAD 已前移一个无关文档提交 de8b1736；真父..HEAD 对本文全部锚点源码 git diff 零命中，静态锚点仍为真父快照）。
- 零源码改；唯一产出：本文档。测试夹具与日志留 gitignored 证据目录，用后即删。

## 立项与范围
- 台账 docs/ops/current-state.md 检索 R604a / 申诉状态机 / v2_appeal / 无状态谓词 零命中，本缺陷从未专项甄别。
- 三端点同文件 crates/attendance_assemble_control/src/lib.rs：v2_appeal_start_process(:7814-7853)、v2_appeal_reset_status(:7856-7888)、v2_appeal_end_process(:7891-7932)。
- 路由 crates/attendance_assemble_control/src/routes.rs:285-287 真实注册：POST .../v2/appeal/{id}/start/process、GET .../reset/status、POST .../end/process。
- 三端点属同一缺陷（审批状态机缺前置谓词），合并为 1 项，不硬凑独立项。

## 机制锚点（真父源码）
- 归属门 appeal_owner_gate(:7713-7735) 只校验 creator_person 归属(shared::middleware::require_owner)，不校验 status；v2_appeal_start_check(:7770-7811) 用 user_id 判 owner(:7791-7794 owner!=session.person_unique→403)，:7796-7797 let status:i32=...unwrap_or(0); let can_start=status==0;:7803-7806 can_start=false 时 message=申诉数据已进入流程，不能重复发起——设计意图明确不可重复发起。
- 非法1 start/process：SQL :7833 UPDATE x_attendance_v2_appeal_info SET status = $2, job_id = $3, update_time = NOW() WHERE id = $1，无 AND status=0，与 start/check 意图直接矛盾。
- 非法2 reset/status：SQL :7869 SET status = $2, job_id = NULL, send_status = FALSE, update_time = NOW() WHERE id = $1，无当前状态前置。
- 非法3 end/process：:7903-7908 入参 end_status 仅校验 ∈{2,3}，SQL :7913 SET status = $2, job_id = COALESCE($3, job_id), update_time = NOW() WHERE id = $1，无须处于流程中前置——未发起(0)可直接写通过(2)。
- 兄弟铁证：crates/mind_assemble_control/src/u2.rs:108-119 UPDATE x_mind SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL 且 n==0 返回已删除，证明写操作带状态谓词是既有惯例。
- DDL migration 076_attendance_u2_closure.sql:178-193 x_attendance_v2_appeal_info(id TEXT PK,record_id,user_id,status INTEGER DEFAULT 0,job_id,send_status BOOLEAN,creator_person,create_time,update_time)；is_admin(rbac.rs:76-99)、require_owner(rbac.rs:582-590) 只管人不管状态。

## 方法与决定性实测（临时 PG + 真迁移 + router oneshot）
- 起独立临时 PostgreSQL（主机/端口/账号/口令均占位 <TEST_HOST>:<PORT> / <USER> / <PASS>，真值由环境变量注入、不入库），shared::migrate::run_migrations 建真表。
- seed 3 条 alice 申诉：appeal-a/b status=2、appeal-c status=0；注入 alice/bob Session 经真 router + oneshot 直调三端点，另测 start/check 意图与 bob 归属门。

| 断言 | 实测 | 结论 |
|---|---|---|
| start/check(appeal-a, status=2) | data.value=false | 设计意图：不可重复发起 |
| status=2 → start/process | 200, status=1 | 进行中申诉被重启 |
| status=2 → reset/status | 200, status=0 | 进行中申诉被回退 |
| status=0 → end/process{status:2} | 200, status=2 | 跳过审批直接通过 |
| bob → alice start/process | 403 | 归属门存在，非跨用户 |
| 合计 | 2 passed / 0 failed | 三非法跃迁全部坐实 |

- 决定性日志（gitignored，用后即删）：test-results/r604a-run.log（10 行），字节 sha256=1a07acc73c071f953da552b41f2f08403c1a2f250f7c928e77497073f0d50f1c。
- 夹具（gitignored，用后即删，93 行）：tests/r604a_appeal_state_machine_audit.rs，字节 sha256=063a6c3db221a89bc2ba7ee6b549f705ec50fd5345bca3a1e2505f69728b4439。

## 三分裁决（据实，不夸大不降级）
1. 核心危害：审批状态机可被 owner 自行绕过——未发起(0)直接写通过(2)、进行中(2)可重启/回退，申诉结论可被当事人篡改，破坏考勤审批完整性与下游数据一致性，故判 P1。
2. 据实披露攻击前提（不夸大）：需有效会话、需是申诉 owner(creator_person=user_id) 或 admin、需知道 appeal id；非匿名、非跨用户越权(bob 403 坐实)，故不表述为无差别越权。
3. 非 L1：三端点真实可达、真测 200 坐实，非不可达缺陷。
4. 三端点合并 1 项：同属审批状态机缺前置谓词，不硬凑独立项。
5. 与 R603a 区分：R603a 为催办/加压任务状态(P2)，本项为申诉审批结论状态，直接影响审批结果与数据一致性，严重性更高。

## Fail Loud
- 队长初判 P1，据实分析显示攻击前提与 R603a(P2) 同类(需鉴权、需知 ID、非跨用户)；最终按队长裁定 P1 并完整披露上述限制条件，如队长改判 P2 需同步本文结论。
- 当前 HEAD 比指定真父多一个无关文档提交 de8b1736，已核实对全部锚点源码零 diff；如有人在此期间改动上述源码需重跑。
- 非回环端口/账号/口令均为临时真值，未写入任何入库文件。

## SHA 清单（runner 紧凑 JSON 哈希与文件字节哈希分列）
- runnerSha=sha256(JSON.stringify(meta))=e6da9227794fc95eddae78dffb279bc4e69f49abf6e247d5c2a8607daf56c2f4
- 证据日志字节 sha256=1a07acc73c071f953da552b41f2f08403c1a2f250f7c928e77497073f0d50f1c
- 夹具字节 sha256=063a6c3db221a89bc2ba7ee6b549f705ec50fd5345bca3a1e2505f69728b4439
- crates/attendance_assemble_control/src/lib.rs=e59e9b38f22152ef95959fbb08d11eecdcdae0378ef5b355189fac95a9c78491
- crates/attendance_assemble_control/src/routes.rs=d829b2f6e01ffac6b900cb59882d0d995104cb799f191cb9623ebd1741e1c0b9
- crates/mind_assemble_control/src/u2.rs=22e6afe3f3ea012da2396360b802b3ebc86cf699613861a3b03ba23aa9770523
- crates/shared/src/middleware/rbac.rs=5d6f6b64efc59739e87dc8fd37ba3848eff651b838b3d3679d73f60145716d6a
- migrations/076_attendance_u2_closure.sql=67ac02898cc4a7fb74418de605079a3f64c6793e6b6eac6ea457669f5d731731

## 修复方向
- 三端点加当前状态前置谓词并入 SQL WHERE（参照 u2.rs 带 AND deleted_at IS NULL 与 n==0 判空）：start/process 仅允许 status=0；reset/status 仅允许流程中态；end/process 仅允许流程中态，非法态返回明确错误。
- 让 start/check 的 can_start 意图真正约束 start/process，并补状态机迁移测试覆盖非法跃迁被拒。