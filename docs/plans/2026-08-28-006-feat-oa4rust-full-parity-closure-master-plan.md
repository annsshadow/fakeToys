---
title: "feat: OA4Rust 100% 替代 o2server 全残差闭环总计划"
type: feat
status: active
date: 2026-08-28
origin: docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md
---

# OA4Rust 100% 替代 o2server 全残差闭环总计划

## Summary

本计划整合所有残差项，系统性推进 oa4rust 达到 100% 替代 o2server 的目标。当前状态：端点注册 99.77%（3085/3092，28/30 模块 100%），行为对比 1242 PASS / 806 FAIL / 1996 SKIP，测试覆盖率 ~15%。计划覆盖三层目标：L1 端点注册补齐、L2 行为语义收敛（806 FAIL → ≤400）、L3 生产切流预备。核心工作包括：FAIL 分类修复（6 类）、Express POST 与深层逻辑端点补全、信封统一收尾、测试覆盖率冲刺（15% → ≥95%）、BAM 模块深度对齐、生成器与 CI 基础设施完善。唯一外部阻塞为 R1 生产影子流量灰度验证（需运维排期 ≥2 周观察期）。

---

## Problem Frame

2026-08-28 时点，oa4rust 已在端点注册层接近完成（99.77%），但 "可替代 o2server" 的判定仍受限于三个层面：

1. **行为一致性缺口**：4044 个端点行为对比，仅 1242 PASS (30.7%)，806 FAIL 散落多类（R500J200:29、R401J200:93、R200J200:279、R403J500:25、R200J405:16、R200J415:15），剩余为业务语义差异与数据依赖类。
2. **测试覆盖严重不足**：整体 handler 级测试覆盖率约 15%，5 个零测试 crate（ldap、auth、personal、organization_assemble_authentication、organization_assemble_personal），核心大模块 cms(1%)、processplatform_assemble_surface(3%)、program_center(3%) 覆盖率极低。
3. **生产验证缺失**：影子流量脚本就绪但未在生产跑过，≥2 周观察期是外部阻塞项。

本计划将三类工作统一管理，分阶段交付，最终使"能否替代"可被书面、可验证地回答。

---

## Requirements

- **R1.** 生产环境按 playbook 执行模块级灰度与影子流量比对，观察期 ≥2 周且无核心链路差异后完成切流，归档报告。（外部阻塞，代码层不依赖）
- **R2.** 产出并维护"可替代 o2server"的正式判定结论文档，明确接管范围、前提与已知限制。
- **R9.** 收敛 handler 行为语义一致性：借行为对比框架持续比对，将剩余 FAIL 从 806 收敛至 ≤400，结构性差异（信封、包装、字段形状）逐类修正或留档。
- **R-test-coverage.** 整体测试覆盖率从 ~15% 提升至 ≥95%，零测试 crate 清零。
- **R-bam.** BAM 模块深度对齐：核验 Java 131 @Path 与 Rust 80+ 路由差异，实现 P0 核心监控端点，分阶段验收。
- **R-attachment.** Attachment 4 条 `{}.{}` 端点行为对比验收通过。
- **R-infrastructure.** 生成器脚本纳管、CI behavior-compare 真实化持久化、差异聚类工具生产化。

---

## Scope Boundaries

| 包含 | 排除（Deferred） |
|------|------------------|
| 806 FAIL 的系统性分类修复 | 工作流引擎核心逻辑（processplatform service/processing 信号/回滚/事件） |
| 信封统一、Express POST、深层逻辑缺口补全 | CMS `mockdeletetoget` 模式语义待确认 |
| 测试覆盖率 15% → ≥95%，零测试 crate 清零 | 国产库（达梦/金仓）适配验证 |
| CI behavior-compare 真实化 + 报告留存 | 新增 o2server 不存在的功能 |
| BAM P0 端点实现 + 差异清单 | IM/XMPP/WebRTC 完整协议（R6 已排除） |
| 生成器与工具链完善 | 数据不对称类 FAIL（需真实业务数据/共享种子） |
| 行为差异 backlog 固化与治理机制 | 真实生产切流执行（R1，运维排期） |

### Deferred to Follow-Up Work

- schema 级投影重构（若字段聚类揭示需重构）：拆独立 refactor 计划
- Java 侧种子进 CI（需容器内 REST 编排）：待部署剧本稳定后评估
- 任何超出 99.77% 基线外的新端点发现：回到需求文档修订

---

## Context & Research

### 权威基线与历史状态

- **端点对齐终扫**：`docs/audits/final-coverage-sweep.md`（generated_at=2026-08-23，3085/3092 = 99.77%，28/30 模块 100%）
- **行为对比基线 V4**：`oa4rust/target/debug/behavior-report.md`（2026-08-27T09:27:20，1242 PASS / 806 FAIL / 1996 SKIP）
- **已完成计划**：
  - `2026-08-25-001-oa4rust-residual-gaps-closure-plan.md`（R3-R10 收官，completed）
  - `2026-08-26-001-feat-parity-convergence-phase2-plan.md`（CI 真实化，completed）
  - `2026-08-26-002-oa4rust-fail-closure-master-plan.md`（信封统一三线，completed）
  - `2026-08-28-005-fix-residual-fail-endpoints-plan.md`（FAIL 分类修复，active）

### 关键代码与模式

| 领域 | 关键文件/模式 |
|------|---------------|
| 行为对比框架 | `tests/behavior_compare.rs`、`tests/behavior_comparison/endpoints.rs`（4687 条自动生成）、`comparator.rs`、`allowlist.yaml`（26+65 条改名对） |
| 响应契约 | `crates/shared/src/response.rs` `ActionResult` 9 字段、`success(data)` vs `java_success(data, count, size)` |
| 错误处理 | `crates/shared/src/error.rs` `AppError` 9 变体、`IntoResponse` 实现 |
| 认证中间件 | `crates/shared/src/middleware/constants.rs` `AUTH_EXEMPT_PATHS`、`rbac.rs` `PermissionRegistry` |
| BAM crate | `crates/processplatform_assemble_bam/`（lib.rs/routes.rs，80+ 路由，Java 参考 131 @Path） |
| 生成器 | `oa4rust/scripts/regen_endpoints.py`、`gen_openapi_paths.py`、`cluster_behavior_diffs.py` |
| 部署剧本 | `oa4rust/deploy/shadow-traffic.sh`、`toggle_module.sh`、`rollback-playbook.md` |
| 种子资产 | `tests/behavior_comparison/seeds/`（seed_fixtures.sql、seed_fixtures_java.http.md） |

### 机构学习（强制遵守）

- `security-issues/idor-vulnerability-write-handlers.md`：**写端点必须 `require_owner` + `creator_person` 取自 Session**，否则 P0 阻断合并
- `architecture-patterns/actionresult-9-field-contract.md`：所有端点遵循 9 字段契约，业务错误 HTTP 200 + type=error
- `integration-issues/nested-tokio-runtime-panic.md`：router factory 为 sync fn，SeaORM 连接须 `catch_unwind` 包裹
- `best-practices/auto-generate-rust-handler-tests.md`：含 Session 的 handler 被测试生成器跳过（105 个），需补 router-based 或手测
- `best-practices/single-source-of-truth-migration-status.md`：状态汇报以 `final-coverage-sweep.md` 为权威
- `oa4rust-o2server-parity-closure-campaign-2026-08-25.md`：包装模式战役方法论（`java_success` 统一、小批重测）

---

## Key Technical Decisions

| 决策 | 理由 |
|------|------|
| **FAIL 分类修复策略**：先易后难（R500→R401→R403→R405/R415→R200J200 Stub→深层逻辑） | 低成本高收益先行消除，每步有量化验收；R500/R401 合计 ~122 条 FAIL，修复即可消除 ~15% |
| **信封统一仅改列表端点** | 单对象 `success(data)` 与 Java 行为一致（count=0, size=0），避免过度改动；Phase 1 经验验证 |
| **不手改 endpoints.rs** | 自动生成文件，必须通过生成器消费的路由面登记后重新生成，否则下次重建丢失 |
| **allowlist 仅收"同义异名"** | 种子轮聚类证明 401 候选全为结构性差异，入库会粉饰真实问题；结构性差异必须 handler 层修正 |
| **测试覆盖率采用"生成器优先 + 关键路径人工"** | `generate_parity_tests.py` 批量基线测试，人工仅补核心业务流；含 Session handler 强制人工补充 |
| **R1 生产切流不阻塞代码层工作** | R7/U5/U9/U13-U22 可并行推进；R1 仅需运维排期，代码交付不等待 |
| **BAM P0/P1/P2 分阶段** | Java 131 @Path 中 P0 核心监控 ~20-30 端点优先补齐；P1 多维聚合进 backlog 标注优先级；P2 高级能力（告警/报表）明确留档 |

---

## Open Questions

### 计划期解决
- [ ] R1 切流回滚触发阈值与观察期具体长度（≥2 周为下限）：**运维确认** → U23
- [ ] 零测试 crate 优先级排序：**auth（认证核心）> personal > ldap > org_assemble_auth > org_assemble_personal** → U13

### 实现期延后
- [ ] Java 131 @Path 与 Rust BAM 路由逐项差异精确清单：U17 实现期从 Java 源码提取
- [ ] 深层逻辑缺口的精确 crate 归属：聚类脚本输出为准 → U22
- [ ] CI 服务容器 `POSTGRES_DB` 与 behavior job `DATABASE_URL` 对齐：执行期定

---

## Implementation Units

### Phase 0：基础设施与工具链（0 依赖，可并行启动）

#### U1. 差异聚类脚本 `cluster_behavior_diffs.py` 生产化与 CI 集成

**Goal**：将本地一次性脚本升级为 CI 可复用工具，输出机读 TSV + 人审 Markdown，纳入版本控制。

**Requirements**：R9、R-infrastructure

**Dependencies**：无

**Files**：
- Create: `oa4rust/scripts/cluster_behavior_diffs.py`（重写生产化版本）
- Modify: `.gitignore`（scripts 白名单追加）

**Approach**：
1. 重构为 CLI：`--report <path> --out-dir <dir> --format tsv,md`
2. 解析 behavior-report.md FAIL 行，聚类规则：
   - 同端点 `A: missing in Java` + `B: missing in Rust` → 候选改名对 (A,B)
   - 跨端点聚合频次
   - `type differs` 单列归类不进改名对
3. 输出写 `target/` 不入仓：`diff_candidates.tsv`（pair、频次、示例端点列表）+ `diff_candidates.md`（按频次排序、每对附最多 3 条端点证据摘录）
4. CI job 新增步骤：全量 compare 后自动跑聚类，上传产物

**Patterns**：`regen_endpoints.py` 路径基准、编码处理、main 流程风格

**Test Scenarios**：
- Happy: 样例报告含 3 类 diff → TSV 行数与配对正确、type-differs 不产生 pair
- Edge: 空报告/无 FAIL 行 → 空产物 exit 0
- Error: 报告文件不存在 → 明确报错退出非零

**Verification**：对当前 V4 behavior-report.md 跑通；CI 上传产物可被 U3 直接消费

---

#### U2. CI behavior-compare 真实化持久化：种子步骤 + 报告产物留存

**Goal**：每次 PR 产出真实 parity 数字，基线有守护、退化可见。

**Requirements**：R9（支撑持续收敛）

**Dependencies**：U1

**Files**：
- Modify: `oa4rust/.github/workflows/ci.yml`

**Approach**：
1. behavior-compare job：就绪探针后、测试前插入种子步骤——psql 对 job 库执行 `tests/behavior_comparison/seeds/seed_fixtures.sql`，失败即 fail-fast
2. 测试后 `actions/upload-artifact@v4` 上传 `target/debug/behavior-report.md`（`if: always()`，命名含 run_id）
3. 日志打印基线注记：CI 无 Java REST 种子，FAIL 基线预期高于本地双栈值
4. 新增 job 步骤：下载报告产物 → 跑 `cluster_behavior_diffs.py` → 上传聚类产物

**Patterns**：现有 ci.yml step 组织、env 注入风格；upload-artifact 写法

**Test Scenarios**：
- 本地 docker 复刻：起 postgres→应用种子→断言行数>0
- 故意错库名 → psql 非零退出、job 终止（fail-fast 验证）
- 种子幂等重放 → 无报错、行数不变

**Verification**：PR 模拟触发能上传 report + 聚类产物；本地复刻步骤全绿

---

#### U3. 字段映射层收敛：候选评审入库 + handler 投影修正 + 重测闭环

**Goal**：将字段形状层"同义异名"经 allowlist 收敛、"Rust 投影错误"经 handler 修正，产出新一轮全量对比数字。

**Requirements**：R9（核心）

**Dependencies**：U1（候选来源）、U2（CI 持续化）

**Files**：
- Modify: `tests/behavior_comparison/allowlist.yaml`
- Modify: 按聚类结果确定的 crate handler（上限先取可转化对最高的 5 个 crate）

**Approach**：
1. 评审门：逐候选对核对 U1 附带证据，三态裁决——采纳改名对 / 判定投影错误转 handler 修正 / 驳回（记录理由）
2. handler 修正仅动 SELECT 别名/组装层，遵循 ActionResult 9 字段契约 + IDOR 门禁
3. 每采纳 ≤20 条即重测一次，小步验证避免回归不可定位
4. 收敛后全量 compare，记录 PASS/FAIL/SKIP 前后对照表入终扫追加小节

**Execution Note**：每批次入库后立即 `cargo test --test behavior_compare` 验证增量

**Patterns**：allowlist.yaml reason 写法；包装模式战役实测形状对齐判据

**Test Scenarios**：
- Happy: 采纳对重测后对应端点 FAIL→PASS
- Edge: 同名对在 A 端点同义、B 端点非同义 → 不入库，记录冲突
- Error: 误入库导致先前 PASS 转 FAIL → 重测捕获回退、reason 标注废弃
- Integration: 触碰 handler 后对应 crate 单测 0 failed

**Verification**：全量 compare 数字相对 1242/806 基线增量记录；allowlist 新增 100% 带 reason+证据引用

---

### Phase 1：FAIL 分类系统性修复（核心代码工作）

#### U4. 修复所有 R500J200（Rust 500，Java 200）——Server Error 消除

**Goal**：R500J200 从 ~29 降至 0。

**Requirements**：R9

**Dependencies**：U2

**Files**：Modify: 各 crate `src/lib.rs`（handler 与 SQL）

**Approach**：
1. 从 behavior-report.md 提取所有 R500J200 端点
2. 分类修复：
   - **SQL cast 问题**（如 `LIMIT $4::bigint` → `LIMIT $4::int`）：query_assemble_surface `view_id_execute_v2_page_page_size_size`
   - **路由参数不匹配**（`Wrong number of path arguments`）：系统性 grep 路由模板 `{param}` 计数 vs handler `Path<...>` 参数数量，tuple Path 修正
   - **DB 查询崩溃**（空表/缺实体）：返回默认值/空数组而非 panic
   - **重复路由导致 panic**：去重注册
3. 每修复一类即 curl 单端点验证 200

**Patterns**：query_assemble_surface line 1957 cast 修复；hotpic/component/jpush `get/{id}` 路由参数补齐

**Test Scenarios**：
- 每个修复端点 curl 返回 200 + `type: "success"`
- 无新增 `Wrong number of path arguments` 错误

**Verification**：R500J200 = 0；全量 compare 无回归

---

#### U5. 扩大 R401J200 豁免范围（Rust 401，Java 200）——认证不一致消除

**Goal**：R401J200 从 ~93 降至 ≤20。

**Requirements**：R9

**Dependencies**：无

**Files**：
- Modify: `crates/shared/src/middleware/constants.rs` (`AUTH_EXEMPT_PATHS`)
- Modify: `crates/shared/src/middleware/rbac.rs` (`PermissionRegistry`)

**Approach**：
1. 分类 93 个 R401J200 端点：
   - 纯查询 GET 列表/详情 → 加入 `AUTH_EXEMPT_PATHS`（前缀匹配优先）
   - 写操作 POST/PUT/DELETE → 保持认证要求
   - 系统配置端点 → 评估是否公开
2. 批量添加豁免路径：
   - `/jaxrs/person/list/*`、`/jaxrs/unit/list/*`、`/jaxrs/group/list/*`、`/jaxrs/role/list/*`（扩展现有）
   - `/jaxrs/processplatform/assemble/surface/work/count/*`、`/jaxrs/attendance/assemble/control/*`（新增）
3. 同步更新 `PermissionRegistry` 对应前缀为 `Public`

**Test Scenarios**：
- 被豁免端点无 token 返回 200
- 写操作端点仍需 token 返回 401

**Verification**：R401J200 ≤ 20

---

#### U6. 修复 R403J500（Rust 403，Java 500）——权限过严消除

**Goal**：R403J500 从 25 降至 0。

**Requirements**：R9

**Dependencies**：U5

**Files**：Modify: `crates/shared/src/middleware/rbac.rs`

**Approach**：
1. 分析 25 个端点共同特征：Java 返回 500（未登录时错误处理），Rust 返回 403（RBAC 正确拦截）
2. 方案优先级：
   - A) 加入 `AUTH_EXEMPT_PATHS`（若 Java 实际公开）
   - B) `PermissionRegistry` 设为 `Public`
   - C) 保持 403（Rust 更严格，可接受，留档）

**Verification**：R403J500 类别计数为 0

---

#### U7. 修复 R200J405（Method Not Allowed）——HTTP 方法不匹配

**Goal**：R200J405 从 16 降至 0。

**Requirements**：R9

**Dependencies**：U4

**Files**：Read: `tests/behavior_comparison/endpoints.rs`；Modify: 各 crate 路由注册

**Approach**：
1. 从 endpoints.rs 找出 16 个端点的 method 定义
2. 若 Rust 只注册 POST 但测试用 GET：更新 endpoints.rs method 为 POST（方案 A，推荐）
3. 保持端点语义，不补注册 GET 变体

**Verification**：R200J405 = 0

---

#### U8. 修复 R200J415（Unsupported Media Type）——Content-Type 缺失

**Goal**：R200J415 从 15 降至 0。

**Requirements**：R9

**Dependencies**：无

**Files**：Modify: `tests/behavior_comparison/comparator.rs`（请求构造逻辑）

**Approach**：
1. comparator.rs 扩展 POST/PUT/PATCH 请求自动添加 `Content-Type: application/json` 头逻辑
2. 验证 Java 不再返回 415

**Verification**：R200J415 = 0

---

#### U9. 修复关键 R200J200 Stub 端点（双方 200 但 data 结构不同）

**Goal**：R200J200 从 ~279 降低 ≥50（优先修复 Stub 类）。

**Requirements**：R9

**Dependencies**：U3

**Files**：Modify: 各 crate handler（填充真实查询逻辑）

**Approach**：
1. 从 behavior-report.md 提取 R200J200，分类：
   - **Stub 类**：handler 返回 `{}`/`[]`，应返回真实查询结果（最易修复）
   - **字段缺失类**：缺 Java 侧字段
   - **类型差异类**：Array vs Object
2. 优先修复 Stub 类：加真实 SQL 查询，返回 `java_success(data, count, size)`
3. 重点模块：processplatform_assemble_surface、attendance_assemble_control、query_assemble_surface

**Test Scenarios**：每个修复端点从 R200J200 → PASS 或 SKIP

**Verification**：R200J200 减少 ≥50；全量 compare 无回归

---

#### U10. Express POST 列表端点补全（~50 端点）

**Goal**：实现 `organization_assemble_express` 所有 POST list 端点。

**Requirements**：R9

**Dependencies**：U3、U4

**Files**：
- Modify: `crates/organization_assemble_express/src/lib.rs`
- Test: `crates/organization_assemble_express/src/tests.rs`

**Approach**：
1. 端点清单（~50 条，按 body 参数分组）：
   - **人员查询**（18 条）：`/person/list`、`/person/list/group`、`/person/list/identity`、`/person/list/role`、`/person/list/unit/sub/direct`、`/person/list/unit/sub/nested`、`/person/list/person/sub/direct`、`/person/list/person/sub/nested`、`/person/list/person/sup/direct`、`/person/list/person/sup/nested`、`/person/list/login/after`、`/person/list/login/recent`、`/person/list/pair/identity`、`/person/list/group/object`、`/person/list/identity/object`、`/person/list/unit/sub/direct/like`、`/person/list/unit/sub/nested/like`、`/person/detail/{flag}`
   - **单位查询**（12 条）：`/unit/list/identity`、`/unit/list/identity/sup/nested`、`/unit/list/level`、`/unit/list/person`、`/unit/list/person/sup/nested`、`/unit/list/types`、`/unit/list/unitduty`、`/unit/identity/level`、`/unit/identity/type`、`/unit/check/unit/has/identity`、`/unit/check/unit/has/person`、`/unit/check/unit/has/unit`
   - **组/角色/属性**（16 条）：`/group/list`、`/group/list/group/sub/direct`、`/group/list/group/sub/nested`、`/group/list/group/sup/direct`、`/group/list/group/sup/nested`、`/group/list/identity`、`/group/list/person`、`/group/has/role`、`/person/has/role`、`/role/list`、`/role/list/person`、`/personattribute/append/person/name`、`/personattribute/set/person/name`、`/unitattribute/append/unit/name`、`/unitattribute/set/unit/name`
2. 每端点：解析 POST body → 复用 Control 模块查询函数 → 返回 `java_success(data, count, size)`
3. 用宏/辅助函数减少重复代码

**Test Scenarios**：
- Happy: 按组查询返回正确人员列表
- Edge: 空查询条件/不存在对象返回空列表（不报错）
- Integration: 与 Control 模块查询结果一致

**Verification**：所有 50 端点 FAIL→PASS；单测覆盖每端点

---

#### U11. 深层逻辑缺口补全（其余模块 ~140 端点）

**Goal**：实现散落在各模块的缺口 handler。

**Requirements**：R9

**Dependencies**：U3、U9

**Files**：Modify: 各模块 `src/lib.rs`；Test: 各模块 `tests.rs`

**Approach**：
分模块实施（按 FAIL 密度排序）：
1. **organization_assemble_control**（13 条）：组织树层级查询（直属/嵌套下级/上级组、按角色/人员查身份、密码检查、属性/名片 VCF）
2. **cms_assemble_control**（13 条）：分类物理删、文档批量删、应用下分类管理、文档关联/站点关联、应用字典设计、应用文件列表、文件批量/单文件下载、表单字段列表、文件上传
3. **processplatform_assemble_surface**（13 条）：工作数据删、应用字典/附件/文件/流程列表、可用身份/可控流程、流水号、前一个手动已完成任务
4. **query 模块**（18 条）：表行删/导入模型/统计/表/视图/搜索/SQL 语句/表行数据/保存行
5. **其余模块**（~45 条）：program_center(9)、message(7)、calendar(6)、file(5)、portal(8)、meeting(3)、general(4)、ai(2)、personal(4)、mind(2)、processplatform_service_processing(5 仅简单 3 条)

**Patterns**：复用现有查询函数、递归遍历 `org_unit.superior`（CTE 限制深度）、StreamingBody 文件下载

**Verification**：可实现 ~140 端点 FAIL→PASS；复杂工作流引擎逻辑进 backlog (U22)

---

#### U12. 信封统一收尾：全量列表端点 `success()` → `java_success()`

**Goal**：消除剩余信封差异类 FAIL（预估 ~200 条）。

**Requirements**：R9

**Dependencies**：U3、U9、U10、U11（新实现端点已用 java_success）

**Files**：Modify: 各 crate `src/lib.rs`（grep `ActionResult::success(` 定位列表端点）

**Approach**：
1. 仅改**列表端点**（返回 `Value::Array`/`Vec`），单对象端点保持 `success(data)`
2. `count` 从查询总数获取，`size` 从实际返回数量获取
3. 分 crate 批次：organization(~23) → cms(~58) → processplatform(~82) → query/attendance/message(~44) → 其余小模块(~60)
4. 每 crate 改完即跑全量 compare 验证 PASS 增量

**Patterns**：Phase 1 提交 9d81b8ca：214 handler 成功经验；`actionresult-9-field-contract.md`

**Verification**：信封差异类 FAIL ≤ 50；全量 compare PASS 增量 ≥ 300

---

### Phase 2：测试覆盖率冲刺（并行 Phase 1）

#### U13. 零测试 Crate 补测（5 个零覆盖 crate）

**Goal**：ldap、auth、personal、organization_assemble_authentication、organization_assemble_personal 从 0% → ≥80%。

**Requirements**：测试覆盖率 ≥95%

**Dependencies**：无

**Files**：Create/Modify: 各 crate `src/tests.rs`、`src/tests_generated.rs`

**Approach**：
1. 优先级：`auth`（认证核心）> `personal` > `ldap` > `org_assemble_auth` > `org_assemble_personal`
2. 策略：
   - `generate_parity_tests.py` 批量生成基线测试（路由挂载+Schema 校验）
   - 含 Session handler 补 router-based 测试（绕过生成器跳过）
   - 核心业务流（login/logout/whoami/captcha/oauth）补手写集成测试
3. 目标：每 crate 覆盖 ≥80%

**Patterns**：`best-practices/auto-generate-rust-handler-tests.md`、现有 `tests_generated.rs` 模式

**Verification**：`cargo test -p <crate>` 全绿；覆盖率报告 ≥80%

---

#### U14. 核心大模块测试覆盖冲刺

**Goal**：cms_assemble_control(1%)、processplatform_assemble_surface(3%)、program_center(3%) 从 1-3% → ≥80%。

**Requirements**：测试覆盖率 ≥95%

**Dependencies**：U12（信封统一后 handler 签名稳定）

**Files**：Modify: 各 crate `src/tests.rs`、`src/tests_generated.rs`

**Approach**：
1. **cms_assemble_control**（311 handler）：核心 CRUD（appinfo/categoryinfo/document/fileinfo）、全文检索、权限 control
2. **processplatform_assemble_surface**（487 handler）：工作流基础 CRUD、任务/工作查询、附件/表单/字典
3. **program_center**（205 handler）：应用打包/认证/配置/部署
4. 方法：生成器批量基线 + 核心路径人工补测 + router-based 绕过 Session 限制

**Verification**：三大模块覆盖 ≥80%；整体覆盖率突破 50%

---

#### U15. 中优先级模块测试补齐（6-20% → ≥80%）

**Goal**：organization_assemble_control(8%)、processplatform_service_processing(10%)、processplatform_assemble_designer(12%)、attendance(6%)、query_designer(13%)、general(7%)、query_surface(15%)、message(10%)、bbs(18%)、portal_designer(20%)。

**Requirements**：测试覆盖率 ≥95%

**Dependencies**：U13、U14

**Approach**：同策略：生成器批量基线 + 核心路径人工补测

**Verification**：所有模块覆盖 ≥80%；整体覆盖率 ≥95%

---

#### U16. 测试基础设施完善：CI 覆盖率门禁

**Goal**：CI 每次 PR 强制检查覆盖率不回滑。

**Requirements**：测试覆盖率 ≥95% 可持续

**Dependencies**：U13-U15

**Files**：Modify: `oa4rust/.github/workflows/ci.yml`

**Approach**：
1. CI 新增 `coverage` job：`cargo llvm-cov --workspace --lcov --output-path lcov.info`
2. 上传 `lcov.info` 为 artifact
3. 可选：自定义脚本阈值检查（如 `< 90%` 则 warn）

**Verification**：CI 产出覆盖率报告；阈值检查生效

---

### Phase 3：BAM 深度对齐与附件验收（依赖 Phase 1）

#### U17. BAM 模块 Java 131 @Path 与 Rust 80+ 路由深度差异核验

**Goal**：产出权威差异清单，分阶段验收（核心优先、低频后补）。

**Requirements**：R-bam

**Dependencies**：U1、U2

**Files**：
- Read: `oa/o2server/x_processplatform_assemble_bam/`（Java 源码）
- Modify: `crates/processplatform_assemble_bam/src/routes.rs`、`lib.rs`

**Approach**：
1. 抽取 Java 全量 @Path（约 131 个），按功能分类：
   - **P0 核心监控**（实时大屏、流程瓶颈、人员效能、SLA 预警）：~20-30 端点
   - **P1 多维聚合**（应用/流程/环节/人员/时间交叉统计）：~40-50 端点
   - **P2 高级能力**（告警订阅、仪表盘模板、定时报表导出）：~30-40 端点
2. 与 Rust 现有路由（period/list/*、state/*、count/*）做差集
3. P0 端点本计划实现；P1 进 backlog 标注优先级；P2 明确留档

**Patterns**：既有 BAM handler 风格：`Path<(String,...)>` + `Extension<Pool>` + `ActionResult<Value>` + `require_owner`（写端点）

**Verification**：P0 端点 behavior_compare PASS；差异清单文档化入 `docs/audits/bam-alignment-gap.md`

---

#### U18. Attachment 4 端点行为对比验收

**Goal**：4 条 `attachment/download/*/{}.{}` 端点 behavior_compare PASS。

**Requirements**：R5、AE5

**Dependencies**：U1（已实现，待验收）

**Files**：无新增（验证现有）

**Approach**：
1. 确认 4 端点已在 endpoints.rs（生成器重生成后）
2. 运行 behavior_compare，验证 4 端点 PASS
3. 终扫文档刷新：将"平台限制"4 条标记为已覆盖

**Verification**：4 端点 PASS；终扫"平台限制"项移除

---

#### U19. 部署剧本本地双栈演练（为 R1 扫障）

**Goal**：在本地 Docker 双栈端到端执行灰度剧本，验证可操作性。

**Requirements**：R1（预备）

**Dependencies**：无

**Files**：
- Execute: `oa4rust/deploy/shadow-traffic.sh`、`toggle_module.sh`
- Modify: 两脚本 `TEST_MODULES`/`DEFAULT_GRAY_MODULES`（补登记 processplatform、bam）
- Modify: `oa4rust/deploy/rollback-playbook.md`（追加演练记录）

**Approach**：
1. bash 容器执行：`status` → `gray 10%` → `status` 断言生效 → `rollback` → `status` 断言还原
2. `shadow-traffic.sh run` 对本地 3000/18080 发一轮影子请求
3. 记录每步输出与偏差；nginx 依赖阻塞则评估最小绕过（直连后端）并留档
4. 演练记录固化进 rollback-playbook.md：命令序列、观察点、已知限制、运维交接注意事项

**Verification**：演练记录存在于 playbook；全程无非预期中断；发现缺陷要么已修要么逐条留档

---

### Phase 4：文档收口与治理（贯穿全程，最后收敛）

#### U20. 「可替代」判定声明维护与迭代

**Goal**：随代码层闭环进展，迭代更新正式判定文档。

**Requirements**：R2

**Dependencies**：U1-U18（各阶段产出）

**Files**：Modify: `docs/REPLACEABLE-oa4rust-2026-08-25.md`

**Approach**：
1. 阶段性更新：
   - Phase 1 后：更新行为一致性进展（PASS/FAIL 数字、收敛类别）
   - Phase 2 后：更新测试覆盖率达标声明
   - Phase 3 后：更新 BAM 深度对齐状态、attachment 验收
2. 最终版：汇总端点对齐度、平台限制、协议排除、BAM 处置、语义一致性、测试覆盖率、生产切流前置条件

**Verification**：声明可被 R1-R10 及测试覆盖率全部追溯；经 A3 签核

---

#### U21. 文档口径一致性刷新

**Goal**：刷新 campaign doc 与 final-coverage-sweep 中残留的"平台限制/不实现"误分类。

**Requirements**：一致性

**Dependencies**：U17、U18

**Files**：
- Modify: `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- Modify: `docs/audits/final-coverage-sweep.md`

**Approach**：
1. campaign doc：将"4 条 axum 平台限制不实现"改为"已用整段 `Path<String>` 捕获模式闭环（U1/U18）"
2. final-coverage-sweep：附录 4 条"🟠 平台限制"改为已覆盖，更新覆盖率口径
3. 补 BAM 现状注记（已 80+ 路由，P0 端点实现中）
4. 建议随后 `/ce-compound` 将"整段捕获推翻平台限制"作为新学习回填 `docs/solutions/`

**Verification**：两文档与代码事实一致，无"平台限制不实现"残留

---

#### U22. 行为差异 backlog 文档化与治理机制

**Goal**：把"真缺口"从报告转化为可排期的开发任务清单，建立定期重测机制。

**Requirements**：R9（长期治理）

**Dependencies**：U3、U9、U11（各轮重测后最新报告为数据源）

**Files**：Create: `docs/audits/behavior-divergence-backlog.md`

**Approach**：
1. 筛选口径：Rust 错误而 Java 成功且 U3/U9 评审未归入改名/投影修正的端点，加上其他结构性差异残留
2. 每条记录：endpoint、method、证据摘录（双侧响应要点）、疑似缺失能力（参照 Java Action 名）、建议归属 crate、出现频次
3. 按频次降序排列；文首注明生成日期与报告版本
4. 治理机制：每季度跑全量 compare → 运行聚类脚本 → 评审新增候选 → 更新 backlog → 记录收敛进度

**Verification**：backlog 覆盖筛选口径下全部端点（计数对账）；抽 3 条核对证据摘录与报告一致

---

### Phase 5：生产影子流量切流（外部阻塞，仅预备）

#### U23. 生产环境影子流量灰度验证与切流（R1）

**Goal**：在生产环境执行 ≥2 周影子流量比对，零核心差异后切流下线 o2server。

**Requirements**：R1、AE1

**Dependencies**：U1-U18（代码层全闭环）、U19（剧本演练通过）

**Files**：Execute: `oa4rust/deploy/shadow-traffic.sh`、`toggle_module.sh`

**Approach**：
1. 前置条件确认：生产环境就绪、灰度脚手架就位、监控告警配置完备
2. 按 playbook 模块级灰度：processplatform(10%→50%→100%)、bam、cms、org 等核心模块
3. `shadow-traffic.sh` 并行比对 Rust/Java 响应，观察期 ≥2 周
4. 差异报告为空则执行切流；否则回滚（5 分钟 RTO）
5. 切流后监控无回归，o2server 下线确认
6. 归档完整比对报告

**External Dependencies**：运维排期、生产环境准入、≥2 周观察窗口

**Verification**：比对报告归档；o2server 可下线且切流后监控无回归

---

## System-Wide Impact

- **Middleware 变更**：U5/U6 修改 `AUTH_EXEMPT_PATHS`/`PermissionRegistry` 影响所有请求路径，需回归测试认证流
- **路由变更**：U4/U7 可能修改 handler 的 `Path<...>` 签名，需验证路由挂载正确性
- **测试框架**：U8 修改 `comparator.rs` 影响所有端点测试行为
- **数据库查询**：U9/U10/U11 修改 SQL 查询，需确保 PostgreSQL 兼容性与索引利用
- **CI 流程**：U1-U2 新增 CI 步骤影响 PR 构建时间与产物大小
- **API 表面一致性**：所有新端点必须同时出现在 Rust 路由与 `ENDPOINTS` 清单，否则 behavior_compare 无法覆盖
- **Unchanged invariants**：ActionResult 9 字段契约、双池架构、前端 action.js 消费方式、IM 协议范围均不变

---

## Risks & Dependencies

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|----------|
| FAIL 修复引入回归（改动 ~600 handler + ~200 新实现） | High | High | 分 Unit 小批重测；每批次全量 compare 验证增量；`success()` 函数保留不删 |
| 部分端点 count 取值不确定（无总数查询） | Medium | Medium | 无总数用 `len()`；分页端点从 COUNT(*) 查询获取 |
| Express POST body 格式不确定 | Medium | Medium | 从 Java 源码推断或 curl 验证真实请求 |
| 组织树递归查询性能（CTE 深度） | Low | Medium | 限制递归深度 `MAX_RECURSION=10` 或用物化路径 |
| CMS `mockdeletetoget` 模式语义不明 | Medium | Low | 标记 backlog，不实现，兜底返回 405 |
| 工作流引擎核心逻辑复杂 | High | High | 仅实现简单查询，复杂逻辑进 backlog (U22) |
| 信封统一后前端可能受影响 | Low | Medium | 信封字段是超集（count/size 仅增不减），向后兼容 |
| 测试覆盖率工具链集成失败 | Low | Medium | 优先 `cargo-llvm-cov`（原生），备选 `tarpaulin` |
| R1 生产切流外部阻塞无法推进 | High | Critical | 代码层工作不阻塞 R1；U19 演练提前暴露脚本问题；文档层声明可先行发布 |

---

## Success Metrics

| 指标 | 当前基线 | 目标 | 验收方式 |
|------|----------|------|----------|
| 端点注册覆盖率 | 99.77% (3085/3092) | 100% | `final-coverage-sweep.md` 终扫 |
| 行为对比 PASS | 1242/4044 (30.7%) | ≥2000 (≤400 FAIL) | `behavior-report.md` 全量跑 |
| R500J200 FAIL | ~29 | 0 | behavior-report 分类统计 |
| R401J200 FAIL | ~93 | ≤20 | behavior-report 分类统计 |
| R403J500 FAIL | ~25 | 0 | behavior-report 分类统计 |
| R200J405 FAIL | ~16 | 0 | behavior-report 分类统计 |
| R200J415 FAIL | ~15 | 0 | behavior-report 分类统计 |
| R200J200 FAIL | ~279 | ↓≥50 | behavior-report 分类统计 |
| 整体测试覆盖率 | ~15% | ≥95% | `cargo llvm-cov --workspace` 报告 |
| 零测试 crate 数 | 5 | 0 | `cargo test -p <crate>` 全绿 |
| 核心模块覆盖 (cms/pps/pc) | 1-3% | ≥80% | 单 crate 覆盖率报告 |
| BAM P0 端点 PASS | 未验证 | 100% | behavior_compare BAM 端点全绿 |
| Attachment 4 端点 PASS | 未验证 | 4/4 | behavior_compare 对应端点 |
| 生产影子流量 | 未启动 | ≥2 周零核心差异 | 运维归档报告 |

---

## Documentation Plan

| 文档 | 更新时机 | 内容 |
|------|----------|------|
| `docs/audits/behavior-divergence-backlog.md` | U22 创建，季度更新 | 深层逻辑缺口清单与治理 |
| `docs/audits/bam-alignment-gap.md` | U17 创建 | BAM 模块差异清单与 P0/P1/P2 分类 |
| `docs/REPLACEABLE-oa4rust-2026-08-25.md` | U20 阶段性更新 | "可替代"判定声明迭代 |
| `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md` | U21 刷新 | 消除"平台限制"误分类 |
| `docs/audits/final-coverage-sweep.md` | U21 刷新 | 端点覆盖率口径一致性 |
| `oa4rust/deploy/rollback-playbook.md` | U19 追加 | 部署剧本本地演练记录 |

---

## Sources & References

- **源需求**：[docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md](docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md)
- **权威基线**：[docs/audits/final-coverage-sweep.md](docs/audits/final-coverage-sweep.md) (99.77%, 2026-08-23)
- **行为基线**：`oa4rust/target/debug/behavior-report.md` (V4, 1242/806/1996)
- **已完成计划**：
  - [docs/plans/2026-08-25-001-oa4rust-residual-gaps-closure-plan.md](docs/plans/2026-08-25-001-oa4rust-residual-gaps-closure-plan.md) (completed)
  - [docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md](docs/plans/2026-08-26-001-feat-parity-convergence-phase2-plan.md) (completed)
  - [docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md](docs/plans/2026-08-26-002-oa4rust-fail-closure-master-plan.md) (completed)
  - [docs/plans/2026-08-28-005-fix-residual-fail-endpoints-plan.md](docs/plans/2026-08-28-005-fix-residual-fail-endpoints-plan.md) (active)
- **方法论沉淀**：[docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md](docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md)
- **机构学习**：
  - `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`
  - `docs/solutions/architecture-patterns/actionresult-9-field-contract.md`
  - `docs/solutions/integration-issues/nested-tokio-runtime-panic.md`
  - `docs/solutions/best-practices/auto-generate-rust-handler-tests.md`
- **关键代码**：
  - 行为对比：`tests/behavior_compare.rs`、`tests/behavior_comparison/`
  - 响应契约：`crates/shared/src/response.rs`、`error.rs`
  - 认证中间件：`crates/shared/src/middleware/constants.rs`、`rbac.rs`
  - 生成器：`oa4rust/scripts/regen_endpoints.py`、`gen_openapi_paths.py`、`cluster_behavior_diffs.py`
  - 部署：`oa4rust/deploy/shadow-traffic.sh`、`toggle_module.sh`、`rollback-playbook.md`
  - 种子：`tests/behavior_comparison/seeds/`

---

## Execution Guidance

**立即启动（三条并行线）**：
1. U1、U2（CI 基础设施）——解除后续依赖
2. U13（零测试 crate 补测）——测试覆盖率基线提升
3. U4-U8（R500/R401/R403/R405/R415 修复）——快速消除 ~178 条 FAIL

**Phase 1 滚动顺序**：U9 → U10 → U11 → U12（Stub 类优先 → Express POST → 深层逻辑 → 信封收尾）

**Phase 2 并行独立**：U14 → U15 → U16（核心模块 → 中优先级 → CI 门禁）

**Phase 3 接入条件**：Phase 1 主干收敛后（FAIL ≤ 500）启动 U17、U18

**Phase 4 贯穿全程**：U20、U21 随代码进展迭代；U22 最终收敛

**R1/U23 独立推进**：代码层交付不等待运维排期，U19 演练提前扫障
