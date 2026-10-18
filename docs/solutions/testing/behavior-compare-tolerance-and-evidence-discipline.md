---
title: "行为对比框架：结构容忍的设计边界、证据纪律与会话毒化防护"
date: 2026-08-28
last_refreshed: 2026-10-14
category: testing
module: oa4rust/backend/tests/behavior_comparison
problem_type: testing_methodology
component: behavior_compare
severity: high
symptoms:
  - "836 条 FAIL 中 ~620 条是信封/包装结构差异（data vs prompt 互换、count 包装、root 元数据字段）——真差异被结构性噪音淹没"
  - "allowlist 盲目扩改名对会粉饰真实差异（种子轮教训）"
  - "登出/切换用户族端点销毁对比会话，毒化其后 ~2000 条请求"
  - "CI 只播 Rust 侧种子，FAIL 基线天然虚高，PR 数字无法与本地对齐"
  - "fail_baseline 停在 bootstrap，门禁从未真正启用"
root_cause: structural_noise_without_discipline
resolution_type: methodology_and_ci_design
tags: [behavior-compare, comparator, allowlist, fail-baseline, seed, regression-gate]
applies_when:
  - "维护双系统行为对比框架（Rust vs Java 参考实现）"
  - "设计「结构等价」与「语义等价」的边界"
  - "把一次性对比变成 CI 持续门禁"
related_components:
  - oa4rust/backend/tests/behavior_comparison/fail_baseline（=170）
  - oa4rust/backend/tests/behavior_comparison/seeds/
  - oa4rust/backend/scripts/seed_java.py
  - oa4rust/backend/migrations/091_seed_real_process_definition.sql
---

# 行为对比框架：结构容忍的设计边界、证据纪律与会话毒化防护

## Problem

双系统行为对比的敌人有两个：结构性噪音（信封/包装/root 元数据差异占 FAIL 大头）把真差异淹没；以及对比过程自身的副作用（登出类端点杀掉会话）污染数据。没有证据纪律的容忍规则会把框架变成"粉饰机"。

## Resolution

### 容忍规则：每条带来量化收益、方法冻结

comparator 七项容忍改进及实测收益（1298/738 → 1821/193）：

| 规则 | 收益 | 边界 |
|------|------|------|
| 信封不对称容忍（data vs prompt 互换） | ~306 FAIL | 仅当双侧均含有效载荷时 |
| 空对象包装容忍 `{}` vs 缺字段 | ~22 | 空集等价的前提是"空=无"语义成立 |
| 上传端点跳过 | ~6 | 上传响应是第四层差异（Java {position,count,spent} vs Rust {status,servlet,url}），独立问题不混入 |
| Root 元数据字段抑制（date/spent 等） | ~126 | 元数据非业务字段 |
| Nested data.* 字段抑制 | ~10 | 限定嵌套层 |
| 写入成功等价（返回 id/时间戳不同） | ~3 | 只判"写成功"形状 |
| Number vs Bool 容忍 | ~1 | JSON 弱类型现实 |

**方法论冻结**：比较器规则冻结不变，单项修订需新证据支持（写进计划 Scope Boundaries）。

### 证据纪律：聚类只产候选，入库必须三态评审

- 聚类脚本（`cluster_behavior_diffs.py`）是**只读分析工具**：同端点 `A: missing in Java + B: missing in Rust` 记候选改名对，跨端点聚频次，`type differs` 单列不进改名对；输出 TSV+MD 到 `target/` 不入仓。
- 评审门三态裁决：**采纳改名对 / 判定 Rust 投影错误转 handler 修正 / 驳回留理由**；每条入库必须带证据（端点 + 双侧响应摘录），同名对在 A 端点同义 B 端点非同义 → 不入库记冲突。
- 每批 ≤20 条入库即全量重测；误入库导致 PASS→FAIL 必须回退并在 reason 标注废弃。**禁止无证据批量白名单**——这是种子轮用鲜血换来的制度。

### 会话毒化防护：副作用端点请求前整类 SKIP

登出/切换用户族端点会销毁对比会话（with_tokens 双侧登录），旧实现"请求后计算 SKIP 标志"形同虚设——请求已发出、会话已死、其后 ~2000 条请求全部毒化。修复：在**发请求前**按 `is_session_destructive_endpoint` 整类 SKIP。同类教训：比较器登录改为从 Set-Cookie 提取会话（HttpOnly 化后登录不再返回 body token）。

### 从一次性对比到 CI 门禁

- **双侧全种子基线**：Java 侧经 `scripts/seed_java.py --init --process-form`（init + testadmin + 组织域 + 091 真实 activities/form 流程种子），Rust 侧自动迁移 + `seed_fixtures.sql`；占位 `'{}'` 种子 = workflow 域形同未测，必须替真实流程定义种子。
- **fail_baseline 固化**：观测 157/158 FAIL（两个全新环境 ±1 稳定）→ 基线 170（+8% 环境余量）；门禁语义 = **observed > baseline 才失败**（165 过 / 175 拦实测），允许基线以下波动不算回归。
- CI 行为 job 必须播种子（psql 失败 fail-fast），产物 `behavior-report.md` + 聚类结果上传 artifact；CI 未播 Java 侧 REST 种子的偏差在 job 输出显式标注，防误读。

### 收敛度量闭环

W12 深语义收敛按"门禁观测值"度量：158→155→152→156→153→136（rev9-rev13 逐批），每批 handler 对齐 Java 信封/Wrap* 后**双栈同环境 A/B 实测 gate 下降**才算数，不是单测绿就算数。

## Why This Matters

行为对比框架的寿命取决于"信噪比纪律"：容忍规则没有证据就变成粉饰，allowlist 没有评审就变成垃圾桶，基线没有双侧播种就不可比。三条纪律（量化收益、三态评审、双栈实测）让 FAIL 数成为可汇报、可追责的工程指标。

## When to Apply

- 双实现/双版本 API 的行为对账
- 设计任何"结构等价"比较器
- 把本地对比工具升级为 CI 门禁

## Related

- 计划：`2026-08-28-005` / `2026-08-28-006-execution-progress.md`（七项容忍+实测数字）、`2026-08-26-001`（U2-U3 聚类与评审门）、`2026-09-11-001` rev9（S2 固化与毒化修复）
- 绿地切流判据：`../architecture-patterns/greenfield-cutover-replaceability-criteria.md`
- 权威基线纪律：`../best-practices/single-source-of-truth-migration-status.md`
