# 四轮安全审计 · 修复批 R1（三个真缺陷 + 测试基建）小结

- 分支：`oa4rust-vuln-audit-r4`
- 关联主账本：`docs/audits/2026-10-14-security-audit-round4.md`（§三.21 第二十一批「新增待修」3/4/5 由本批闭合）
- 日期：2026-10-15
- 本批性质：**只修「新增待修」里已取证的三个真缺陷 + 顺带打通测试基建**，未新增任何路由门禁，故 GET 写面暴露面不变（未重跑尺子，理由见 §四）。

## 一、修复的缺陷

### 缺陷 1：`require_owner` 归一化口径不一致（IDOR 校验双族 / 本人误拒）
- 位置：`crates/shared/src/middleware/rbac.rs::require_owner`
- 根因：`session.person_unique == owner_id` 是**原始字符串**比较，而 attendance 侧在写库前把身份 `normalize_key`（`trim().to_lowercase()`）归一化。两端口径不一致：
  - 本人（大小写/空白变体）操作被误判为越权而 `403`（`import_checkin_rows` 的 ignored 臂曾红在 `Forbidden`，账本 §三.21 已取证）；
  - 归一化输入 vs 原始会话比较，在 IDOR 判定上语义歧义。
- 修复：`require_owner` 改为两侧统一 `trim().to_lowercase()` 归一化后比较（`admin` 放行支路不变）。
  - 语义：同一身份的大小写/空白变体视为本人；归一化后确为他者仍一律 `Forbidden`，不因归一化放松越权判定。
  - 影响面：全仓 `require_owner` 调用点。唯一行为变化是「大小写变体的本人」从误拒转为放行——这正是既有 intent；`require_owner(..., "")` 一类用法（bam crate admin-only）不受影响（真实 `person_unique` 归一化后非空，仍不等于空串）。
- 调用点同步对齐（attendance，改前都把归一化输入与原值会话比，属同族大小写缺陷）：
  - `mobile_check_impl`（`v2_mobile_check` / `v2_mobile_check_from_out`）
  - `v2_record_delete_people_date` 之前已归一化 `people`，现经 `require_owner` 统一口径即自洽
  - `import_checkin_rows` 逐行权限比较
  - 这三处 handler 的比较改为 `normalize_key(&session.person_unique)` 后与已归一化的目标比。

### 缺陷 2：`checkDetailWithPersonByCycle` 的 42883（integer 列绑 text 路径参数）
- 位置：`crates/attendance_assemble_control/src/lib.rs::attendancedetail_checkDetailWithPersonByCycle_cycleYear_cycleMonth`
- 根因：`cycle_year` / `cycle_month` 真库为 INTEGER（迁移 101 加列），handler 却绑 `Path<(String, String)>` 进 `WHERE cycle_year = $1` → PG 42883（`integer = text` 无隐式转换），`UPDATE` 被 `.unwrap_or(0)` 吞掉、回读 `COUNT` 的 `.map_err(Internal)?` 当场炸出 500。
- 修复：路径绑定改为 `Path<(i32, i32)>`，按真库 integer 口径绑定；非数字路径由 axum 直接 400 拒绝（比原先暗到 500/Internal 更正确）。
- 测试豁免撤销：`security_gate_tests.rs` 曾对该把「admin 格只允许 `Internal`」的**明列豁免**——已按账本 §三.21 要求同步撤销。admin 段现要求 9 把全部放行，落笔回读里 `x_attendance_detail.checked` 由期望 `false` 改为 `true`（造数行 `cycle_year=9999, cycle_month=99` 恰在射程内，UPDATE 真落笔）。

### 缺陷 3：三张考勤表缺主键
- 表：`x_attendance_appeal_info` / `x_attendance_detail` / `x_attendance_setting`（迁移 032 建成时仅 `"id" TEXT`、无 PK、可空）
- 影响：按 id `UPDATE` 无法保证单行，重复 id 时「归档某一条」写成「归档若干条」。
- 修复：新增迁移 `migrations/109_add_attendance_pk.sql`（+ `_rollback.sql`）。逐表：`UPDATE ... SET id = gen_random_uuid()::text WHERE id IS NULL`（补空）→ 自连接删重复 `id`（保留 `ctid` 最小一条）→ `SET NOT NULL` → `ADD PRIMARY KEY (id)`。正常运行库无重复/空时前两步空操作，幂等安全。
- 已对 Docker 测试库应用并验证：三表均 `xxx_pkey`、`id` 置 `NOT NULL`，`schema_migrations` 命中 `109_add_attendance_pk.sql`。

## 二、测试基建（打通本地真实库验证）
- `crates/shared/src/testing.rs`：`test_pool()` / `test_sea_orm_pool()` 增加 `DATABASE_URL` 覆盖（解析 host/port/user/password/dbname，缺失回退默认）。CI 不设该变量 → 行为逐字节不变；本地可用非标端口指向 Docker PG。
- 背景：Windows 主机上 Docker 原生 `5432:5432` 端口代理不通，改用 `15432:5432` 发布 + `DATABASE_URL` 直连，弃用了先前不稳的 Node 双栈代理。

## 三、验证结果

- **本批改动的 attendance crate（`cargo test -p attendance_assemble_control`）**：`45 passed / 0 failed / 15 ignored`，doc-test 0。关键臂：
  - `b21_get_writes_reject_without_session` ok（无库臂，9 把全 401）
  - `b21_get_writes_gate_lands` ok（真库臂：他人全 403、本人只放行归属那把、admin 9 把全放行、`checked` 落笔 true、同步表各 1 行、收口零残留）
  - `r1_require_owner_case_insensitive_self_match` ok（缺陷 1 回归）
  - `r3_attendance_tables_have_primary_key` ok（缺陷 3 回归；admin 段顺带证缺陷 2 的 i32 放行 + 落笔）
- **迁移 109**：应用成功，PK/NOT NULL 逐表确认（见缺陷 3）。
- **shared `require_owner` 归一化变更的跨 crate 影响**（全仓 `require_owner` 消费方逐 crate 对真实库跑 lib 门禁测试）：
  `calendar` / `calendar_assemble_control` / `meeting_assemble_control` / `mind_assemble_control` /
  `ai_assemble_control` / `file_assemble_control` / `general_assemble_control` 共
  **280 passed / 0 failed / 90 ignored**；连同 attendance（45/0/15）共 **325 passed / 0 failed**，
  **无任何回归**。依据：该变更唯一行为变化是「大小写/空白变体的本人」由误拒转放行；上述 crate 的
  gate 测试均用精确身份串区分本人/他者，无依赖大小写变体被拒的断言，故读数不变。
- **全 workspace `cargo test --workspace --lib`（对真实库）**：**编译通过**——`-j 3` 全量重编 96 个
  crate 全部编译成功（证明无编译回归）；但**整跑运行时**两度撞到本机 Windows 构建资源抖动
  （`os error 5` 拒绝访问写 `*.rmeta` 增量目录、`os error 1455` 页面文件太小 mmap `shared` 的 rlib），
  均非代码回归。鉴于 (a) 全量编译通过、(b) 全部 `require_owner` 消费方门禁测试 325/0 绿、
  (c) 本批改动的两个 crate 全绿，本批以「全量编译 + 定向门禁测试全绿」作为验收证据；
  整跑读数受限于本机内存/分页（1455）与构建并发竞态，列为环境噪声而非本批质量问题。

## 四、范围与诚实说明（未做/遗留）
- 本批**未新增任何路由门禁**，故 GET 写面暴露面不变，**未重跑尺子** `audit_write_gating.mjs`（无 gate tier 变动即无表面变化）。工作树里有 14 份审计产物 TSV 显示为 M，**先于本批即存在**（上一轮 campaign 遗留），本批未触碰。
- 本批只闭合**三个已取证真缺陷**。账本「沿用的待拍格」里的大项仍是进行时，**未在本批收口**，按优先级：
  1. **GET 写面队列 53 条**（下一格首选 `auth` 6：OAuth 回调 GET 写，且与「第三方回调零验签」族正面重叠——加门与验签须分开记）；
  2. 第三方回调**零验签**族；`shared/src/crud.rs` CrudSpec 无归属；
  3. `GATE_RE` 里 `require_actor` 无独立见证的冗余词；尺子 `gateEvidence` 剥注释（尺子缺陷 26）；
  4. `row_to_json` JSONB/时间盲口；只读挂写 764 等其余待拍格（详见主账本 §五）。
- 换言之：本批的**三个缺陷已修且验证通过、无回归**；但「审计零遗留」是跨批次的 campaign 目标，尚未到达——上表 1–4 是明确的下一批入口。

## 五、改动文件清单
- 修改：
  - `crates/shared/src/middleware/rbac.rs`（`require_owner` 归一化）
  - `crates/shared/src/testing.rs`（`test_pool` / `test_sea_orm_pool` 读 `DATABASE_URL`）
  - `crates/attendance_assemble_control/src/lib.rs`（42883 i32 绑定 + 三处会话侧归一化对齐）
  - `crates/attendance_assemble_control/src/security_gate_tests.rs`（撤 42883 豁免 + `p2i`/i32 常量 + 回归 `r1`/`r3`）
- 新增：
  - `migrations/109_add_attendance_pk.sql`
  - `migrations/109_add_attendance_pk_rollback.sql`
