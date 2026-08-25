# OA4Rust 可替代 o2server —— 正式判定声明

- **文档编号**：REPLACEABLE-oa4rust-2026-08-25
- **生成日期**：2026-08-25
- **生成方**：本仓库 parity 计划 U7（对应需求 `docs/plans/2026-08-25-001-oa4rust-residual-gaps-closure-plan.md` 之 R2）
- **签核对象**：技术负责人（A3）
- **判定性质**：**有条件判定** —— 端点级与模块级"可接管"成立；**完全接管（关闭 Java 侧）需先满足 R1 生产影子流量前提**。
- **权威依据**：`docs/audits/final-coverage-sweep.md`（端点对齐终态基准，generated_at=2026-08-23，2026-08-25 复核收口）、`docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`、`docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md`
- **当前代码基线**：仓库 HEAD = `a4aeb065`（parity 计划合并时基于 `950a18e1` 的 99.77% 口径，后续复核已闭合其 3 个残留缺口）

---

## 一、判定结论摘要

> **结论**：在不包含 IM/XMPP/WebRTC 完整协议（R6，v1 范围外）的前提下，OA4Rust 对 o2server 的可实施端点覆盖率为 **100%**（30 个有端点的 Java 模块组中 28 个达 100%，其余 2 个的残留缺口已于 2026-08-25 复核全部闭合，4 条 axum 单段多参数端点按原规则排除不计入）。
>
> 据此，**OA4Rust 已可在端点与模块层面正式替代 o2server 承接流量**。
>
> **唯一外部阻塞项**：R1（生产环境影子流量灰度比对，观察期 ≥2 周，无核心链路差异后切流）。该项脚本与 playbook 已就绪，但需在真实生产环境执行，目前尚未运行。满足 R1 后方可宣布"完全接管并关闭 Java 侧"。

关键数字（均来自上述权威文件，非估算）：

| 指标 | 数值 | 来源 |
|------|------|------|
| 唯一端点覆盖（基线） | 3085 / 3092 = 99.77% | `final-coverage-sweep.md` §一 |
| 100% 覆盖模块数（基线） | 28 / 30 | 同上 |
| 2026-08-25 复核后残留缺口 | 0（3 个已闭合） | 同上 §六 |
| 可实施端点覆盖率（复核后） | **100%** | 同上 §六 |
| 行为对比期望端点重扫 | missing = 0 | 同上 §六（基于 `tests/behavior_comparison/endpoints.rs`） |
| BAM 模块路由数 | 88 条 `.route(` 注册（≥80） | `crates/processplatform_assemble_bam/src/lib.rs` |
| attachment 平台限制排除项 | 4 条 `{}.{}` 单段多参数 | `final-coverage-sweep.md` 附录 |

---

## 二、端点对齐度

### 2.1 基线口径

依据权威终扫 `docs/audits/final-coverage-sweep.md`（生成于 2026-08-23 23:17，扫描 `crates/*/src/**/*.rs` 共 4573 条 `.route(` 注册、4155 条唯一归一化路径）：

- **唯一端点口径**（模块内去重合计）：Java 3092 个，已覆盖 3085 个，**覆盖率 99.77%**。
- **达到 100% 的模块**：28 / 30（30 为含 JAXRS 端点的 Java 模块组）。
- 匹配口径：路径参数归一化为 `{}`；`method + 全路径 exact（允许 Rust 侧更长前缀）∪ casefold` 计入覆盖；`verb_mismatch` / `literal_shift` 仅诊断不计入（影子副本会真实 404）。双分母（注解口径 vs 唯一端点口径）不得混用，终扫以**唯一端点口径**为权威。

### 2.2 2026-08-25 复核闭合 3 个残留缺口

终扫原列 3 个"排除留档后剩余缺口"，本次复核**逐条闭合**，可实施端点覆盖率回到 **100%**：

1. **processplatform 2 条发票端点**
   - `/attachment/invoice/{}/jobOrWorkOrWorkCompleted/{}`
   - `/attachment/download/invoice/{}/jobOrWorkOrWorkCompleted/{}`
   - 原为 `u2_capability_unavailable` 桩，已在提交 **`62fdf48d`**（`feat(processplatform): 实现 invoice 文档信息/下载端点（替换 capability 桩）+ 迁移 087`）替换为真实 handler，并由迁移 `087_add_invoice_storage_columns.sql` 补齐 `x_general_invoice` 的 StorageObject 列。
2. **bbs `user/subject/acceptreply/{}/{}`**
   - 经 `grep` 核实早已注册于 `bbs_assemble_control/src/routes.rs`，此前扫描为假阴性（链式路由提取缺陷，见 2.3）。

### 2.3 根因修复与重扫口径

- 旧路由提取逻辑对链式写法 `.route("p", get(a).put(b))` 只识别首个 method，导致靠后 PUT/DELETE 误判缺失。已修正 `oa4rust/scripts/extract_routes.py`（对 `.route(` 整段做平衡括号提取，扫出全部 `get/post/put/delete`），PUT/DELETE 类端点由 0 → 662。
- 基于 `tests/behavior_comparison/endpoints.rs`（行为对比期望端点清单）在链式路由拆分后重扫，**missing = 0**。
  - 注：当前仓库内该文件经统计含 **4510 条 `EndpointDef`**（终扫 §六 记录口径为 1491 条 java 对齐期望端点；差异源于生成器后续重跑/expand，不影响 missing=0 结论）。此计数差异建议回填刷新（见第九节"事实缺口"）。
- `extract_routes.py` 已自 `.gitignore` 放开纳入版本控制（与 `gen_openapi_paths.py` 一并强制追踪），根因修复可随仓库共享。

---

## 三、平台限制与范围排除

### 3.1 attachment 4 条单段多参数端点（axum 平台限制，按原规则排除不实现）

以下 4 条属 axum 框架**单段多参数**（`{}.{}` 段）不可表达，按终扫原规则**留档排除、不实现**，不计入回归覆盖：

| 模块 | 方法 | 路径 |
|------|------|------|
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/stream/{}.{}` |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/work/{}/{}.{}` |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/stream/{}.{}` |
| `x_processplatform_assemble_surface` | GET | `/attachment/download/{}/workcompleted/{}/{}.{}` |

> 说明：既有 `u2_att_ext_download_handler!` 宏采用整段 `Path<String>` 捕获含点文件名、名取元数据，理论上可绕过路由层拆分；但本判定依**原规则与终扫口径**将其维持为"平台限制排除项"，不列入 v1 实现范围。若后续决定补齐，应同步以 U8 刷新 `final-coverage-sweep.md` 与其余 docs 的"平台限制"分类，避免口径冲突。

### 3.2 IM / XMPP / WebRTC 完整协议（R6，设计性范围外）

即时通讯完整协议（IM/XMPP/WebRTC）依 R6 明确**排除在 v1 接管范围外**（沿用 2026-08-19-002 范围约束）。Rust 侧保留 WebSocket 基础广播与 `ImAction`×33 端点作为部分能力，而非完整协议替代。该排除属书面接受的范围边界，不影响"端点级可替代"判定。

---

## 四、BAM 业务活动监控模块（R4 核验闭合）

- `x_processplatform_assemble_bam`（Java 参考面约 90 个 `.java` / 131 `@Path`）在 Rust 侧 `crates/processplatform_assemble_bam/` 当前已注册 **88 条 `.route(`**（lib.rs，`processplatform_assemble_bam_router()`），达"80+ 路由"核验标准，R4 由"从零建设"转为"对齐核验 + 残差闭合"并已闭合。
- **写端点所有权**：R4 写端点强制适用 IDOR 防护（`docs/solutions/security-issues/idor-vulnerability-write-handlers.md`，critical）——用户自有资源写 handler 必须 `require_owner` + `creator_person` 取自 Session，否则 P0 阻断合并。

---

## 五、handler 行为语义一致性（R9 现状与留档）

- **路由层**：cms 等模块路由层面已 100%（cms 437/437），终扫无"cms 语义不匹配"排除项（留档明细为空）。
- **深层语义层**：R9 关注路由 100% 之外的 handler 深层语义（响应语义 / 业务一致性），属行为对比框架（`behavior-compare` CI job + `oa4rust/tests/behavior_compare.rs`）持续比对范畴。
  - 已知模块：`cms` 等存在"深层语义不匹配"候选，经行为对比框架识别后逐模块收敛或显式留档（`allowlist.yaml` 附理由）。
  - **当前状态**：R9 为**持续收敛项**，尚未宣告全量语义零差异；但其残留差异已纳入显式留档机制，不阻断端点级"可替代"判定。语义级结论将随 R1 影子流量报告一并归档。

---

## 六、生成器纳管（R8）与文档口径（U8）状态

### 6.1 生成器纳管（R8）

- 根 `.gitignore` 规则 `oa4rust/scripts/**` 整体忽略，但已用否定规则强制纳入：
  - `!oa4rust/scripts/gen_openapi_paths.py`（OpenAPI 路径生成，注入 `securitySchemes`）
  - `!oa4rust/scripts/extract_routes.py`（路由提取，2026-08-25 根因修复后放开）
- R8 现状：**生成器已纳入版本控制**，下阶段为扩展其余生成器纳管 + CI 守卫，确保 OpenAPI 注入重建不静默丢失。

### 6.2 文档口径（U8）

- **权威口径已刷新**：`docs/solutions/best-practices/single-source-of-truth-migration-status.md` 于 2026-08-25 刷新，将"当前权威端点对齐状态"改指 `docs/audits/final-coverage-sweep.md`（99.77% 基线 + 残差清单），其 2026-08-10 "81 crate 100%" 示例标记为历史快照不再引用。
- `final-coverage-sweep.md` 为唯一端点对齐收口文件，后续不再重新生成度量（见 parity-closure-campaign doc "权威收口文件"）。
- 若后续按 3.1 说明补齐 attachment 4 条，应以 U8 同步刷新相关 docs 的"平台限制"分类，保持口径一致。

---

## 七、已知限制与前提（R1 外部阻塞）

### 7.1 唯一阻断"完全接管"宣告的项

- **R1（生产影子流量灰度验证）**：需在真实生产环境按 playbook 执行模块级灰度 + 影子流量比对（Rust 与 Java 响应一致性：状态码、响应体大小、内容摘要），观察期 **≥2 周**且无核心链路差异后切流并归档报告。
- 脚本与支柱已就绪：`oa4rust/deploy/shadow-traffic.sh`（enable/disable/run/compare/report/status）、`oa4rust/deploy/toggle_module.sh`、`docs/gray-release-playbook.md`、行为对比中间件（`X-Behavior-Comparison` 头）。
- **现状**：R1 属**外部阻塞**，尚未在生产环境运行。当前所有模块的"完全接管"判定均受此前提约束。

### 7.2 尚未跑生产影子流量的模块

影子流量 harness（`shadow-traffic.sh` 的 `TEST_MODULES`）当前预定覆盖以下模块组，**均尚未在生产环境跑过影子流量**：

- `attendance`（考勤）
- `control`（组织控制）
- `express`（组织快递/express）
- `meeting`（会议）
- `processplatform`（流程平台，含 surface/bam/designer）
- `bam`（业务活动监控）

> 上述为 harness 内已登记的首批验证面；`docs/audits/final-coverage-sweep.md` 覆盖的其余 28/30 模块同样需在 R1 切流前纳入影子流量比对范围。R1 完成并归档后方可宣布"完全接管、可关闭 Java 侧"。

---

## 八、接管范围、前提与已知限制（可追溯 R1–R10）

### 8.1 接管范围（v1）

- 30 个有 JAXRS 端点的 Java 模块组中，**28 个 100% 覆盖**，另 2 个（processplatform_assemble_surface、bbs_assemble_control）残留缺口已于 2026-08-25 闭合；可实施端点覆盖率 **100%**（不含 4 条 axum 平台限制端点）。
- 含 BAM 监控模块（R4 核验闭合，88 路由）。
- 不含 IM/XMPP/WebRTC 完整协议（R6 范围外）。

### 8.2 接管前提

- **P1（硬前提）**：R1 生产影子流量比对通过（≥2 周观察期、无核心链路差异、报告归档）。未满足前，结论限定为"端点/模块级可承接流量"，不宣布"完全接管并关闭 Java 侧"。
- **P2（口径前提）**：所有覆盖数字以 `final-coverage-sweep.md` 为唯一权威，不引用历史快照数字。

### 8.3 已知限制（可追溯 R1–R10）

| 编号 | 限制 / 前提 | 类别 | 可追溯 |
|------|------------|------|--------|
| R1 | 生产影子流量未跑，完全接管待验证 | 外部阻塞 | §七 |
| R3 | 3 条零星端点（已闭合，2026-08-25） | 已闭合 | §2.2 |
| R4 | BAM 核验闭合，写端点 require_owner | 已闭合 | §四 |
| R5 | attachment 端点按原规则排除不实现 | 平台限制 | §3.1 |
| R6 | IM/XMPP/WebRTC 完整协议排除 v1 | 设计性范围外 | §3.2 |
| R7 | Linux 文档翻译校正（独立轨道，不阻塞） | 独立工程 | 独立轨道 |
| R8 | 生成器纳管（已纳入版本控制，CI 守卫待补） | 已基线满足 | §6.1 |
| R9 | handler 深层语义持续收敛/留档中 | 持续项 | §五 |
| R10 | 模块卡片 Key Flows/Dependencies 深度填充（55+86 张） | 文档工作 | 需求 R10 |
| R2 | 本判定声明文档（U7） | 本文 | 全文 |

---

## 九、可追溯性矩阵（R1–R10 → 证据）

| R | 需求 | 状态 | 关键证据 |
|---|------|------|----------|
| R1 | 生产影子流量切流验证 | **待办（外部阻塞）** | `oa4rust/deploy/shadow-traffic.sh`、`toggle_module.sh`、`docs/gray-release-playbook.md`；尚未运行 |
| R2 | 可替代正式判定文档 | **完成（本文）** | `docs/REPLACEABLE-oa4rust-2026-08-25.md` |
| R3 | 3 条零星缺失端点 | **已闭合** | `final-coverage-sweep.md` §六；commit `62fdf48d`；bbs routes 假阴性核实 |
| R4 | BAM 核验闭合 + require_owner | **已闭合** | `crates/processplatform_assemble_bam/src/lib.rs`（88 `.route(`）；IDOR 学习 |
| R5 | attachment 4 条端点 | **按原规则排除不实现** | `final-coverage-sweep.md` 附录（平台限制） |
| R6 | IM/XMPP/WebRTC 排除 v1 | **声明排除** | `2026-08-25-001` 计划 R6；residual-gaps 需求 Key Decisions |
| R7 | Linux 文档校正 | **独立轨道** | 需求 R7，不阻塞判定 |
| R8 | 生成器纳管 | **已基线满足** | `.gitignore` 强制追踪 `gen_openapi_paths.py` + `extract_routes.py` |
| R9 | handler 语义一致性 | **持续收敛/留档** | `behavior-compare` CI + `behavior_compare.rs`；cms 候选留档 |
| R10 | 模块卡片深度填充 | **范围内文档工作** | 需求 R10（55+86 张） |

---

## 十、A3 签核建议与后续动作

1. **签核本判定**：在"端点/模块级可承接 o2server 流量"层面，依据 100% 可实施覆盖率与 R3/R4/R5/R6/R8 闭合证据，建议 A3 签核。
2. **R1 为放行闸门**：仅当生产影子流量报告（≥2 周、零核心差异）归档后，方可签署"完全接管、可关闭 Java 侧"。
3. **待回填（见下）**：endpoints.rs 计数与终扫口径（1491 vs 4510）需回填对齐；R9 语义留档清单应随 R1 报告一并归档。
4. **范围边界书面确认**：请 A3 书面确认 IM/XMPP/WebRTC 完整协议排除 v1、attachment 4 条平台限制排除为可接受范围边界。

---

### 附：事实缺口与需回填项

- **endpoints.rs 计数口径**：`final-coverage-sweep.md` §六 记为 1491 条 java 对齐期望端点；当前仓库 `tests/behavior_comparison/endpoints.rs` 经统计含 4510 条 `EndpointDef`。两者差异源于生成器后续重跑/expand，缺失结论（missing=0）不受影响，但**建议回填刷新终扫的计数口径**以消除歧义。
- **R9 语义留档清单**：cms 等模块的深层语义差异尚未形成量化留档表，建议随 R1 影子流量报告一并产出。
- **R10 模块卡片**：55+86 张模块卡片的 Key Flows/Dependencies 深度填充状态本判定未逐卡核验，列为范围内文档工作，不影响端点级结论。
