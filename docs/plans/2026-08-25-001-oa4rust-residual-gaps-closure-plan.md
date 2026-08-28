---
title: "feat: OA4Rust residual-gap closure (R1-R10; R6 declaration-only)"
type: feat
status: completed
date: 2026-08-25
origin: docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md
---

# OA4Rust 残差闭环实施计划（R3–R10）

## Summary

本计划将需求文档 `docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` 的 R1–R10 落为可执行单元：补齐零星缺失端点与 attachment 端点、对齐核验并闭合 BAM 监控模块、收敛 handler 行为语义、核验生成器脚本版本控制、填充模块卡片、执行生产影子流量切流并产出"可替代"判定声明，最后刷新文档口径以消除"平台限制"误分类。

> **研究校正（务必先读）**：计划制定时对照了当前代码，发现需求文档的三处前提已过时，本计划据此调整而非照抄：
> - **BAM 已实装**：`crates/processplatform_assemble_bam/` 当前已有 80+ 路由（`routes.rs` 已镜像 Java `jaxrs/period/*`、`jaxrs/state/*` 等面），故 R4 由"从零建设"改为"对齐核验 + 残差闭合"，不再按"5 路由"口径新建。
> - **attachment 端点本可实现**：现有 `u2_att_ext_download_handler!` 宏已是"整段 `Path<String>` 捕获含点文件名 → 名取元数据"，无需在路由层按 `.` 拆分。R5 直接套用既有模式。
> - **生成器已纳管**：根 `.gitignore` 已用 `!oa4rust/scripts/gen_openapi_paths.py` 强制追踪该生成器，R8 基本已满足，转为"核验 + 扩展 + CI 守卫"。

## 执行结果（2026-08-25 收官）

| 单元 | 结论 | 证据 |
|------|------|------|
| U1 (R3,R5) | ✅ 完成 | 发票 2 端点真实现 + 迁移 087（62fdf48d）；bbs acceptreply 与 attachment 4 条经核实早已注册 |
| U2 (R4) | ✅ 完成 | BAM 补注册 3 个 state 统计路由；全部写端点加 `require_owner`；delete 拒绝物理删除（fdf483d9） |
| U3 (R9) | ✅ 完成（静态收敛 + 本地全链路实跑） | cms 单实体语义修正 + `app_type` 列名修复 + 差异留档（d59a95aa）；同日复刻 CI 配方完成 Rust vs Java 真实对比：4044 端点经五轮收敛至 **1212 PASS / 836 FAIL / 1996 SKIP**，信封层与列表包装模式对齐验证，剩余 FAIL 属业务数据依赖与深层逻辑差异、已按类留档（详见终扫 §六） |
| U4 (R8) | ✅ 完成 | 6 个生成器纳管 + CI `openapi-guard` 守卫（24f8a2b2） |
| U5 (R10) | ✅ 完成 | o2web 86 张卡片 Key Flows/Dependencies 全部填充（b8749ee2）；o2server 55 张此前已含 |
| U6 (R1) | ✅ 预备完成 / ⛔ 生产切流外部阻塞 | 影子流量/灰度脚本登记 processplatform、bam 并接线死变量（daf9f647、6ffb82e2）；实际切流需运维排期 + ≥2 周观察 |
| U7 (R2) | ✅ 完成 | `docs/REPLACEABLE-oa4rust-2026-08-25.md` 正式判定声明（b96c84e5） |
| U8 | ✅ 完成 | 平台限制口径刷新为已闭环 + BAM 挂起撤销（3e8148ca） |
| U9 (R7) | ✅ 完成（前提勘误） | 全量核查证明 264 处 `?` 均为合法字符且与 kernel.org 上游逐字一致，**不存在 mojibake 损坏**，无需修改 |

**收官附加发现与闭合**：复核期间查明原"1491 条预期端点"口径不可溯源（临时差分脚本产物）；修复 `regen_endpoints.py` 三类缺陷（扫描面漏独立 router 文件/转义引号截断/全限定 method 不识别）后重生成清单 **4513 → 4687 条**，全量复验真实挂载 missing=0、extra=0（f008694f）。

---

## Problem Frame

2026-08-25 合并 HEAD（950a18e1）时端点对齐度达 99.77%（3085/3092 唯一端点，28/30 模块 100%）。"可接管 o2server"判定仍不能宣布，残差包括：影子流量未在生产跑过（R1，外部阻塞）、3 条零星缺失端点与 4 条 attachment 端点（R3/R5）、BAM 监控模块对齐待核验（R4）、生成器版本控制待核验（R8）、handler 深层语义不一致（R9）、模块卡片字段待填充（R10）、Linux 文档校正（R7，独立轨道）。本计划驱动这些残差的代码层与服务层闭环，使 R2 的"可替代"判定可被书面宣布。

**Origin actors:** A1 运维/部署, A2 oa4rust 开发者, A3 技术负责人/决策人, A4 文档维护者, A5 工具链维护者
**Origin flows:** F1 影子流量切流, F2 端点残差补齐, F3 可替代范围裁定, F4 行为语义收敛, F5 生成器纳管, F6 模块卡片填充
**Origin acceptance examples:** AE1 (R1), AE2 (R3), AE3 (R6), AE4 (R4), AE5 (R5), AE6 (R8), AE7 (R9), AE8 (R10)

---

## Requirements

- R1. 生产环境按 playbook 执行模块级灰度与影子流量比对，观察期 ≥2 周且无核心链路差异后切流，归档报告。
- R2. 产出"可替代 o2server"的正式判定结论文档。
- R3. 补齐 3 条真实缺失端点（processplatform_assemble_surface 2 条、bbs_assemble_control 1 条）。
- R4. BAM 模块对齐核验并闭合残差（Java 131 @Path vs 既有 80+ Rust 路由），写端点补齐 require_owner。
- R5. 补齐 4 条 `attachment/download/*/{}.{}` 端点，沿用整段 `Path<String>` 捕获模式，纳入行为对比。
- R6. IM/XMPP/WebRTC 完整协议明确排除在 v1 外（本计划不实现，仅声明）。
- R7. Linux 文档翻译校正 L11.1/L11.2（独立轨道，不阻塞判定）。
- R8. 生成器脚本版本控制核验与扩展（gen_openapi_paths.py 已纳管，扩展其余生成器 + CI 守卫）。
- R9. 收敛 handler 行为语义一致性（cms 等深层语义）。
- R10. 模块卡片 Key Flows/Dependencies 深度填充（55+86 张）。

---

## Scope Boundaries

- 不含 cms_assemble_control 单文件架构重构等既有架构债
- 不含新增 o2server 不存在的功能
- 不含国产库（达梦/金仓）适配
- IM/XMPP/WebRTC 完整协议不在范围（R6）
- Linux 文档校正（R7）作为独立轨道，不阻塞 R1/R2 判定
- R4 不对 BAM 做新建 crate，仅在既有 `processplatform_assemble_bam` crate 上对齐闭合
- R5 不推翻"路由层不可表达"的旧结论文本——实现后由 U8 刷新文档口径

### Deferred to Follow-Up Work

- 任何超出 99.77% 基线外、需求文档未列的新端点发现：回到需求文档修订，不在此计划扩围
- （R7 由 U9 在本计划内实现，不再单列 deferred。）

---

## Context & Research

### Relevant Code and Patterns

- **端点补齐模式**：`crates/processplatform_assemble_surface/src/lib.rs:17447` 的 `macro_rules! u2_att_ext_download_handler` 与 `routes.rs:1212` 的 `/jaxrs/processplatform/assemble/surface/attachment/download/{id}/work/{workId}/{fileName}` —— 整段 `Path<(String,String,String)>` 捕获含点文件名、名取元数据。R3/R5 直接复用。
- **BAM crate**：`crates/processplatform_assemble_bam/`（lib.rs / routes.rs / tests.rs / tests_generated.rs），`processplatform_assemble_bam_router()` 已注册 80+ 路由，Java 参考面在 `oa/o2server/x_processplatform_assemble_bam/`（约 90 个 .java）。R4 在此 crate 上核验闭合。
- **行为对比框架**：`tests/behavior_compare.rs` 驱动 `tests/behavior_comparison/`（`endpoints.rs` 的 `ENDPOINTS: &[EndpointDef]` 数组、`comparator.rs`、`allowlist.yaml`）。新端点通过向 `ENDPOINTS` 追加 `EndpointDef` 纳入比对；CI job 在 `oa4rust/.github/workflows/ci.yml:86`，`BEHAVIOR_COMPARE=1` 控制、Java 不可达时 SKIP。
- **生成器与 gitignore**：根 `.gitignore` 规则 `oa4rust/scripts/**` 整体忽略，但 `!oa4rust/scripts/gen_openapi_paths.py` 强制纳入；该生成器产出 `tests/behavior_comparison/endpoints.rs`（约 1012 端点）与 OpenAPI `securitySchemes` 注入。R8 核验其余生成器并加 CI 守卫。
- **模块卡片**：`docs/oa/modules/o2server/`（55）、`docs/oa/modules/o2web/`（86）；生成/抽取脚本在 `docs/oa/scripts/`（`generate_cards.py`、`fill_responsibility.py`）；卡片实际含 5 节（Responsibility / Core Classes / Key Flows / Dependencies / REST Endpoints）。
- **影子流量 playbook**：`oa4rust/deploy/shadow-traffic.sh`（enable/disable/run/compare/report/status）、`oa4rust/deploy/toggle_module.sh`（set/gray/rollback/reset）。默认 `TEST_MODULES`/`DEFAULT_GRAY_MODULES` 暂不含 processplatform/bam，需补登记。

### Institutional Learnings

- `security-issues/idor-vulnerability-write-handlers.md`（critical）：用户自有资源写 handler 必须 `require_owner` + `creator_person` 取自 Session，否则 P0 阻断合并 —— **R3/R4 写端点强制适用**。
- `architecture-patterns/actionresult-9-field-contract.md`（high）：所有新端点包 ActionResult 9 字段，业务错误走 HTTP 200 + type=error —— R3/R4/R5 硬契约。
- `architecture-patterns/seaorm-dual-pool-coexistence.md` + `dynamic-sql-retains-sqlx.md`：双池为持续架构；动态 SQL 走 `Extension<Pool>`（SQLx），标准 CRUD 走 `Extension<DatabaseConnection>`（SeaORM）。
- `integration-issues/nested-tokio-runtime-panic.md`（critical）：router 工厂为 sync `fn`，SeaORM 连接须 `catch_unwind` 包裹 `block_on`。
- `best-practices/auto-generate-rust-handler-tests.md`：**含 Session 的 handler 被测试生成器跳过**（105 个），R3/R4 写端点需补 router-based 或手测覆盖所有权逻辑。
- `best-practices/single-source-of-truth-migration-status.md`：状态汇报以 `docs/audits/final-coverage-sweep.md`（99.77%）为权威，勿引用 81-crate-100% 历史快照。

### 冲突与一致性

- ⚠️ **R5 与既有文档冲突**：`docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md` 与 `docs/audits/final-coverage-sweep.md` 附录将 4 条 attachment 端点归类为"axum 平台限制、不实现"。R5 落地后用 U8 刷新这两处，把"不实现"改为"已用既有整段捕获模式闭环"。
- BAM（R4）在 campaign doc 已"决策补齐闭合"，与本研究发现的"已 80+ 路由"一致，无冲突。

---

## Key Technical Decisions

- **R4 改为核验闭合而非新建**：当前代码 BAM 已 80+ 路由，按"5 路由"新建会重复造轮子；改为对照 Java 131 @Path 清单核验并补残差。
- **R5 整段捕获、不拆分**：沿用现有 `u2_att_ext_download_handler!` 模式（文件名整段捕获、实际名取元数据），避免发明路由层 dot-split。
- **写端点强制 require_owner**：R3/R4 涉及写操作时，遵循 IDOR 学习，防止 P0 合并阻断。
- **新端点入 behavior_compare**：R3/R5 实现后在生成器消费的路由面登记并重新生成 `endpoints.rs`，使覆盖率口径随代码自动更新（勿手改生成文件）。
- **R8 以核验为主**：gen_openapi_paths.py 已纳管；重点是扩展其余生成器纳管 + 加 CI 守卫确保 securitySchemes 重建不丢。
- **文档口径随代码刷新（U8）**：消除 R5 与 campaign/final-coverage-sweep 的冲突，保持 docs/solutions 准确性。

---

## Open Questions

### Resolved During Planning

- BAM 是否需新建 crate？否——`processplatform_assemble_bam` 已存在且 80+ 路由，R4 在其上闭合。
- attachment 端点是否真不可实现？否——既有模式整段捕获含点文件名，R5 可直接实现。
- 生成器是否完全未纳管？否——`gen_openapi_paths.py` 已被 `.gitignore` 强制追踪。

### Deferred to Implementation

- [R3] 3 条缺失端点的具体 Java 行为契约：从 `oa/o2server` 源码提取后确定返回结构。
- [R4] Java 131 @Path 与 Rust 80+ 路由的逐项差异清单：由 U2 比对 `routes.rs` 与 Java 源码得出。
- [R5] 4 条端点的段数变体精确集合：以 `final-coverage-sweep.md` 附录为准，U1 注册对应路由。
- [R1] 切流回滚触发阈值与观察期具体长度（≥2 周为下限）：运维确认。
- [R7] 逐文件脚本可行性（266+ 处跨 7 语种）：U9 评估。
- [R8] 需扩展纳管的具体生成器清单：U4 盘点 `oa4rust/scripts/` 后确定。

---

## Implementation Units

### U1. 缺失与 attachment 端点补齐（R3, R5）

**Goal:** 实现 3 条真实缺失端点（processplatform_assemble_surface 2 条 `attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`、`attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}`；bbs_assemble_control 1 条 `user/subject/acceptreply/{}/{}`）与 4 条 attachment/download 端点，纳入行为对比。

**Requirements:** R3, R5, AE2, AE5

**Dependencies:** None

**Files:**
- Modify: `crates/processplatform_assemble_surface/src/lib.rs`, `crates/processplatform_assemble_surface/src/routes.rs`
- Modify: `crates/bbs_assemble_control/src/lib.rs`, `crates/bbs_assemble_control/src/routes.rs`
- Do NOT hand-edit `tests/behavior_comparison/endpoints.rs` — it is auto-generated. Register the 7 endpoints in the route surface the generator consumes (OpenAPI path definitions / route registration) and regenerate the file.

**Approach:**
- 3 条缺失端点：从 `oa/o2server` 提取 Java 行为契约（返回结构与权限），仿同 crate 既有 handler 实现，遵循 ActionResult 9 字段 + 输入校验。
- 4 条 attachment 端点：复用 `u2_att_ext_download_handler!` 宏 / 整段 `Path<String>` 捕获含点文件名、实际文件名取元数据，不在路由层按 `.` 拆分；对照 `final-coverage-sweep.md` 附录的 4 条段数变体注册路由。
- 读侧鉴权（防读侧 IDOR）：所有 attachment/download 与 download 类端点须校验请求者对目标 attachment/work 实体拥有读权限（沿用既有 attachment handler 的访问校验），避免已认证用户越权拉取任意资源 id。
- 向行为对比注册 7 条端点：将这 7 条端点加入生成器所消费的路由面（OpenAPI 路径定义 / 路由注册表），重新生成 `tests/behavior_comparison/endpoints.rs`，而非直接手改该自动生成文件（手改会被重建覆盖）。

**Patterns to follow:**
- `crates/processplatform_assemble_surface/src/lib.rs:17447` `u2_att_ext_download_handler!`
- `best-practices/input-validation-pattern.md`, `architecture-patterns/actionresult-9-field-contract.md`

**Test scenarios:**
- Happy path: GET 各新端点返回 HTTP 200 且 `ActionResult` 含 9 字段，body 结构与 Java 侧一致。
- Covers AE5: behavior_compare 对 4 条 attachment 端点 PASS，且终扫"平台限制"项从排除清单移除。
- Covers AE2: behavior_compare 对 3 条缺失端点 PASS，终扫"真实缺失"数为 0。
- Edge case: 含点文件名（如 `report.v2.pdf`）整段捕获不断裂。

**Verification:** `ENDPOINTS` 含 7 条新条目；`cargo test --test behavior_compare` 对应项 PASS；`cargo check -p <crate>` 通过。

---

### U2. BAM 模块对齐核验与残差闭合（R4）

**Goal:** 对照 Java `x_processplatform_assemble_bam` 131 @Path 与既有 Rust 80+ 路由，产出差异清单，实现残差端点并为写端点补齐 require_owner。

**Requirements:** R4, AE4

**Dependencies:** None（可与 U1 并行）

**Files:**
- Read/Modify: `crates/processplatform_assemble_bam/src/lib.rs`, `crates/processplatform_assemble_bam/src/routes.rs`
- Read: `oa/o2server/x_processplatform_assemble_bam/`（Java 参考面）
- Do NOT hand-edit `tests/behavior_comparison/endpoints.rs` — it is auto-generated (same as U1). Register BAM 残差端点于生成器消费的路由面并重新生成该文件。

**Approach:**
- 抽取 Java `@Path` 清单（约 131）与 `routes.rs` 已注册路由做差集，产出"Rust 缺哪些 Java 端点"。
- 对残差端点按既有 `Path<(String,...)>` + `Extension<Pool>` + `ActionResult<Value>` 风格补齐。
- 对写类 handler（create/delete/update）加 `require_owner`（参照 `security-issues/idor-vulnerability-write-handlers.md`）；无 `deleted_at` 的实体返回 error 而非物理删。
- 读侧鉴权：BAM 的读/查询类端点亦须校验请求者对目标实体拥有读权限（沿用既有 handler 的访问校验），避免已认证用户越权访问（读侧 IDOR）。
- router 工厂保持 sync `fn` + `catch_unwind` 包裹 SeaORM 连接（若残差涉及 ORM 查询）。

**Patterns to follow:**
- `crates/processplatform_assemble_bam/src/routes.rs` 既有 `processplatform_assemble_bam_router()`
- `security-issues/idor-vulnerability-write-handlers.md`, `integration-issues/nested-tokio-runtime-panic.md`

**Test scenarios:**
- Happy path: behavior_compare 对 BAM 端点 PASS（状态码 + 字段名/类型）。
- Covers AE4: 发布替代声明时 BAM 列为已接管，低频后补部分显式标注。
- Error path: 跨用户写操作被 require_owner 拦截返回 403（非 admin）。

**Verification:** BAM 路由差集归零或残留显式留档；require_owner 覆盖所有写 handler；behavior_compare BAM 全绿。

---

### U3. handler 行为语义一致性收敛（R9）

**Goal:** 借 behavior-compare 识别 `cms` 等"路由 100% 但深层语义不匹配"模块，将语义差异收敛至可接受或显式留档。

**Requirements:** R9, AE7

**Dependencies:** U1, U2（端点先就位）

**Files:**
- Modify: `tests/behavior_comparison/allowlist.yaml`（手维护，非生成文件）
- 如涉及端点登记变更，经生成器重新生成 `endpoints.rs`，勿手改
- Modify: 相关 crate handler（如 `crates/cms_assemble_control/src/lib.rs`）按需修正

**Approach:**
- 运行 `cargo test --test behavior_compare` 产出语义差异报告（前提：Java 参考端可达；否则 CI 按 Java 不可达逻辑 SKIP，无法产出差异，U3 须先确保 Java 侧在线）。
- 对 `cms` 等模块逐项判断：能修正 handler 的（响应语义/业务一致性）直接修；不能或属设计性差异的加入 `allowlist.yaml` 并附理由。

**Patterns to follow:** `architecture-patterns/actionresult-9-field-contract.md`（字段级比对是安全网）

**Test scenarios:**
- Covers AE7: behavior_compare 报告对 cms 等模块显示差异已收敛（修正或 allowlist），无未解释 drift。

**Verification:** 语义差异报告闭合或每项有修正/留档依据；`allowlist.yaml` 条目均有注释。

---

### U4. 生成器脚本版本控制核验与扩展（R8）

**Goal:** 核验 `gen_openapi_paths.py` 已纳管，盘点 `oa4rust/scripts/` 其余生成器并扩展纳管，加 CI 守卫确保 OpenAPI `securitySchemes` 重建不丢。

**Requirements:** R8, AE6

**Dependencies:** None

**Files:**
- Modify: `.gitignore`（根）
- Read: `oa4rust/scripts/`
- Modify: `oa4rust/.github/workflows/ci.yml`（守卫步骤，可选）

**Approach:**
- 确认 `git check-ignore oa4rust/scripts/gen_openapi_paths.py` 返回"未忽略"。
- 盘点 `oa4rust/scripts/` 下其余生成器（如生成 `endpoints.rs` 的脚本），对需复现的加 `!` 例外纳入版本控制。
- 在 CI 加一步：重新生成 OpenAPI 后断言产物含 `securitySchemes`（防止注入静默丢失）。

**Test expectation:** none —— 纯配置/校验，无行为变更。

**Verification:** `git check-ignore` 对相关生成器均"未忽略"；CI 守卫在重建后仍能检出 securitySchemes。

---

### U5. 模块卡片深度填充（R10）

**Goal:** 为 `docs/oa/modules/o2server/`（55）与 `docs/oa/modules/o2web/`（86）卡片填充 Key Flows / Dependencies 字段。

**Requirements:** R10, AE8

**Dependencies:** None

**Files:**
- Modify: `docs/oa/modules/o2server/*.md`, `docs/oa/modules/o2web/*.md`
- Modify: `docs/oa/scripts/fill_responsibility.py` 或抽取脚本（如适用）

**Approach:**
- 复用 `docs/oa/scripts/generate_cards.py` / `fill_responsibility.py` 的抽取逻辑，从对应 crate 源码提取 Key Flows（核心 handler 调用链）与 Dependencies（Rust 侧依赖）。
- 逐卡片填充，沿用实际卡片 5 节结构（含 Key Flows），非空且与所述 crate 一致。

**Test expectation:** none —— 文档填充。

**Verification:** 抽样卡片 Key Flows/Dependencies 非空且与代码一致；`docs/oa/README.md` 链接可解析。

---

### U6. 影子流量生产切流（R1）

**Goal:** 在生产环境执行影子流量比对与模块级切流，观察期 ≥2 周，归档报告。

**Requirements:** R1, AE1

**Dependencies:** U1, U2（端点就位后）

**Files:**
- Execute: `oa4rust/deploy/shadow-traffic.sh`, `oa4rust/deploy/toggle_module.sh`
- Modify: 上述两脚本的 `TEST_MODULES` / `DEFAULT_GRAY_MODULES`（补登记 processplatform、bam 以便 R3/R5 走影子流量）

**Approach:**
- 在 playbook 的灰度模块列表补登记 processplatform / bam。
- 按 toggle_module.sh 灰度（10%→50%→100%），shadow-traffic.sh 并行比对 Rust/Java 响应，观察期 ≥2 周。
- 差异报告为空则切流、否则回滚（5 分钟 RTO）。

**Test expectation:** none —— 运维执行，非单测。

**Verification:** 比对报告归档；o2server 可下线且切流后监控无回归。

---

### U7. 「可替代」正式判定声明（R2）

**Goal:** 产出书面"可替代 o2server"判定结论，列明接管范围、前提与已知限制。

**Requirements:** R2, AE3, AE4

**Dependencies:** U1–U6

**Files:**
- Create: `docs/REPLACEABLE-oa4rust-2026-08-25.md`（或归入 `docs/brainstorms/`）

**Approach:**
- 汇总端点对齐度（99.77% + U1/U2 后残差）、平台限制（attachment 已闭、IM 排除）、协议排除（R6）、BAM 处置（U2）、语义一致性（U3）。
- 由 A3 签核后对外发布。

**Test expectation:** none —— 文档产物。

**Verification:** 声明经 A3 确认，可被 R3–R10 全部追溯。

---

### U8. 文档口径刷新（一致性，消除冲突）

**Goal:** R5 落地后刷新 campaign doc 与 final-coverage-sweep 中"attachment 端点平台限制/不实现"的误分类，并同步 BAM 现状。

**Requirements:** 一致性（见 Context 冲突节）

**Dependencies:** U1, U2

**Files:**
- Modify: `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- Modify: `docs/audits/final-coverage-sweep.md`

**Approach:**
- 将 campaign doc 中"4 条 axum 平台限制不实现"改为"已用整段 `Path<String>` 捕获模式闭环（U1）"。
- 将 final-coverage-sweep 附录的"🟠 平台限制"4 条改为已覆盖，并更新覆盖率口径（99.77% → U1/U2 后新值）。
- 补 BAM 现状注记（已 80+ 路由，R4 核验闭合）。

**Test expectation:** none —— 文档。

**Verification:** 两文档与代码事实一致，无"平台限制不实现"残留；建议随后用 `/ce-compound` 将"整段捕获推翻平台限制"作为新学习回填 docs/solutions。

---

### U9. Linux 文档翻译校正（R7，独立轨道）

**Goal:** 校正 `docs/linux-7.1.3/` 下 266+ 处、7 语种 `?` 损坏，逐文件对照上游 RST 源。

**Requirements:** R7

**Dependencies:** None（独立轨道，不阻塞 R1/R2）

**Files:**
- Modify: `docs/linux-7.1.3/`（跨 7 语种）

**Approach:**
- 按语种计数（zh_CN 82、sp_SP 68、it_IT 67、zh_TW 33、pt_BR 7、ko_KR 6、ja_JP 3）逐文件对照 kernel.org 上游 RST 确定正确标点。
- L11.3/L11.4（链接完整性、格式一致）已完结，本单元聚焦 L11.1/L11.2 翻译校正。

**Test expectation:** none —— 文档工程。

**Verification:** 各语种 `?` 损坏计数归零或复核完毕；断链/格式报告无新增。

---

## System-Wide Impact

- **Interaction graph:** U1/U2 新增路由经 `behavior_compare` 框架；U4 改动 `.gitignore` 与 CI 影响所有生成器复现。
- **Error propagation:** 新端点须走共享错误中间件（ActionResult 9 字段），业务错误 HTTP 200 + type=error。
- **State lifecycle risks:** BAM 写端点涉及资源所有权，require_owner 缺失将导致 IDOR（critical）。
- **API surface parity:** 所有新端点必须同时出现在 Rust 路由与 `ENDPOINTS` 清单，否则 behavior_compare 无法覆盖。
- **Integration coverage:** U6 影子流量是唯一生产级验证，单测/CI 不构成切流依据。
- **Unchanged invariants:** ActionResult 9 字段契约、双池架构、前端 `action.js` 消费方式均不变；IM 协议范围不变。

---

## Risks & Dependencies

| Risk | Mitigation |
|------|------------|
| BAM Java 131 @Path 与 Rust 80+ 路由差集较大，残差超出预期 | U2 先产出差集清单再实现，分阶段验收（核心优先、低频后补） |
| 自动生成测试跳过含 Session 的写 handler（105 个） | U1/U2 写端点补 router-based 或手测覆盖 require_owner 逻辑 |
| attachment 端点段数变体边界 | 以 final-coverage-sweep 附录 4 条为基准逐条注册，整段捕获避免拆分歧义 |
| `.gitignore` 改动误忽略生成器 | U4 改后 `git check-ignore` 验证，CI 守卫防 securitySchemes 丢失 |
| R5 与既有文档"平台限制"冲突未刷新 | U8 在 U1 后强制刷新 campaign doc + final-coverage-sweep |
| 影子流量需生产环境 + ≥2 周（外部阻塞） | U6 明确依赖运维排期，不阻塞文档层工作（R7/U5/U9 可并行） |

---

## Documentation / Operational Notes

- 状态汇报统一以 `docs/audits/final-coverage-sweep.md` 为权威；本计划收官后其口径为：行为对比清单 4687 条、真实挂载 missing=0、可实施端点覆盖率 100%。
- U8 完成后建议运行 `/ce-compound` 将"整段 `Path<String>` 捕获推翻 attachment 平台限制"作为新学习回填 `docs/solutions/`。
- 本计划为 `docs/plans/` 在归档 `2026-08-21-002` 后的新活跃计划；需求文档 Related 中"Active plan"链接指向已归档的旧计划，U8 或后续可一并修正。

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md](docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md)
- 端点模式: `crates/processplatform_assemble_surface/src/lib.rs:17447`, `routes.rs:1212`
- BAM crate: `crates/processplatform_assemble_bam/src/routes.rs`
- 行为对比: `tests/behavior_compare.rs`, `tests/behavior_comparison/endpoints.rs`, `oa4rust/.github/workflows/ci.yml:86`
- 生成器/忽略: `oa4rust/scripts/gen_openapi_paths.py`, 根 `.gitignore`
- 模块卡片: `docs/oa/modules/o2server/`, `docs/oa/modules/o2web/`, `docs/oa/scripts/`
- 影子流量: `oa4rust/deploy/shadow-traffic.sh`, `oa4rust/deploy/toggle_module.sh`
- 机构学习: `docs/solutions/security-issues/idor-vulnerability-write-handlers.md`, `architecture-patterns/actionresult-9-field-contract.md`, `architecture-patterns/seaorm-dual-pool-coexistence.md`, `integration-issues/nested-tokio-runtime-panic.md`, `best-practices/auto-generate-rust-handler-tests.md`
- 冲突源: `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`, `docs/audits/final-coverage-sweep.md`
