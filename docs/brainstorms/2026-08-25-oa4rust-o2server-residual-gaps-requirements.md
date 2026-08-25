---
date: 2026-08-25
topic: oa4rust-o2server-residual-gaps
---

# OA4Rust 仍不能完全替代 o2server 的残差闭环需求

## Summary

将截至 2026-08-25 已查明的、oa4rust 仍不能完全替代 o2server 的全部残差收敛为一份可执行的需求文档：完成生产级影子流量验证与切流（唯一真正阻断"可替代"判定的项），补齐零星缺失端点、BAM 业务活动监控模块缺口与明确 axum 平台限制及 IM 完整协议作为书面接受的替代范围边界。

---

## Problem Frame

2026-08-25 合并 HEAD（950a18e1）时，Java-Rust 端点对齐度终扫已达 99.77%（3085/3092 唯一端点，28/30 模块 100%），Tantivy 检索、行为对比 CI、Linux 文档精修等历史计划单元均已闭环。但"可接管 o2server"的判定仍不能正式宣布，因为：

- 唯一真正阻断判定的项是**影子流量灰度验证与切流**——脚本与 playbook 早已就绪，但从未在生产环境跑过，需要 ≥2 周观察期（外部阻塞）。
- 存在 3 条零星真实缺失端点、4 条 axum 框架不可表达的平台限制端点、1 个 BAM 业务活动监控大缺口（Java 131 @Path vs Rust 5 路由），以及 IM/XMPP/WebRTC 完整协议被既有范围约束排除。
- 这些残差散落在 `2026-08-21-002` 汇总计划的"实现情况更新"注记与各审计文档中，没有统一的需求视图与验收口径，后续规划要么重复审计、要么基于过期口径决策。

本需求文档是这些残差的单一真相源，使"能否替代、替代到什么程度、哪些明确不替代"可被书面回答并驱动执行。

---

## Actors

- A1. 生产运维 / 部署负责人：执行影子流量灰度与切流，归档比对报告（R1）
- A2. oa4rust 开发者：补齐缺失端点、处置 BAM 监控缺口（R3, R4）
- A3. 技术负责人 / 决策人：裁定"可替代"范围，确认平台限制与协议排除（R2, R5, R6）
- A4. 文档维护者：完成 Linux 文档翻译校正人工工程（R7）

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
  - **Steps：** 提取 3 条缺失端点的 Java 行为契约 → 仿现有 handler 实现并注册路由 → 纳入行为对比清单
  - **Outcome：** 终扫未覆盖端点数归零（平台限制项除外），行为对比对应端点 PASS
  - **Covered by:** R3

- F3. 可替代范围裁定
  - **Trigger：** 残差清理与验证完成
  - **Actors：** A3
  - **Steps：** 汇总端点对齐度、平台限制、协议排除、BAM 处置 → 输出书面"可替代"判定与替代范围声明
  - **Outcome：** 对外声明明确列出接管范围与已知限制
  - **Covered by:** R2, R5, R6

---

## Requirements

**生产替代判定（可替代阻断项）**

- R1. 在真实生产环境按既有 playbook 执行模块级灰度与影子流量比对，观察期 ≥2 周且无核心链路差异后完成切流，并归档比对报告。
- R2. 产出"可替代 o2server"的正式判定结论文档，明确接管范围、前提条件与已知限制。

**端点与功能残差**

- R3. 补齐 3 条真实缺失端点（processplatform_assemble_surface 2 条、bbs_assemble_control 1 条），逐条仿现有 handler 实现并注册，纳入行为对比清单。
- R4. 补齐 BAM 业务活动监控模块缺口（Java 131 @Path vs Rust 5 路由）：实现核心监控端点闭合该模块，使 v1 接管范围包含 BAM 监控能力。

**已确认的平台与范围限制**

- R5. 将 4 条 axum 单段多参数路由限制端点（attachment/download/*/{}.{}）记录为 oa4rust 的已知平台限制与接受缺口，不在 v1 实现。
- R6. 将 IM/XMPP/WebRTC 完整即时通讯协议明确排除在 v1 替代范围外，Rust 侧保留的 WebSocket 基础广播与 ImAction×33 端点作为部分能力而非完整协议替代。

**文档与人工工程**

- R7. 完成 Linux 文档翻译校正 L11.1/L11.2（266+ 处、7 语种 `?` 损坏），作为独立人工/脚本工程逐文件对照上游 RST 源执行，或明确作为后续独立轨道不与本替代判定耦合。

---

## Acceptance Examples

- AE1. **Covers R1.** Given 影子流量比对已运行 ≥2 周且核心链路差异报告为空，when 运维执行切流，o2server 可下线且切流后监控无回归。
- AE2. **Covers R3.** Given 3 条缺失端点已实现并注册，when 运行行为对比，对应端点对比 PASS 且端点覆盖终扫的"真实缺失"数为 0（平台限制项除外）。
- AE3. **Covers R5, R6.** Given 平台限制与协议排除已书面记录，when 对外发布替代声明，声明中明确列出 attachment 4 条端点限制与 IM 完整协议不在接管范围内。
- AE4. **Covers R4.** Given BAM 处置决议已落定，when 发布替代范围声明，BAM 监控要么列为已接管能力、要么显式标注为 v1 外不接管。

---

## Success Criteria

- 人类结果：可明确回答"oa4rust 现在能否替代 o2server"——是（在 PDF 签章、IM 基础能力、CMS/流程/认证等核心 OA 范围内），且对剩余限制有书面、被决策人接受的说明。
- 下游代理：ce-plan 无需发明替代范围或验收口径；本需求文档 + `docs/audits/final-coverage-sweep.md` 可直接驱动执行，不依赖重复审计。

---

## Scope Boundaries

- 不含代码层大规模重构（如 cms_assemble_control 单文件拆分，属既有架构债）
- 不含新增 o2server 不存在的功能
- 不含国产库（达梦/金仓）适配验证
- axum 单段多参数路由限制端点不实现，仅作接受缺口记录
- IM/XMPP/WebRTC 完整协议不实现（部分能力保留）
- Linux 文档校正（R7）可作为独立轨道，不阻塞"可替代"判定
- 端点对齐度终态以 `docs/audits/final-coverage-sweep.md`（99.77%）为准，不重复扫描

---

## Key Decisions

- 沿用 `2026-08-19-002` 范围约束：IM 完整协议不在 v1 内，避免范围蔓延。
- 残差按"阻断替代判定（U3）/ 可补齐代码缺口（3 端点）/ 平台限制（axum）/ 大缺口待决（BAM）/ 人工工程（docs）"分级处置，避免一刀切。
- 最终覆盖口径统一引用 2026-08-24 终扫报告，不重新生成对齐度量。
- 影子流量（U3）是唯一真正阻断"可替代"判定宣告的项，其余均为可书面接受的范围边界或低成本代码缺口。
- BAM 模块选择补齐而非挂起：决策人确认 v1 接管范围包含 BAM 监控，缺口（Java 131 @Path vs Rust 5 路由）按核心监控端点补齐闭合。

---

## Dependencies / Assumptions

- 依赖生产环境 + ≥2 周观察窗口（U3 外部阻塞，非代码层可关闭）
- 依赖 `docs/audits/final-coverage-sweep.md` 作为缺口基准（generated_at=2026-08-23）
- 假设 3 条缺失端点补齐成本低（零星 handler，仿现有模式）
- 假设 BAM 监控类低频，挂起不影响核心 OA 替代判定
- 假设 Linux 文档校正与"可替代"判定无功能耦合，可独立排期

---

## Outstanding Questions

### Resolve Before Planning

（无 —— BAM 处置已决议：补齐，见 Key Decisions）

### Deferred to Planning

- [Affects R4][Needs research] BAM 131 @Path 中哪些属于"核心监控端点"需优先补齐、哪些低频可后补，计划在计划阶段从 o2server 源码与流量重要性评估。
- [Affects R3][Needs research] 3 条缺失端点的具体 Java 行为契约细节，在计划阶段从 o2server 源码提取。
- [Affects R1][Technical] 切流回滚触发阈值与观察期具体长度（≥2 周为下限，需运维确认）。
- [Affects R7][Technical] Linux 文档校正的逐文件脚本可行性评估（266+ 处跨 7 语种）。

---

## Related

- **Campaign retrospective:** `docs/solutions/best-practices/oa4rust-o2server-parity-closure-campaign-2026-08-25.md`
- **Current parity state:** `docs/audits/final-coverage-sweep.md` (99.77% as of 2026-08-23)
- **Active plan:** `docs/plans/2026-08-21-002-feat-remaining-work-consolidation-plan.md`
