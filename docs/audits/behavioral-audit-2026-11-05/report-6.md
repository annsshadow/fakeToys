# 第六轮行为审计：path-IDOR 全量 / 空归属放行扫描 / 读面双向

审计目录：`docs/audits/behavioral-audit-2026-11-05/`
证据原文：`evidence-round6.txt`　造数脚本：`seed-round6-idor.sql`、`seed-round6-pp.sql`
基线：**读码与实测时 HEAD = `4b4e17010`**；实测窗口 2026-11-06 00:56–01:35（Asia/Shanghai）。
落盘时仓库 HEAD 已漂移到 `c0622c5da`，中间 5 个提交逐文件核对：
- `b4aed34c5` 维护作业业务日界换算 UTC（`processplatform_service_processing/src/u2.rs`）—— `git diff` 检索 `require_actor|require_owner|is_admin` **零命中**；
- `27ba0d6fd` 合并 PR #186（同上）；
- `58eb6c366` 考勤 401 断言配套反证（测试文件）；
- `b169523f0`、`c0622c5da` 只改 `docs/plans/f2e-endpoint-tracker.md`。

即：**本轮所有端点与归属 helper 均未被漂移触及**，证据继续有效；`u4_require_actor` 仍在 `lib.rs:422`、`u4_require_actor_of` 仍在 `u2.rs:104`。
工作树仅 `?? docs/audits/behavioral-audit-2026-11-05/`。
环境：本机 `127.0.0.1:3999`（Rust），scratch 库 `oa_audit_20261105`（容器 `lyp-pg`，UTC）。
身份：攻击者 `audit-user`（`audit-u1`）、第二普通用户 `audit-user2`（`audit-u2`）、受害者/管理员 `person-admin`、匿名（无 cookie）。

---

## 零、结论先行

| 指标 | 数值 |
|---|---:|
| 含 `{id}` 的路由条目 / 唯一 handler | **1559 / 1115** |
| 「路径带 id、直接操作该行」的 handler | **421** |
| 其中静态有归属门 | **176** |
| 其中静态无归属门（逐个实测） | **245**（写 65 / 读 180） |
| **实测越权成立** | **141**（写 24 / 读 117） |
| 空归属放行（fail-open）真例 | **2 处**（`u4_require_actor` 16 调用点 + `work_terminate` 内联） |
| 双向交叉读 | 一级读端点 2 条 **双向都泄漏**；不再有「单向」不对称 |

三句话：

1. **path-IDOR 是当前最大的一类洞**：只要登录，按 ID 就能读 117 类他人数据、改/删 24 类他人数据；覆盖面远超第五轮点名的 125 条（那 125 条只覆盖「请求体可改敏感字段」，不含 `delete_*`/`update_*`/`{id}` 形态）。
2. **`u4_require_actor` 不是孤例的判断反转，但同形态真例只有 1 个内联副本**；全仓 750 个带归属语义的函数里，87 个命中「空/缺失 → 继续执行」的语法形态，逐个读体后只剩 `u4_require_actor` 与 `processplatform_core_express::work_terminate` 两处真放行。
3. **读面双向验证的结论不是「A 看不到 B」**：A 能读 B、B 也能读 A，两个方向都成立；匿名全 401、管理员 200，说明这不是「整体不可读」，而是**归属谓词根本没生效**。

---

## 一、方法与判据

### 1.1 分母怎么来的

1. 从 `routes2.json`（3106 个 fn key）展开全部路由条目：**4136 条**；
2. 筛 `path` 含 `{id}`：**1559 条 / 1115 个唯一 (crate, fn)**；
3. 读 handler 源码，判定「该 handler 是否用这个 id 直接读/写目标行」（排除只拼 URL、只做过滤、id 传给下游不改行的）：**421 个**；
4. 静态门禁识别正则：`require_owner` / `is_admin` / `owner_gate` / `require_actor` / `require_row_owner` / `u2_gate_by_sql` / `guard_write` / `crud_owner_ok` / `check_owner` 等；
5. 无门 245 条逐条动态请求（造受害者行，owner=`person-admin`），按响应与库内复核判定。

**残留分母缺口（主动标注）**：本口径不含「id 在查询串或请求体、handler 再按该 id 操作行」的端点（第五轮 125 条正是这一类），也不含 Go 侧。两类并集才是完整 IDOR 面。

### 1.2 判定口径

| 结论 | 判据 |
|---|---|
| `EXPLOIT`（读） | 200 且响应体含受害者行主键/内容，且该行 owner ≠ 攻击者 |
| `EXPLOIT-CHANGED`（写） | 200 且库内关键列 before/after 复核已变 |
| `EXPLOIT-REACHABLE`（写） | 200 且已到达写 SQL（返回 success/updated/deleted），未逐列复核 |
| `GATED` | 401/403 |
| `NO-OWNER-COL` | 目标表无 creator/owner/person 列，「非本人」无定义 |
| `REACHED-ERROR` / `FAIL-500` / `FAIL-404` / `EMPTY` | 到达 handler 但行/文件不存在、缺必填列、响应为 error —— 无越权后果 |
| `SKIP-NO-TABLE` | handler 走 `PP_C_*`/share 等另一套表名，静态映射未覆盖 |

### 1.3 造数

- `seed-round6-idor.sql`：**82 张表**的受害者行，固定 id `r6v-<表名>`（uuid 表用 `md5(...)::uuid` 派生稳定值），owner 列统一 `person-admin`，先 DELETE 后 INSERT，**可重复执行**。
- `seed-round6-pp.sql`：`x_work`/`x_task` 空 `creator`/`person` 行、`x_timer_job`、考勤两条明细、第二普通用户 `audit-user2`。
- 117 条读越权的受害者行 owner 分布：`person-admin` 112、`admin` 3、空 2（见 §3.2）。写入侧的受害者行统一 `person-admin`。

---

## 二、四档统计

| 档 | 数量 | 动态复核结果 |
|---|---:|---|
| ① 总条数（直接按 id 操作行） | **421** | — |
| ② 有归属判断 | **176** | 403 = 145；200 但仅权限探测 = 3；无后果/无表 = 28 |
| ③ 无归属判断 | **245** | 越权成立 **141**；门禁实际生效 11；无后果 53；无 owner 列 31；无表映射 9 |
| ④ 实测越权成立 | **141** | 写 24 / 读 117 |

**② 桶必须动态复核，否则会高估安全性**：176 条里有 **3 条 200**（`cms` 的 `document_control_u3`、`document_permission_read_u3`、`document_cipher_permission_read_u3`）。静态扫到 `is_admin` 就记为「有门」，但这三个 handler 里的 `is_admin` 只用于**计算返回的布尔值**，不是拦截：

- `document_control_u3`（`cms_assemble_control/src/lib.rs:11144`）：任何登录用户按 ID 都返回 `{id, allowRead:true, allowEdit, allowManage, controller}`，`controller` 直接暴露文档创建人；
- 另两条返回 `{id, readable}`，且 `document_cipher_permission_read_u3` 的 `person` 取自**路径参数**，等于允许「查询任意人能否读某文档」的权限探测。

它们不返回正文，故不计入 141，但属于**假门**（`is_admin` 在场、门禁不在场），修的时候不能因为「扫到 is_admin」就放过。

---

## 三、无归属判断 245 条逐条实测

### 3.1 写（65 条）

| 结论 | 条数 | 说明 |
|---|---:|---|
| `EXPLOIT-CHANGED`（已复核落库） | **10** | 见下 |
| `EXPLOIT-REACHABLE`（200 到达写） | **14** | 见下 |
| `NO-OWNER-COL` | 13 | 字典/配置/组织 bind，自定义「本人」 |
| `REACHED-ERROR` | 11 | message/mind 等：行不存在或参数缺失 |
| `FAIL-500` | 9 | bbs 写 6（造数不满足约束，第五轮已用合法行实证）+ query import 3 |
| `GATED` | 8 | 静态漏判，运行时 403（org person/group、cms publish） |

**已复核落库的 13 条**（10 条由自动 md5 判定为 `EXPLOIT-CHANGED`；另 3 条自动判为 `REACHABLE`，经 §E 人工复核确认列已变）：

| 端点 | 文件:行 | 复核结果 |
|---|---|---|
| `POST /api/general/invoice/update/{id}` | `general_core_entity/src/lib.rs:823` | `number=r6seed→R6-HACKED`、`amount=0→99999`、`status=approved` |
| `POST /api/general/invoice/delete/{id}` | `general_core_entity/src/lib.rs:893` | 行物理消失 |
| `POST /api/general/file/update/{id}` | `general_core_entity/src/lib.rs:589` | `name=r6seed→R6-HACKED` |
| `POST /api/general/file/delete/{id}` | `general_core_entity/src/lib.rs:644` | 行物理消失 |
| `PUT /api/organization/definition/{id}` | `organization_core_entity/src/lib.rs:396` | `name=r6seed→R6-HACKED` |
| `DELETE /api/organization/definition/{id}` | `organization_core_entity/src/lib.rs:436` | 软删（`deleted_at` 置位） |
| `PUT /api/organization/identity/{id}` | `organization_core_entity/src/lib.rs:593` | `name=r6seed→R6HACK` |
| `DELETE /api/organization/identity/{id}` | `organization_core_entity/src/lib.rs:633` | 软删 |
| `POST /api/mind/core/entity/folder/{id}` | `mind_core_entity/src/lib.rs:369` | `name=r6seed→R6-HACKED` |
| `POST .../attendanceappealInfo/appeal/{id}` | `attendance_assemble_control/src/lib.rs:443` | `appeal_status=pending→appealed`、`updated=1` |
| `POST .../record/{id}/update` | `attendance_core_entity/src/lib.rs:162` | `status=r6seed→R6HACK`、`check_in_time` 被改（`user_id=person-admin` 不变） |
| `DELETE /api/file/core/entity/folder/{id}` | `file_core_entity/src/lib.rs:350` | `deleted_at` 置位 |
| `DELETE /api/hotpic/core/entity/delete/{id}` | `hotpic_core_entity/src/lib.rs:222` | `deleted_at` 置位 |

（表内 13 行 = 10 条自动 `CHANGED` + `definition_update`、`identity_update`、`record_update` 3 条人工补验；`record_update` 的正确路径形态是 `POST .../record/{id}/update`，首次用错路径得到 404 后已重测。）

**`EXPLOIT-REACHABLE` 14 条**：`hotpic` 批量删、`program_center` collect/module 4 条、`cms` icon/projection/array-data 3 条、`mind update_mind`、`processplatform` attachment edit / task will 2 条、`organization` definition_update / identity_update、`attendance` record_update。其中后 3 条已人工复核确认落库（上表），其余 11 条仅确认「已到达写 SQL」，未逐列复核。

### 3.2 读（180 条）

| 结论 | 条数 |
|---|---:|
| `EXPLOIT`（返回他人行） | **117** |
| `NO-OWNER-COL` | 18 |
| `FAIL-500` | 17（portal/query widget、message 结构化列缺失） |
| `EMPTY`（200 但 error 体） | 13 |
| `SKIP-NO-TABLE` | 9 |
| `GATED` | 3 |
| `FAIL-404` | 3 |

**117 条按 crate**（完整逐条见 `evidence-round6.txt` §B）：

| crate | 条数 | 代表端点 |
|---|---:|---|
| `cms_assemble_control` | 18 | `/api/document/{id}/view`、`/api/file/{id}/content`、`/api/file/{id}/download`、`/api/document/{id}/persons`、`/api/comment/{id}` |
| `attendance_assemble_control` | 15 | `/attendancedetail/{id}`、`/attendancedetail/mobile/{id}`、`/attendanceappealInfo/{id}`、`/v2/appeal/{id}`、`/v2/record/{id}` + 10 条考勤配置 |
| `processplatform_service_processing` | 12 | `/task/{id}`、`/task/v2/{id}`、`/workcompleted/{id}`、`/review/{id}`、`/snap/{id}`、`/record/{id}` |
| `query_assemble_designer` | 9 | `/get/{id}`、`/view/permission/{id}`、`/stat/simulate/{id}`、`/view/{view}/{id}` bundle |
| `program_center` | 8 | `/application/{id}`、`/module/output/{id}/file`、`/dict/{id}`、错误日志 |
| `portal_assemble_designer` | 7 | `/get/{id}`、`/portal/icon/{id}`、`/file/download/{id}`、`/scriptversion/{id}` |
| `general_assemble_control` | 5 | `/invoice/get/{id}`、`/securityclearance/{id}`、`/qrcode/{id}` |
| `portal_assemble_surface` | 5 | `/get/{id}`、`/preview/{id}`、`/portal/icon/base64/{id}` |
| `query_assemble_surface` | 5 | `/execute/{view}/{id}`、`/excel/{view}/{id}`、`/bundle/{view}/{id}` |
| 其余 26 crate | 33 | org identity、meeting、mind、jpush device、calendar、message mass、general invoice/file 等 |

**读越权最重的几条**（P0 排序依据）：

1. 考勤明细 / 申诉 / 记录：`attendancedetail_id:1944`、`attendanceappealInfo_id:955`、`v2_record_get:6220` —— 与第五轮 `mobilepreview`/`mobile/my` 同根，**按 ID 单条读**补齐了同类缺口。
2. 组织身份：`organization_assemble_control /api/identity/{id}`、`organization_assemble_authentication /identity/{id}` —— 返回姓名/身份/PII。
3. 文档与文件内容：`cms_assemble_control /api/file/{id}/content`、`/api/file/{id}/download`、`/api/document/{id}/view`、`/api/document/{id}/persons` —— 正文/附件/PII 可读；其中 `anonymous_document_id_view:657` 对**登录用户**可读他人 `draft`（与台账 E13 同一根）。
4. 财务：`general_assemble_control /invoice/get/{id}`、`general_core_entity /invoice/{id}`，与写侧 `invoice_update/delete` 成对。
5. 流程数据：`processplatform_core_express` 的 `work_processing:91`、`task_processing:125`、`work_retract:58` —— **这三个 handler 连 session 都不取**（见 §5 未闭环）。

### 3.3 不可判 + 无归属列

- **`NO-OWNER-COL` 31 条**（写 13 / 读 18）：`x_attendance_rule`、`x_cms_category`、`x_general_application_dict(_item)`、`x_org_bind`、`x_org_custom`、`x_meeting_room/building`、`x_jpush_template`、`x_process_task` 等。表本身没有「本人」概念，属全局配置/组织绑定；是否应限定写权限需业务裁决，本轮**不计入越权**。
- **`SKIP-NO-TABLE` 9 条**：processplatform 的 attachment/snap 走 `PP_C_*` 与 `share` 表，静态映射未覆盖，**未判**（§7）。
- **`FAIL-500` 26 条**：bbs 写 6 条已在第五轮用合法行实证同类越权；其余为 widget/结构化列缺失，属**环境不可判**，不是「已证明安全」。
- **`EMPTY` 13 条**：200 但响应为 `{"type":"error","message":"not found"}`。file 族出现「行在库、handler 报 file not found」（可能按 `xid`/`file_status` 二次过滤），需人工确认不是漏判。

---

## 四、空归属放行（fail-open）全仓扫描

### 4.1 扫描口径与收敛

解析全部 crate 的 `fn` 定义体（跳过 test 文件），遍历 **750 个**与归属语义（session/owner/actor/person/creator/reviewer/admin/user_id/author）相关的函数，匹配三类「缺失归属 → 继续执行」形态：

1. `if <x>.is_empty() { return Ok(...) }`；
2. `match ... { None => Ok(...) }` 且 `Ok` 不是 error/401/403；
3. `if let Some(x) = x.filter(|v| !v.is_empty())` —— 空值整段跳过门禁。

命中语法形态的候选 **87 个**。逐个读体后，分诊结果为（三类存在交叠，故不并列求和）：

| 分类 | 数量 | 代表 |
|---|---:|---|
| **真·放行（fail-open）** | **2** | `u4_require_actor`、`work_terminate` 内联 |
| 严格（空 → 仅 admin / 403） | 21 | `require_actor_opt`、`u4_require_actor_of`、`u4_require_work_owner`、`u4_require_actors`、`require_row_owner`、`guard_write`×3、`u2_gate_by_sql`×3、`crud_owner_ok`、`owner_gate`、`mind_owner_gate`、`meeting_require_owner`、`appeal_owner_gate`、`jpush_owner_gate`、`u2_meeting_owner_gate`、`require_file_read_owner`、`check_owner`、`u2_invoice_check_owner` |
| 其中额外容忍「行不存在」（后续 SQL 0 行，无副作用） | 6 | `general owner_gate`、`mind_owner_gate`、`meeting_require_owner`、`u2_meeting_owner_gate`、`guard_write`×2、`gate_appdict_writer` |
| 输入校验 / 登录流程（误命中） | 58 | `sso_login`、`mpweixin_login`、`whoami`、`from_env`、`get_ai_control_config` 等 —— 空值表示「参数缺失」，返回错误而非授权放行 |

### 4.2 `u4_require_actor`（`processplatform_service_processing/src/lib.rs:422`）

```rust
if actor.trim().is_empty() { return Ok(()); }   // 空 actor 直接放行
shared::middleware::require_owner(pool, session, actor).await
```

简单调用 `require_owner` 的逻辑本身是 fail-closed（空 owner vs 非空 person → 403）；问题在**前面这个短路**，把「拿不到归属」解释成了「无需校验」。

**16 个调用点：14 个注册了 HTTP 路由，逐个实测（空 creator/person 行 + 攻击者 `audit-user`）；另 2 个是内部辅助函数（无路由），静态同门**：

| 调用点 | handler | 空归属实测 | 库内变化 |
|---|---|---|---|
| `:233` | `execute_process` | **200** | 流程被推进（未逐列复核） |
| `:383` | `cancel_process_instance` | **200** | 取消流程（状态推进，未逐列复核） |
| `:549` | `work_id_processing` (PUT) | **200** | pending→processing |
| `:583` | `work_v2_id_terminate` | **200** | →terminated，`end_time` 写入 |
| `:628` | `work_v2_id_retract` | **200** | →retracted |
| `:2094` | `review_init_review`（内部辅助，无路由） | 静态命中 `u4_require_actor`，归入 16 调用点 | — |
| `:2843` | `service_work_id_touch_core`（内部辅助，无路由） | 静态命中，归入 16 调用点 | — |
| `:3434` | `work_v3_retract` | **200** | 撤回（状态推进，未逐列复核） |
| `:3479` | `work_start` | **200** | pending→processing |
| `:3538` | `work_complete` | 500（副作用已发生：task→active） | 状态机自身报错，门已过 |
| `:3600` | `task_claim` | **200** | pending→active |
| `:3688` | `task_complete` | **200** | 完成（状态推进，未逐列复核） |
| `:3796` | `task_reject` | **200** | 驳回（状态推进，未逐列复核） |
| `:3878` | `task_transfer` | **200** | `person: NULL→person-admin` |
| `:3936` | `gateway_join` | **200** | 插 `x_record(gateway_join)`；`joined:"joined"` |
| `:4325` | `cancel_timer` | **200** | `cancelled_at` 写入 |

**对照组**：

- 同 crate `task_id_processing` 用严格门 `u4_require_actors` → **403**（证明收紧写法在同一个 crate 里已有）；
- `work_start(creator=person-admin)` / `task_claim(person=person-admin)` → **403**；
- 本轮新增的 `terminate` / `retract` 对照：`creator=person-admin` → **403**，且库未变（`work-own-1` 仍为 pending）；
- 匿名 → **401**（取 `work_start` / `task_claim` 对照；新增 3 条匿名请求在 harness 层异常 `-1`，未取到状态码，不计为结论）。

**诚实边界**：上述 200 全部建立在「行本身 creator/person 为空」的脏行上。生产是否有此类行需业务确认；但修法很便宜（空值也要求 admin，或显式白名单 `"system"`），不要留「空 = 放行」。

### 4.3 第二个实例：`work_terminate`（`processplatform_core_express/src/lib.rs:16`）

```rust
if let Some(creator) = creator.filter(|c| !c.is_empty()) {
    shared::middleware::require_owner(...).await?;   // 空 creator 时整段跳过
}
```

实测：

- `creator=NULL` 的 `work-null-pending`：`audit-user` → **200**，`work_status: pending→terminated`；
- `creator=person-admin` 对照：**403**，库未变。

它与 `u4_require_actor` 是同一判断反转的**内联副本**（不是 helper），台账 P0-b 只写了 helper 家族，这条要补进去。

### 4.4 全仓严格家族（供修复时复用，不要再新写门）

`require_actor_opt`（assemble_surface:42）、`u4_require_actor_of`（u2.rs:104）、`u4_require_work_owner`（:438）、`u4_require_actors`（:501）、`require_row_owner`（query_assemble_designer:30）、`guard_write`（designer:3653 / surface:1015 / designer u2:1422）、`u2_gate_by_sql`（bbs u2:70 / cms:8210 / pp surface:24773）、`crud_owner_ok`（shared/crud:170）、`owner_gate`（general:55）、`mind_owner_gate`（mind:33）、`meeting_require_owner`（meeting_core_entity:58）、`u2_meeting_owner_gate`（:2272）、`appeal_owner_gate`（attendance:7481）、`jpush_owner_gate`（:319）、`require_file_read_owner`（file:1227）、`check_owner`（empower:64）、`u2_invoice_check_owner`（pp surface:28700）。

统一口径：**非空归属 → 本人或 admin；空归属/行不存在 → 仅 admin**（`require_owner` 对 `owner=""` 与 `person≠""` 返回 403，天然 fail-closed）。

---

## 五、读面双向交叉读

一级 16 路由 / 13 fn 中，**读端点只有 2 个**：`attendancedetail/mobile/mobilepreview`、`attendancedetail/mobile/my`。本轮用两个普通用户 A（`audit-u1`）、B（`audit-u2`）双向测，并补测同族 by-id 读 `GET .../attendancedetail/mobile/{id}` 与写 `recive` 的读回。

| 场景 | A→B | B→A | 结论 |
|---|---|---|---|
| `mobilepreview`（body `personId`） | 200，返回 `r6-ad-b\|audit-u2\|LATE` | 200，返回 `r6-ad-a\|audit-u1\|NORMAL` | **双向泄漏** |
| `mobile/my`（body `personId`） | 200，返回 B 的明细数组 | 200，返回 A 的明细数组 | **双向泄漏** |
| `GET /mobile/{id}`（路径 id） | 200，返回 B 的行 | 200，返回 A 的行 | **双向泄漏** |
| `recive`（写） | 200，`r6-ad-b.received f→t` | 200，`r6-ad-a.received f→t` | **双向可写** |
| 匿名对照 | 401 | 401 | 会话门有效 |
| 管理员对照 | 200 | 200 | 不是「整体不可读」 |

**结论**：不存在「A 能看到 B、B 看不到 A」的单向不对称；两个方向都由同一缺陷产生 —— 归属谓词（`personId`/`Path(id)`）直接取自请求，未与会话比对。因此修复守卫必须做**双向用例**（A→B 与 B→A 都要红），只验单向会漏掉「反向被某个 admin 分支挡住」的假修复。

补充：`mobile/my` 用 GET + query 传 `personId` → **400**（handler 强制 JSON body），用 GET + body → 200。这解释了一个容易误判的现象：**同一端点的方法差异会改变「能不能打」**，复跑时必须固定方法/载荷形态，否则红绿不稳定。

---

## 六、与修复线（F2-E 台账）的关系

| 本轮发现 | 台账状态 | 说明 |
|---|---|---|
| `u4_require_actor` 空 actor 放行（16 调用点） | **已登记**：`P0-b`（提交 `c0622c5da`） | 台账原文：「空 actor 被放行；空值必须拒绝；全仓扫 `require_actor` 同形态」；本轮补的是逐调用点实测与影响面 |
| `work_terminate` 内联 fail-open | **未收录（新增）** | 建议并入 `P0-b`，作为第二个实例 |
| `record_update`（`/record/{id}/update`）越权改 | **已被覆盖**：`P0-2` | 本轮复核：200，`status`/`check_in_time` 已变 |
| `anonymous_document_id_view` 读他人 draft | **已被覆盖**：`E13` | 与 E13 同一根（登录用户可读非本人 draft） |
| `file` by-id 读/删 | **部分覆盖**：`L1-3`（仅 anonymous download） | 普通 `/api/file/{id}` 读、`folder_delete` 未入台账 |
| 流程状态机（`work_id_processing` 等直接改 status） | **已被覆盖**：`L1-5` | 本轮实测空 creator 可推进；非空 creator 403 |
| 24 条写 IDOR（invoice/file/org/mind/attendance） | **未收录（新增）** | 第五轮只点名 `invoice_update/delete`（报告 5.3 #5/#6），台账尚未建行 |
| 117 条读 IDOR | **大部分未收录（新增）** | 台账 E 系列是**列表**端点；按 ID 单条读是另一组，建议按 crate 拆成 E18–E24 |
| 3 条 cms「假门」权限探测 | **未收录（新增，P1）** | `is_admin` 只算返回值，不拦截 |

---

## 七、修复排序建议

**P0（先做，都是「登录即可」级别）**

1. **`u4_require_actor` + `work_terminate` 空归属 fail-closed**（16+1 处）。改动极小：空 actor → `require_admin` 或显式 `"system"` 白名单；配套 16 个调用点的「空 creator 403」用例（现有对照组已现成：非空 → 403、匿名 → 401，补空值即可）。
2. **考勤 by-id 三条读**（`attendancedetail_id`、`attendanceappealInfo_id`、`v2_record_get`）与 **`record_update`**：与第五轮 P0-1/2/3 同 crate，一次做完，双向用例（A→B / B→A）。
3. **财务写 IDOR**：`invoice_update`、`invoice_delete`、`general file_update/delete` —— 已复核真落库，优先于所有读端点。
4. **组织写 IDOR**：`definition_update/delete`、`identity_update/delete` —— 可改组织架构/身份，破坏面大。
5. **文档/文件内容读**：`/api/file/{id}/content`、`/api/file/{id}/download`、`/api/document/{id}/view`、`/api/document/{id}/persons` —— 内容泄露，且实现路径与 E13 同 crate，可合并。

**P1**

6. **其余 100+ 读端点按 crate 批量收口**：`processplatform`（12）、`query_assemble_*`（14）、`portal_*`（12）、`program_center`（8）、`meeting/mind/jpush`（各 1–2）。统一改用 `require_owner` 或「本人 OR admin」的 SQL WHERE。
7. **3 条 cms 假门**：把 `is_admin` 用于拦截（非本人且非 published → 403），而不是只返回布尔；`document_cipher_permission_read_u3` 的 `person` 必须改取自会话。

**P2**

8. **31 条 `NO-OWNER-COL`**：与业务确认「全局配置/字典/会议室」的写权限；不要机械套 owner 谓词。
9. **9 条 `SKIP-NO-TABLE`**：补 `PP_C_*`/share 表映射后复跑。

---

## 八、未闭环 / 诚实边界

1. **9 条 `SKIP-NO-TABLE`**：`processplatform_assemble_surface` 的 attachment/snap（`PP_C_*`、share）无静态表映射，**未判**。
2. **31 条 `NO-OWNER-COL`**：无「本人」定义，**需业务裁决**，不计入越权。
3. **26 条 `FAIL-500`**：bbs 写 6 条已由第五轮合法行实证；其余 20 条为结构化列缺失，**未判**。
4. **13 条 `EMPTY`**：file 族出现「库里有行、handler 报 not found」，可能还有一层 `xid`/`file_status` 过滤，需人工确认不是漏判（尤其 `attachment_id`、`file_download`）。
5. **`processplatform_core_express` 的 `work_processing:91`、`task_processing:125`、`work_retract:58` 连 session 都不取**：本轮按「返回数据」计入读越权，但它们的修复不是加归属谓词，而是先补会话门（否则匿名也能读/看到伪造的 `retracted`）。
6. **分母**：421 不含「id 在 query/body」的 125 条口径，也不含 Go 侧；141 是下界，不是全量 IDOR 总数。
7. **漂移**：实测在 `4b4e17010`，落盘 HEAD `c0622c5da`；漂移提交只改维护作业 UTC、测试与台账文档，`require_actor|require_owner|is_admin` 零命中，未触及归属 helper，但**复跑前请以当前 HEAD 重新确认** helper 行号。

---

## 九、复跑

```text
1) docker cp seed-round6-idor.sql lyp-pg:/tmp/ && docker exec lyp-pg psql -U postgres -d oa_audit_20261105 -f /tmp/seed-round6-idor.sql
2) docker cp seed-round6-pp.sql  lyp-pg:/tmp/ && docker exec lyp-pg psql -U postgres -d oa_audit_20261105 -f /tmp/seed-round6-pp.sql
3) 起 Rust 服务 127.0.0.1:3999，登录 audit-user / audit-user2 / admin
4) %TEMP%\audit4\r6_run2.py        # 245 条无门端点（JSONL 断点续跑）
5) %TEMP%\audit4\r6_run_gated.py    # 176 条有门端点动态复核
6) %TEMP%\audit4\t_r6_gate.py       # u4_require_actor 16 调用点
7) %TEMP%\audit4\t_r6_corex.py      # work_terminate 内联
8) %TEMP%\audit4\t_r6_cross.py、t_r6_cross_get.py  # 双向交叉读
9) %TEMP%\audit4\r6_failopen_scan.py              # helper 全仓扫描
```

证据文件：`evidence-round6.txt`（§A 静态分母 → §B 245 条逐条 → §C 不可判 → §D fail-open 逐调用点 → §E 写落库复核 → §F 双向交叉读 → §G 复跑）。
