---
date: 2026-08-25
topic: oa4rust-o2server-residual-gaps
---

# OA4Rust 仍不能完全替代 o2server 的残差闭环需求

> 2026-08-25 初次成稿；同日本轮扩充：将 4 条原「axum 平台限制」端点升级为可补齐（R5），并新增生成器脚本纳管（R8）、handler 行为语义一致性（R9）、模块卡片深度填充（R10），补充 BAM 分阶段验收口径。

## Summary

将截至 2026-08-25 已查明的、oa4rust 仍不能完全替代 o2server 的全部残差收敛为一份可执行的需求文档：完成生产级影子流量验证与切流（唯一真正阻断"可替代"判定的项），补齐零星缺失端点、原判为平台限制的 attachment 端点、BAM 业务活动监控模块缺口，并将生成器脚本纳管、handler 行为语义一致性、模块卡片深度填充等可独立执行的工作纳入范围，IM 完整协议与 Linux 文档校正则作为书面接受的替代范围边界或独立轨道。

---

## Problem Frame

2026-08-25 合并 HEAD（950a18e1）时，Java-Rust 端点对齐度终扫已达 99.77%（3085/3092 唯一端点，28/30 模块 100%），Tantivy 检索、行为对比 CI、Linux 文档精修等历史计划单元均已闭环。但"可接管 o2server"的判定仍不能正式宣布，因为：

- 唯一真正阻断判定的项是**影子流量灰度验证与切流**——脚本与 playbook 早已就绪，但从未在生产环境跑过，需要 ≥2 周观察期（外部阻塞）。
- 存在 3 条零星真实缺失端点、4 条原归类为 axum 平台限制的 `attachment/download/*/{}.{}` 端点（本轮复核确认为可补齐）、1 个 BAM 业务活动监控大缺口（Java 131 @Path vs Rust 5 路由），以及 IM/XMPP/WebRTC 完整协议被既有范围约束排除。
- 此外还有若干"实际可实现但散落未统管"的工作：生成器脚本被 `.gitignore` 忽略导致 OpenAPI 注入静默丢失风险、路由 100% 之外的 handler 深层语义不一致、模块卡片字段未填充。
- 这些残差散落在 `2026-08-21-002` 汇总计划的"实现情况更新"注记与各审计文档中，没有统一的需求视图与验收口径，后续规划要么重复审计、要么基于过期口径决策。

本需求文档是这些残差的单一真相源，使"能否替代、替代到什么程度、哪些明确不替代"可被书面回答并驱动执行。

---

## Actors

- A1. 生产运维 / 部署负责人：执行影子流量灰度与切流，归档比对报告（R1）
- A2. oa4rust 开发者：补齐缺失端点、attachment 端点、BAM 监控缺口、收敛行为语义（R3, R4, R5, R9）
- A3. 技术负责人 / 决策人：裁定"可替代"范围，确认平台限制与协议排除（R2, R6, R7）
- A4. 文档维护者：完成 Linux 文档翻译校正人工工程（R7）、模块卡片深度填充（R10）
- A5. oa4rust 工具链维护者：将生成器脚本纳入版本控制（R8）

---

## Key Flows

- F1. 影子流量验证切流
  - **Trigger：** 生产环境就绪且灰度脚手架就位
  - **Actors：** A1, A3
  - **Steps：** 按 playbook 模块级灰度上线 → 影子流量并行比对 Rust 与 Java 响应 → 观察期 ≥2 周 → 差异报告为空则切流、否则回滚
  - **Outcome：** 比对报告归档，o2server 可下线且核心链路监控无回归
  - **Covered by:** R1, R2

- F2. 端点残差补齐
  - **Trigger：** 计划排期确认
  - **Actors：** A2
  - **Steps：** 提取 3 条缺失端点 + 4 条 attachment 端点的 Java 行为契约 → 仿现有 handler 实现并注册路由 → 纳入行为对比清单
  - **Outcome：** 终扫未覆盖端点数归零（不含明确排除的协议项），行为对比对应端点 PASS
  - **Covered by:** R3, R5

- F3. 可替代范围裁定
  - **Trigger：** 残差清理与验证完成
  - **Actors：** A3
  - **Steps：** 汇总端点对齐度、平台限制、协议排除、BAM 处置 → 输出书面"可替代"判定与替代范围声明
  - **Outcome：** 对外声明明确列出接管范围与已知限制
  - **Covered by:** R2, R6, R7

- F4. handler 行为语义收敛
  - **Trigger：** 路由层 100% 后
  - **Actors：** A2
  - **Steps：** 运行行为对比框架识别 `cms` 等模块的深层语义差异（响应语义/业务一致性） → 逐模块收敛或显式留档
  - **Outcome：** 语义级差异收敛至可接受，替代声明可附带语义一致性结论
  - **Covered by:** R9

- F5. 生成器脚本纳管
  - **Trigger：** 计划排期确认
  - **Actors：** A5
  - **Steps：** 调整 `oa4rust/.gitignore` 取消对 `scripts/` 忽略 → 提交生成器（含 `gen_openapi_paths.py`） → 重新生成校验 OpenAPI 含 `securitySchemes`
  - **Outcome：** 注入式改动纳入版本控制，重建不丢配置
  - **Covered by:** R8

- F6. 模块卡片深度填充
  - **Trigger：** 计划排期确认
  - **Actors：** A4
  - **Steps：** 读取各 crate 源码提取 Key Flows/Dependencies → 填充卡片字段 → 一致性核验
  - **Outcome：** 55+86 张卡片字段非空且与代码一致
  - **Covered by:** R10

---

## Requirements

**生产替代判定（可替代阻断项）**

- R1. 在真实生产环境按既有 playbook 执行模块级灰度与影子流量比对，观察期 ≥2 周且无核心链路差异后完成切流，并归档比对报告。
- R2. 产出"可替代 o2server"的正式判定结论文档，明确接管范围、前提条件与已知限制。

**端点与功能残差**

- R3. 补齐 3 条真实缺失端点（processplatform_assemble_surface 2 条、bbs_assemble_control 1 条），逐条仿现有 handler 实现并注册，纳入行为对比清单。
- R4. 补齐 BAM 业务活动监控模块缺口（Java 131 @Path vs Rust 5 路由）：实现核心监控端点闭合该模块，使 v1 接管范围包含 BAM 监控能力。分阶段验收：核心监控端点优先补齐、低频可后补（具体清单在计划阶段从 o2server 源码与流量重要性评估）。
- R5. 补齐 4 条 `attachment/download/*/{}.{}` 端点（`processplatform_assemble_surface` 共 4 条，见 `docs/audits/final-coverage-sweep.md` 附录）：原归类为 axum「单段多参数不可表达」的端点，实际可用 `Path<String>` 捕获含 `.` 的整段后按最后一个 `.` 拆分为文件名/扩展名（或自定义提取器）实现；段数变体（`stream/{}.{}` 与 `{}.{}`）需分别注册路由。升级为可补齐项，纳入行为对比清单。

**已确认的平台与范围限制**

- R6. 将 IM/XMPP/WebRTC 完整即时通讯协议明确排除在 v1 替代范围外，Rust 侧保留的 WebSocket 基础广播与 ImAction×33 端点作为部分能力而非完整协议替代。

**文档、工具链与行为一致性**

- R7. 完成 Linux 文档翻译校正 L11.1/L11.2（266+ 处、7 语种 `?` 损坏），作为独立人工/脚本工程逐文件对照上游 RST 源执行，或明确作为后续独立轨道不与本替代判定耦合。
- R8. 将 `oa4rust/scripts/` 生成器（含 `gen_openapi_paths.py`）纳入版本控制：调整 `oa4rust/.gitignore` 去除对该目录的忽略，避免 OpenAPI `securitySchemes` 等注入式改动仅存于本地工作副本、他人重新生成时静默丢失（plan002 U8 已识别的运维风险）。
- R9. 收敛 handler 行为语义一致性：在路由注册 100% 之外，对 `cms` 等存在「深层语义不匹配」（响应语义/业务一致性，非路由缺失）的模块，借行为对比框架（`behavior-compare` CI job + `oa4rust/tests/behavior_compare.rs`）持续比对，将残留语义差异收敛至可接受或显式留档。
- R10. 完成模块卡片文档深度填充：填充 `docs/oa/modules/o2server/`（55 张）与 `docs/oa/modules/o2web/`（86 张）卡片的 Key Flows、Dependencies 等字段（`REST Endpoints` 字段已于 62d67e1d 填充），逐卡片与代码一致性核验。

---

## Acceptance Examples

- AE1. **Covers R1.** Given 影子流量比对已运行 ≥2 周且核心链路差异报告为空，when 运维执行切流，o2server 可下线且切流后监控无回归。
- AE2. **Covers R3.** Given 3 条缺失端点已实现并注册，when 运行行为对比，对应端点对比 PASS 且端点覆盖终扫的"真实缺失"数为 0（平台限制项除外）。
- AE3. **Covers R6.** Given 协议排除已书面记录，when 对外发布替代声明，声明中明确列出 IM 完整协议不在接管范围内。
- AE4. **Covers R4.** Given BAM 处置决议已落定（分阶段验收），when 发布替代范围声明，BAM 监控列为已接管能力（核心端点）并标注低频后补部分。
- AE5. **Covers R5.** Given 4 条 `attachment/download` 端点已用 `Path<String>`+拆分 实现并注册，when 运行行为对比，对应端点对比 PASS 且「平台限制」项从终扫排除清单移除。
- AE6. **Covers R8.** Given `.gitignore` 已取消对 `scripts/` 忽略且生成器已提交，when 他人重新生成 OpenAPI，`securitySchemes` 等注入仍保留、不静默丢失。
- AE7. **Covers R9.** Given 行为对比框架已对 `cms` 等模块跑语义比对，when 发布替代声明，可附带「语义一致性已收敛/已留档」结论。
- AE8. **Covers R10.** Given 模块卡片 Key Flows/Dependencies 已填充，when 抽查卡片，字段非空且与代码一致、`docs/oa/README.md` 链接可解析。

---

## Success Criteria

- 人类结果：可明确回答"oa4rust 现在能否替代 o2server"——是（在 PDF 签章、IM 基础能力、CMS/流程/认证等核心 OA 范围内），且对剩余限制有书面、被决策人接受的说明。
- 下游代理：ce-plan 无需发明替代范围或验收口径；本需求文档 + `docs/audits/final-coverage-sweep.md` 可直接驱动执行，不依赖重复审计。
- 范围完整性：原散落未统管的"可实现项"（生成器纳管、语义一致性、卡片填充、attachment 端点）均已纳入，避免后续规划遗漏或重复发现。

---

## Scope Boundaries

- 不含代码层大规模重构（如 cms_assemble_control 单文件拆分，属既有架构债）
- 不含新增 o2server 不存在的功能
- 不含国产库（达梦/金仓）适配验证
- 4 条 `attachment/download/*/{}.{}` 端点已升级为可补齐（R5），不再作为接受缺口；其余真正不可表达项（如有）仍书面记录
- IM/XMPP/WebRTC 完整协议不实现（部分能力保留）
- Linux 文档校正（R7）可作为独立轨道，不阻塞"可替代"判定
- 生成器脚本纳管（R8）、handler 行为语义一致性（R9）、模块卡片深度填充（R10）均在本需求范围内
- 端点对齐度终态以 `docs/audits/final-coverage-sweep.md`（99.77%）为准，不重复扫描

---

## Key Decisions

- 沿用 `2026-08-19-002` 范围约束：IM 完整协议不在 v1 内，避免范围蔓延。
- 残差按"阻断替代判定（U3）/ 可补齐代码缺口（3 端点 + attachment 端点）/ 大缺口待决（BAM）/ 工具链与一致性（R8-R10）/ 人工工程（docs）"分级处置，避免一刀切。
- 最终覆盖口径统一引用 2026-08-24 终扫报告，不重新生成对齐度量。
- 影子流量（U3）是唯一真正阻断"可替代"判定宣告的项，其余均为可书面接受的范围边界或低成本代码缺口。
- BAM 模块选择补齐而非挂起：决策人确认 v1 接管范围包含 BAM 监控，缺口（Java 131 @Path vs Rust 5 路由）按核心监控端点补齐闭合，并采用分阶段验收（核心优先、低频后补）。
- R5 原「axum 单段多参数不可表达」的归类于本轮复核中被推翻：单段内 `name.ext` 可用 `Path<String>` 捕获后拆分实现，段数变体用多条路由覆盖，故升级为可补齐项而非接受缺口。

---

## Dependencies / Assumptions

- 依赖生产环境 + ≥2 周观察窗口（U3 外部阻塞，非代码层可关闭）
- 依赖 `docs/audits/final-coverage-sweep.md` 作为缺口基准（generated_at=2026-08-23）
- 假设 3 条缺失端点补齐成本低（零星 handler，仿现有模式）
- 假设 BAM 监控类低频，分阶段验收不影响核心 OA 替代判定
- 假设 Linux 文档校正与"可替代"判定无功能耦合，可独立排期
- 假设 4 条 attachment 端点的段数变体有限、可用固定路由集合覆盖（计划阶段确认）
- 假设 `oa4rust/.gitignore` 调整 `scripts/` 忽略无副作用（生成器本就应在版本控制内）
- 假设 `cms` 等深层语义差异可被行为对比框架识别并收敛，不需要改路由层
- 假设模块卡片字段可从源码稳定提取，填充工作量与卡片数线性相关

---

## Outstanding Questions

### Resolve Before Planning

（无 —— BAM 处置已决议：补齐并分阶段验收，见 Key Decisions）

### Deferred to Planning

- [Affects R4][Needs research] BAM 131 @Path 中哪些属于"核心监控端点"需优先补齐、哪些低频可后补，计划在计划阶段从 o2server 源码与流量重要性评估。
- [Affects R3][Needs research] 3 条缺失端点的具体 Java 行为契约细节，在计划阶段从 o2server 源码提取。
- [Affects R5][Needs research] 4 条端点的段数变体精确集合与 `Path<String>` 拆分后的参数语义映射，计划阶段确认路由设计。
- [Affects R1][Technical] 切流回滚触发阈值与观察期具体长度（≥2 周为下限，需运维确认）。
- [Affects R7][Technical] Linux 文档校正的逐文件脚本可行性评估（266+ 处跨 7 语种）。
- [Affects R8][Technical] `oa4rust/.gitignore` 中需移除的具体忽略规则与提交范围。
- [Affects R9][Needs research] 哪些模块存在「深层语义不匹配」及其严重度，计划阶段用行为对比报告量化。
- [Affects R10][Technical] 模块卡片字段提取的脚本化可行性（55+86 张卡片）。

---

## Related

- **Campaign retrospective:** `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- **Current parity state:** `docs/audits/final-coverage-sweep.md` (99.77% as of 2026-08-23)
- **Active plan:** `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md`
