---
title: "绿地切流（greenfield cutover）的「功能可替换」判定标准与设计器↔引擎契约闭环"
date: 2026-09-11
last_refreshed: 2026-10-14
category: architecture-patterns
module: oa4rust + oa4rust/frontend（vs o2server/o2web）
problem_type: architecture_decision
component: migration_strategy
severity: high
symptoms:
  - "「能否替代旧系统」无法回答：端点 100% 注册但行为差异无收敛机制，验证锚点缺失"
  - "ProcessDesigner 输出自定义 {config:{nodes,edges}}，Rust 引擎按 O2OA activities 键解析 JSONB——设计器产物引擎无法执行"
  - "设计器前端调用 15 条不闭合（4×404 + 5×405 + 6×静默影子误路由）：GET 列表命中 {id} 宽路由，生产带 DB 即 200 错体"
  - "38 个 172 行 crud-view 空壳冒充设计器/管理器"
root_cause: missing_contract_closure_and_acceptance_criteria
resolution_type: architecture_decision_and_process
tags: [greenfield-cutover, replaceability, designer-engine-contract, activities-schema, route-shadow, acceptance-gate]
applies_when:
  - "评估新系统能否替代旧系统（无历史数据迁移场景）"
  - "设计器类前端与执行引擎的契约对齐"
  - "定义「完全替代」的验收判据"
related_components:
  - oa4rust/backend/crates/processplatform_service_processing/src/lib.rs（activities 解析）
  - oa4rust/backend/tests/designer_route_match.rs
  - oa4rust/e2e/（designer-roundtrip / workflow-runtime）
  - docs/audits/w13-w14-repository-closure.manifest.json
---

# 绿地切流的「功能可替换」判定标准与设计器↔引擎契约闭环

## Problem

「完全替代」三个字包含两个可分离的问题：**能力替代**（新系统能提供等价业务行为）与**数据迁移**（旧数据可搬迁）。当现场没有旧系统历史数据时，影子流量/灰度比对/回滚演练这些经典判据全部失去前提，必须重新定义验证锚点；同时设计器→引擎→运行时的数据契约不闭合会让"结构 100% 覆盖"成为假象。

## Resolution

### 判据重定义：绿地切流 = 能力替代，不是数据迁移

| 经典判据（有旧系统） | 绿地等价物（无旧系统） |
|---|---|
| R1 生产影子流量比对 ≥2 周 | 双侧匹配种子 behavior_compare + fail_baseline 固化门禁（observed > baseline 才失败） |
| 导入历史 .o2 流程可编辑 | 全新建闭环：web 从零创作 → 存 Rust → 引擎执行 → web 渲染 → 审批回传 |
| 业务状态不对称靠真实数据 | 扩充种子使两侧命中同一资源，把"数据缺失假象"转成"可判定的真语义差异" |
| 生产切流 + 回滚剧本 | 无旧系统则判据 = 静态门禁全绿 + 仓内 live E2E + 上线后常态监控（非前置） |

**验收项（缺一不可宣布「功能可替换」）**：
1. 双侧富种子下 behavior_compare FAIL ≤ 固化阈值且 workflow 域有真实种子覆盖（占位 `'{}'` 种子 = workflow 行为未测）；
2. 前端 E2E 闭环（四设计器保存 round-trip 无 404 + 工作流发起→填报→审批→收尾）；
3. 无伪装实装（壳视图逐个"实装 or 书面范围外"，机器可校验 manifest）；
4.（已废除）真实流量金丝雀——无旧系统时此项**正式移除**，不是"降级"；
5. 书面边界声明（IM 完整协议排除 / 无历史迁移 / 参考锚点局限 / 已知偏差留档）。

保留边界：即使 1/2/3/5 全达成，也**不宣布"100% 逐字节完全替代"**——残余行为差异（本仓 136 条 FAIL 多为定时注册类环境性差异）+ SKIP + 书面范围外客观存在。

### 被代码锁死的契约：引擎决定设计器目标格式

Rust 引擎把流程定义存 JSONB 并按 **`activities`** 键解析——这不是设计选择而是存储事实，前端设计器必须产出 O2OA activities JSON（begin/manual/choice/split/merge/embed/delay/invoke/service/agent/cancel 15 类 activity、字段权限、edition、waypoint），与「有没有历史数据要导入」无关。表单同理（`moduleList` 而非自定义 `{schema}`）。**先查引擎/存储的解析点，再定前端契约方向**，避免双向各说各话。

### 设计器↔后端路由对账：三档不闭合分级处置

机器抽取前端真实调用点 × 后端路由表，不闭合分三档：
- **404（后端缺）**：补 handler（如 query `designer_execute` 仅允许单条 SELECT + LIMIT 500）；
- **405（方法不符）**：优先前端改名命中既有（PUT `/update/{id}` → 后端已有 POST `/save/{id}`）；
- **静默影子（最高危）**：`GET process/list` 命中 `process/{id}` 且 id="list"，**带 DB 时返回 200 错体**——收窄宽路由或前端改打字面量路径，逐条消影子。
固化于 `tests/designer_route_match.rs` 纳入 CI 无 DB check 作业，作为死链/影子回归门。

### 无伪装实装：manifest + 守卫测试

壳视图（crud-view 空壳）逐个裁决进 manifest（implemented / out_of_scope / still_blocked + `replacementClaim:false`），`w13-w14-closure.test.ts` 机器一致守卫——禁止以壳冒充分替代，新增壳必须显式声明。

## Why This Matters

「结构覆盖率」与「可替代性」之间隔着三层：行为门禁、端到端契约闭环、诚实声明。本模式把每层变成可机器校验的资产（fail_baseline / route_match / closure manifest / e2e specs），使"能不能替代"成为可以引用证据回答的问题，而不是发布会口号。

## When to Apply

- 重写/替换 legacy 系统且无历史数据可迁移
- 任何「设计器 + 引擎」组合的契约设计
- 定义大型迁移的验收标准

## Related

- 计划：`docs/plans/2026-09-11-001-assess-oa4rust-web-full-replacement-gap-plan.md`（§〇 判据重定义、§三 P0、§四 横切缺口、§六 W3-W14、§七 验收总纲 rev13-rev16）
- 行为门禁：`../testing/behavior-compare-tolerance-and-evidence-discipline.md`
- 机器审计方法：`../development-workflow/machine-audit-extraction-methodology.md`
