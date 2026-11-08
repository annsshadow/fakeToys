# 第八轮：安全回归门禁（代码交付）

基线：HEAD `f4c26455e` + 修复线未提交改动（工作树）。

本轮的产出是代码，不是报告。本文只回答三件事：**怎么跑、基线怎么改、两个破坏实验原文**。

## 一、交付物

| 文件 | 作用 |
|---|---|
| `oa4rust/backend/tests/security_regression.rs` | 门禁主体（148 条逐条断言 + 棘轮比对 + scratch 库自建自删） |
| `oa4rust/backend/tests/security_regression_cases.tsv` | 用例表：`id / kind / method / path / table / victim / leak` |
| `oa4rust/backend/tests/security_regression_seed.sql` | 受害者行造数（82 张表，id 取自第六轮实测行 rid） |
| `oa4rust/backend/tests/security_regression_fixtures.sql` | 身份、cms 文档、空 creator 的 x_work、empower 授权行 |
| `oa4rust/backend/tests/security_regression_baseline.txt` | 棘轮基线（当前 142 条非 PASS） |
| `oa4rust/backend/scripts/security_regression_gate.py` | 一条命令的 runner（含两个破坏实验） |

`.gitignore` 已加两条 `!` 例外（`scripts/**` 与 `*.txt` 默认忽略）：脚本与基线必须入库，否则干净克隆上无处可跑/棘轮失效。

## 二、怎么跑

```bash
# 在 oa4rust/backend 下
python scripts/security_regression_gate.py
# 或直接
cargo test --test security_regression -- --nocapture
```

需要真实 PostgreSQL（默认 `postgres://o2server:password@localhost:5432/oa4rust_test`，可用 `DATABASE_URL` 覆盖）。
**不依赖任何预置测试库**：门禁自行 `DROP/CREATE oa_par_gate` → 跑全量迁移 → 灌最小数据 → 148 条逐条请求 → `DROP DATABASE`。
每次跑完都会清理（已实测：跑完后 `pg_database` 中无 `oa_par_gate`）。

输出逐条一行，失败可直接读到端点、身份、预期：

```text
[LEAK] attendance_assemble_control|attendanceadmin_id|GET | GET /api/attendance/assemble/control/attendanceadmin/{id} | ident=audit-user | expect=deny | 2xx 响应体含 canary `r6-attendanceadmin_id`
[UNVERIFIED] program_center|collect_save|PUT | PUT /api/program_center/collect/save/{id} | ident=audit-user | expect=deny | 2xx 但行未变
[PASS] attendance_core_entity|record_update|POST | POST /api/attendance/core/entity/record/{id}/update | ident=audit-user | expect=deny | denied
```

判定口径（刻意的严格）：

| 结果 | 含义 |
|---|---|
| PASS | 401/403 明确拒绝；软门还要求「拒绝语义 + 零写入」 |
| LEAK | 2xx 且响应体含受害者 canary，或写用例 2xx 且受害者行被改/被删 |
| UNVERIFIED | 404/5xx/种子缺失/2xx 无 canary —— **不算 PASS**，计入基线 |

「2xx 无 canary」不判 PASS，是因为「加了过滤器返回空」和「种子漂移导致查不到」在响应上不可区分；宁可红，不假绿。

## 三、基线怎么改

规则是**集合相等**，不只是「数量不增」：

- 出现基线之外的非 PASS（新 LEAK / 新 UNVERIFIED）→ 红，打印 `NEW-FAIL <用例 id>`；
- 基线内的用例转为 PASS（修了却没减基线）→ 红，打印 `STALE-BASELINE <用例 id>`。

后者保证「单调下降」：修复线每修一条，必须删掉基线里对应那一行，否则门禁红。

```bash
# 修复落地后：重写基线（会把当前非 PASS 全量落盘）
python scripts/security_regression_gate.py baseline
# 等价的直接命令
SECURITY_GATE_UPDATE_BASELINE=1 cargo test --test security_regression -- --nocapture
```

改完把 diff 一起提交；基线文件里每行就是「用例 id \t 状态」，删行即减一。

### 当前读数（本工作树）

```text
cases=148 pass=6 leak=131 unverified=11
门禁通过：非 PASS 142 条，与基线一致
```

148 = 141 条第六轮实测越权 + 3 条 cms 假门 + 2 处 fail-open + 2 条软门。

- IDOR 117 读：115 LEAK、2 UNVERIFIED
- IDOR 24 写：14 LEAK、9 UNVERIFIED、1 PASS
- cms 假门 3：2 LEAK（`allowRead:true`、cipher 端点信任路径上的 `person`）、1 PASS（draft 文档 `readable:false`）
- fail-open 2：**当前 PASS**（修复线在本次会话期间提交了 `work_terminate` 与 `u4_require_actor` 两处修复，工作树可见）
- 软门 2：PASS（HTTP 200 但 `forbidden` 且零写入）

基线从 144 降到 142，正是 fail-open 两条转 PASS 后按棘轮规则减一 —— 这条 STALE 报警就是棘轮生效的实证。

## 四、两个破坏实验（原文）

两条都可以重跑；脚本在 `finally` 里按**原始字节**还原业务文件（已复核 `git status`，业务代码无残留改动）。

### 实验①：把已修的归属条件摘掉

```bash
python scripts/security_regression_gate.py sabotage ownership
```

做法：把 `attendance_core_entity` 的 `RECORD_OWNER_PREDICATE` 换成 `id = $1 AND $2 IS NOT NULL`（即只校验行存在，不校验属主）。

```text
== 破坏实验：摘掉考勤记录的归属谓词（record_update 原本已修，应变 NEW-FAIL）
[LEAK] attendance_core_entity|record_update|POST | POST /api/attendance/core/entity/record/{id}/update | ident=audit-user | expect=deny | 2xx 且受害者行被改/被删
基线 142 条 / 本次非 PASS 143 条
NEW-FAIL	attendance_core_entity|record_update|POST
实验成立：门禁变红（exit=101），并点名 attendance_core_entity|record_update|POST
[restore] lib.rs 已还原
```

### 实验②：把一条本来正确的端点改坏

```bash
python scripts/security_regression_gate.py sabotage correct
```

做法：把 `document_permission_read_u3` 的 `readable` 从「admin / 属主 / published」改为「admin 或 creator 列存在」，让非属主也能拿到 `readable:true`。

```text
== 破坏实验：把本来正确返回 readable=false 的文档读权限端点改成恒 true（应变 NEW-FAIL）
[LEAK] gate-cms-readable | GET /api/document/{id}/permission/read | ident=audit-user | expect=deny | 2xx 响应体含 canary `"readable":true`
基线 142 条 / 本次非 PASS 143 条
NEW-FAIL	gate-cms-readable
实验成立：门禁变红（exit=101），并点名 gate-cms-readable
[restore] lib.rs 已还原
```

两次实验都满足「新增非 PASS → 点名具体用例 → exit≠0」，且还原后门禁恢复绿（`非 PASS 142 条，与基线一致`）。

## 五、已知局限（修复线需要知道）

1. **11 条 UNVERIFIED 不在「已修」语义内**：多数是写端点 2xx 但行未变（handler 走了别的表/需要附件），以及 2 条读端点 200 无 canary。它们计入基线，不会被误当成修复。
2. **种子来自第六轮实测行 rid**，不是为门禁重新设计的业务数据；handler 若改变查询键，会从 LEAK 掉到 UNVERIFIED（门禁会红，提示种子漂移，而不是假绿）。
3. **身份矩阵只有 attacker=victim 之外的单一非管理员**；「A 看不到 B」的第二方向（属主互查）未纳入本轮 148 条，沿用第六轮的双向验证结论。
4. **工作树并发修改**：门禁读的是当前工作树。修复线提交后需重跑 `baseline` 重写基线，否则新修复会触发 STALE 红（这是设计行为，不是误报）。
