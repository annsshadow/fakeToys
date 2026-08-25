---
category: docs/solutions/best-practices/
module: "oa4rust"
date: 2026-08-25
problem_type: best_practice
component: development_workflow
severity: medium
applies_when:
  - "running a reimplementation parity campaign against a reference platform (e.g. Rust oa4rust vs Java o2server endpoints)"
  - "measuring endpoint or route coverage between two implementations and reconciling divergent metric denominators"
  - "consolidating scattered gap-closure plans and Deferred items into one single-source-of-truth plan"
  - "auditing plan-doc status drift before declaring a milestone (e.g. 99.77% parity) complete"
related_components:
  - testing_framework
  - documentation
  - tooling
tags:
  - oa4rust
  - o2server
  - endpoint-parity
  - coverage-measurement
  - gap-closure
  - single-source-of-truth
  - behavior-compare-ci
  - audit-consolidate-execute
---

# OA4Rust→o2server 缺口收敛战役的可复用实践

## Context

oa4rust（Rust 重写）长期以"crate 个数 / handler 数 / 测试通过率"作为替代 o2server（Java 参考实现）的进度信号，但这些信号存在系统性失真：同名路由 ≠ 同等功能，且"测试通过"在无 PostgreSQL 环境下只验证路由已注册、不验证数据与逻辑（证据：`docs/plans/2026-08-13-002-oa4rust-o2server-parity-analysis.md` §1.5；`docs/plans/2026-08-13-003-oa4rust-completion-plan.md` Phase 1）。同时，18 份历史计划的 frontmatter `status` 与代码库实际状态脱节（16 份中有 4 份仍标 `active`，2 份无 frontmatter），遗留工作散落在各计划的 Deferred 小节与 `docs/todo.md` 中，没有统一真相源（证据：`docs/plans/2026-08-21-001-feat-plans-status-audit-plan.md` Problem Frame）。这导致"能否替代"这一决策长期无法被书面、可验证地回答。

本战役的根本约束来自参考实现规模：o2server/o2web 合计约 **1M LOC / 8588 文件**，纯人工重写不可行，必须走"辅助 + 拆分"路线（session history：7/30 规模度量 `scale-measurements.md` 标记 oa 为"辅助/拆分"）。战役早期决策是"先文档化 Java 侧 → 再驱动 Rust 对齐"：2026-07-30 向 `feat/systems-documentation` 提交 780 个 Java 系统文档文件（`oa/o2web` 489 + `oa/o2server` 291），把 Java 侧结构作为对齐的单一事实来源基础（session history）。

## Guidance

**1. 端点对齐度量方法论（endpoint-alignment measurement）**
- 用 Java `@Path` 注解与 `@GET/@POST/@PUT/@DELETE` 方法静态提取 Java 端点，与 Rust 侧 `tests/behavior_comparison/endpoints.rs`（由 `scripts/gen_openapi_paths.py` 生成的 1012 个端点）及 `crates/*/src/**/*.rs` 的 `.route(` 注册做交叉比对（证据：`docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md` U4；`docs/audits/final-coverage-sweep.md` 第 4 行）。
- **路径参数归一化为 `{}`**，匹配口径为 `method + 全路径 exact（允许 Rust 侧更长前缀） ∪ casefold` 计入覆盖（证据：`docs/audits/final-coverage-sweep.md` 第 5 行）。
- **两类诊断项只诊断、不计入覆盖率**：`verb_mismatch`（路径存在但缺某 HTTP 方法变体）与 `literal_shift`（同段数形变候选，影子副本会真实 404）。原因：它们会产生"虚假未覆盖"噪声，不应拉低真实覆盖数字（证据：`docs/audits/final-coverage-sweep.md` 第 5 行、§五.1、§五.6）。
- **双分母报告，不混用**：注解口径（含变体与自有端点，约 4510/4386 ≈ 102.8%，见 `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` U2 行）与唯一端点口径（模块内去重，3085/3092 ≈ 99.77%，见 `docs/audits/final-coverage-sweep.md` §一）。两口径不可直接相比，终扫用唯一端点口径作为权威。
- **权威收口文件**：`docs/audits/final-coverage-sweep.md`（generated_at=2026-08-23）是端点对齐的终态基准，后续不再重新生成度量（证据：`docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` Scope Boundaries、Key Decisions）。
- **行为对比 CI 职位**：`BEHAVIOR_COMPARE=1` 环境变量控制 `oa4rust/.github/workflows/ci.yml` 的 `behavior-compare` job，拉起 o2server 容器 + 1000s 就绪探针 + postgres + u2_probe 冒烟；Java 不可达时 SKIP，Rust 不可达时 FAIL（证据：`docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` U9；`docs/plans/2026-08-20-001-feat-oa4rust-remaining-gap-closure-plan.md` R2/R3）。

**2. 缺口收口战役编排模式（campaign orchestration）**
四大阶段顺次推进，前序产出是后序输入：
- **审计（audit）**：对 18 份计划逐实施单元做静态验证（文件存在性、路由挂载、测试文件、git 历史、反向索引），原地更新三态 `status` 并追加"实现情况"小节——以代码实测为准修正状态（证据：`docs/plans/2026-08-21-001-feat-plans-status-audit-plan.md` U1–U3、Key Technical Decisions）。
- **汇总（consolidate）**：把三批审计发现的未竟单元 + 各 Deferred 小节 + `docs/todo.md` 项去重收敛为**一份**含 U1–U12 单一真相源的计划，每条标注来源计划与原始单元编号（证据：`docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` U4、图例与口径、总览表）。
- **执行（execute）**：按模块分批闭环——先 P0（对齐/测试基线），再 P1（生产切换类），后 P2/P3（增强/文档），每批以可验证证据关闭（证据：`docs/plans/2026-08-21-002` 各单元"执行结论"小节）。
- **收口扫描（closeout sweep）**：用 `docs/audits/final-coverage-sweep.md` 做最终静态度量，输出权威覆盖率与剩余缺口清单（证据：`docs/audits/final-coverage-sweep.md`）。

**3. 残差缺口分类与处置（residual-gap taxonomy）**
每类残差用不同处置策略，避免一刀切（证据：`docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` 实现情况更新 §残差、`docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` Requirements）：
- **外部阻塞（external-blocked）**：影子流量灰度验证与切流（U3）——脚本就绪，需生产环境 + ≥2 周观察期，唯一真正阻断"可替代"判定宣告的项。
- **微小代码缺口（tiny code gaps）**：3 条真实缺失端点（processplatform_assemble_surface 2 条、bbs_assemble_control 1 条），逐条仿现有 handler 补齐（证据：`docs/audits/final-coverage-sweep.md` §四）。
- **框架平台限制（framework platform-limit）**：4 条 axum 单段多参数 `{}.{}` 路由（attachment/download/*/{}.{}），axum 无法表达，留档为已知接受缺口、不实现（证据：`docs/audits/final-coverage-sweep.md` 附录）。
- **大型潜在缺口（large latent gap）**：BAM 业务活动监控模块（Java `x_processplatform_assemble_bam` 131 @Path vs Rust 5 路由），决策补齐闭合（证据：`docs/brainstorms/2026-08-25` R4/Key Decisions）。
- **设计性范围外（out-of-scope-by-design）**：IM/XMPP/WebRTC 完整即时通讯协议，沿用 2026-08-19-002 范围约束排除 v1（证据：`docs/plans/2026-08-21-002` Scope Boundaries；brainstorm Key Decisions）。
- **人工/独立工程（human/standalone engineering）**：Linux 文档翻译校正 266+ 处、7 语种 `?` 损坏，需逐文件对照 kernel.org 上游 RST，作为独立轨道不与替代判定耦合（证据：`docs/plans/2026-08-21-002` U11、L11.1/L11.2）。

**4. 关键决策及其理由（key decisions）**
- **Value::Null 字面归零指标作废（VOIDED）**：残留 15 处经逐点核实 = 测试断言×7 + 文档注释/Option 序列化 helper×5 + 可选 content 列解析（DB NULL→JSON null）×3，无一处为静默空数据桩；继续压低计数会破坏测试与 API 契约，故实质目标达成后作废该代理指标（证据：`docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` U1 执行结论）。
- **SQLx 完全移除被否决（VETOED）**：workspace 直接依赖已清零，但 sea-orm 核心依赖 sqlx，移除底层等于重写 ORM；故仅追求直接依赖清零（证据：`docs/plans/2026-08-21-002` U8）。
- **IM 完整协议排除 v1**：避免范围蔓延，Rust 侧保留 WebSocket 基础广播与 ImAction×33 端点作为部分能力而非完整协议替代（证据：`docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` Key Decisions、R6）。
- **BAM 选择补齐而非挂起**：决策人确认 v1 接管范围包含 BAM 监控，缺口按核心监控端点补齐闭合（证据：`docs/brainstorms/2026-08-25` R4、Key Decisions）。

## Why This Matters

- **把"能否替代"从模糊叙事变为可验证数字**：双分母 + 路径归一化 + 诊断项不计入，使 99.77% 唯一端点覆盖 / 28/30 模块 100% 成为可被决策引用的硬指标，而非"82/83 crate done"式的失真自述（对比：`docs/plans/2026-08-13-002` §0 三条硬阻断证据）。
- **收口战役消除状态漂移**：审计→汇总→执行→扫描四阶段，把散落 18 份文档 + todo.md 的遗留项收敛为单一真相源，杜绝重复审计与基于过期状态的决策（证据：`docs/plans/2026-08-21-001` Summary）。
- **分类处置避免资源错配**：将"外部阻塞 / 微小缺口 / 平台限制 / 大缺口 / 范围外 / 人工工程"分级，使唯一真正阻断项（影子流量）被显式识别，其余作为可书面接受的范围边界，避免无限扩大工程（证据：`docs/brainstorms/2026-08-25` Success Criteria）。

## When to Apply

- 当用一个新实现替代/对齐一个大型既有参考实现（特别是跨语言重写，如 Java→Rust），且现有进度信号（模块数、handler 数、测试通过率）失真或不可信时。
- 当历史计划文档堆积、status 与实际脱节、遗留工作无统一视图时，采用"审计→汇总→执行→收口扫描"编排。
- 当需要对外宣告"可接管/可替代"且必须提供书面范围边界与已知限制时，用残差分类法产出可接受的判定。
- 注：三态 `status` 生命周期纪律本身已由 `docs/solutions/development-workflow/plan-status-lifecycle.md` 与 `docs/solutions/best-practices/single-source-of-truth-migration-status.md` 覆盖，本文不重复；本方法有赖于那两篇作为底层纪律支撑（Related 见下）。

## Examples

**例 1 — 端点对齐度量（before/after）**
- Before：用 `cargo test` 通过率与 handler 计数自称"99% 完成"，但 `Value::Null` 实有 201 处、CMS 311 handler 仅 3.5% 查库（证据：`docs/plans/2026-08-13-002` §0 第 2–3 条）。
- After：路径参数归一化为 `{}` 后交叉比对，终扫得 4573 条 `.route(` 注册（4155 唯一归一化路径），唯一端点口径 3085/3092 = 99.77%，28/30 模块 100%（证据：`docs/audits/final-coverage-sweep.md` §一、第 4 行）。`verb_mismatch`/`literal_shift` 仅诊断不计入，因影子副本会真实 404（§五.1）。

**例 2 — 收口战役编排（concrete usage）**
- 审计阶段产物：`docs/plans/2026-08-21-001` 对 18 份计划逐单元静态验证，补全 2 份无 frontmatter 文档、将 `partially_completed` 归一为 `active`。
- 汇总阶段产物：单一真相源 `docs/plans/2026-08-21-002` 含 U1–U12，每条标注来源（如 U2 来源 `docs/plans/2026-08-20-001` U4）。
- 收口阶段产物：权威终扫 `docs/audits/final-coverage-sweep.md`（generated_at=2026-08-23）。
- 该序列之所以有效：前序的"实现情况"小节是后序汇总的输入，汇总的 U 编号是执行的追踪键，终扫是数字的封板——三段证据链互不重复且可追溯。

**例 3 — 残差分类处置（concrete usage）**
- 微小缺口：`docs/audits/final-coverage-sweep.md` §四列出 3 条真实缺失端点，建议"零星补齐：逐条仿既有 handler + 注册"。
- 平台限制：同文件附录列出 4 条 `attachment/download/{}/work/{}/stream/{}.{}` 等 axum 单段多参数不可表达，留档不实现。
- 外部阻塞：U3 影子流量在 `docs/plans/2026-08-21-002` 实现情况更新中标注为唯一未竟单元（需生产环境 + ≥2 周），脚本 `deploy/shadow-traffic.sh`、`toggle_module.sh` 等已就绪。
- 设计性范围外：IM/XMPP/WebRTC 在 brainstorm `2026-08-25` R6 中明确排除 v1。

**例 4 — 关键决策作废错误指标（concrete usage）**
- `Value::Null` 字面归零标准在 `docs/plans/2026-08-21-002` U1 执行结论中被 VOIDED：残留 15 处经核实均为测试断言/文档注释/可选列解析，非静默空数据桩；继续压低会破坏测试与契约，故判定实质达成。
- `SQLx` 完全移除在 `docs/plans/2026-08-21-002` U8 被 VETOED：sea-orm 依赖 sqlx，移除等于重写 ORM，目标降级为"workspace 直接依赖清零"。

## Related

- `docs/solutions/development-workflow/plan-status-lifecycle.md` — 三态 `status` 生命周期纪律（本战役"审计"支柱的底层支撑，勿重复，交叉引用）
- `docs/solutions/best-practices/single-source-of-truth-migration-status.md` — 单一真相源迁移状态纪律（本战役"汇总"支柱的底层支撑）；注意其示例表为 2026-08-10 时点快照（"81 crate 100% / 0 桩代码"），当前权威状态以 `docs/audits/final-coverage-sweep.md` 的 99.77% + 残差清单为准，该 snapshot 可能需刷新（见下"待办"）
- `docs/solutions/architecture-patterns/strangler-fig-migration-pattern.md` — 无停机渐进迁移架构模式（本 gap-closure 战役是其收尾阶段，解决角度不同，不重复）
- `docs/audits/final-coverage-sweep.md` — 端点对齐终态基准（唯一端点口径 99.77%）
- `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md` — 单一真相源汇总计划（U1–U12）
- `docs/brainstorms/2026-08-25-oa4rust-o2server-residual-gaps-requirements.md` — 残差缺口闭环需求（BAM 已决议补齐）

**待办（已处理，2026-08-25）**：`single-source-of-truth-migration-status.md` 的"100% 完成"示例已刷新——标记示例为 2026-08-10 历史快照，并将 "Current doc" 改指 `docs/audits/final-coverage-sweep.md`（99.77%）作为当前权威来源（见该文 2026-08-25 刷新注记）。
